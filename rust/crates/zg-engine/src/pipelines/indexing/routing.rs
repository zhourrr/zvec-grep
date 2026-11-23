use std::collections::{BTreeMap, BTreeSet};

use crate::{
    EngineError,
    api::index::{
        IndexOptions,
        options::{Device, EmbeddingModelSpec},
    },
    domain::{ContentKind, EmbeddingModelInfo, Workspace},
    models::get_embedding_model_catalog_entry,
};

pub(super) struct EmbeddingPlan {
    pub default_reference: String,
    pub overrides: BTreeMap<ContentKind, EmbeddingModelInfo>,
    pub requests: BTreeMap<String, IndexOptions>,
    pub active: BTreeSet<String>,
}

impl EmbeddingPlan {
    pub(super) fn new(
        existing: Option<&Workspace>,
        options: &IndexOptions,
        default_reference: String,
    ) -> Result<Self, EngineError> {
        let previous = existing.and_then(|w| w.index().descriptor());
        let mut overrides = previous
            .map(|d| d.embedding_routes().clone())
            .unwrap_or_default();
        let mut cleared = BTreeSet::new();
        for &kind in &options.clear_embedding_routes {
            validate_kind(kind)?;
            if !cleared.insert(kind) || options.embedding_routes.contains_key(&kind) {
                return Err(EngineError::invalid_argument(format!(
                    "duplicate embedding update for {}",
                    kind.as_str()
                )));
            }
            overrides.remove(&kind);
        }
        let mut specs = BTreeMap::new();
        let mut default_spec = options
            .embedding
            .clone()
            .unwrap_or_else(|| bare_spec(default_reference.clone()));
        if let Some(endpoint) = &options.endpoint {
            default_spec.endpoint = Some(endpoint.clone());
        }
        specs.insert(default_reference.clone(), default_spec);
        for (&kind, spec) in &options.embedding_routes {
            validate_kind(kind)?;
            let model = catalog_info(&spec.reference)?;
            if !model.model().supports_content(kind) {
                return Err(EngineError::unsupported(format!(
                    "model {} cannot embed {}",
                    spec.reference,
                    kind.as_str()
                )));
            }
            if let Some(stored) = specs.get_mut(&spec.reference) {
                merge_spec(stored, spec)?;
            } else {
                specs.insert(spec.reference.clone(), spec.clone());
            }
            overrides.insert(kind, model);
        }
        let references = std::iter::once(default_reference.clone())
            .chain(overrides.values().map(|m| m.model().reference()))
            .collect::<BTreeSet<_>>();
        let mut requests = BTreeMap::new();
        for reference in references {
            let spec = specs
                .remove(&reference)
                .unwrap_or_else(|| bare_spec(reference.clone()));
            if spec.revision.is_some() {
                return Err(EngineError::unsupported(
                    "embedding revision overrides are not supported by the catalog-backed runtime",
                ));
            }
            let mut request = options.clone();
            request.embedding = Some(spec);
            request.embedding_routes.clear();
            request.clear_embedding_routes.clear();
            if reference != default_reference {
                request.api_key = None;
                request.runtime_api_key = None;
                request.endpoint = None;
                if !reference.starts_with("local/") {
                    request.device = None;
                    request.runtime_device = None;
                }
            }
            requests.insert(reference, request);
        }
        let active = ContentKind::INDEXED
            .into_iter()
            .map(|kind| {
                overrides.get(&kind).map_or_else(
                    || default_reference.clone(),
                    |model| model.model().reference(),
                )
            })
            .collect();
        Ok(Self {
            default_reference,
            overrides,
            requests,
            active,
        })
    }
}

fn bare_spec(reference: String) -> EmbeddingModelSpec {
    EmbeddingModelSpec {
        reference,
        revision: None,
        cache_dir: None,
        endpoint: None,
        device: Device::Auto,
    }
}

/// Routes sharing a runtime can specify complementary settings. Reject two
/// explicit values for the same property before acquiring either model.
fn merge_spec(
    stored: &mut EmbeddingModelSpec,
    other: &EmbeddingModelSpec,
) -> Result<(), EngineError> {
    fn merge<T: Clone + Eq>(current: &mut Option<T>, next: Option<&T>) -> bool {
        match (current.as_ref(), next) {
            (Some(first), Some(second)) => first == second,
            (None, Some(value)) => {
                *current = Some(value.clone());
                true
            }
            _ => true,
        }
    }
    let mut device = (stored.device != Device::Auto).then_some(stored.device);
    if !merge(&mut stored.endpoint, other.endpoint.as_ref())
        || !merge(&mut stored.cache_dir, other.cache_dir.as_ref())
        || !merge(&mut stored.revision, other.revision.as_ref())
        || !merge(
            &mut device,
            (other.device != Device::Auto).then_some(&other.device),
        )
    {
        return Err(EngineError::invalid_argument(format!(
            "conflicting settings for embedding model {}",
            stored.reference
        )));
    }
    stored.device = device.unwrap_or(Device::Auto);
    Ok(())
}

fn validate_kind(kind: ContentKind) -> Result<(), EngineError> {
    if !ContentKind::INDEXED.contains(&kind) {
        return Err(EngineError::unsupported(format!(
            "unsupported index content kind: {}",
            kind.as_str()
        )));
    }
    Ok(())
}

pub(super) fn catalog_info(reference: &str) -> Result<EmbeddingModelInfo, EngineError> {
    get_embedding_model_catalog_entry(reference)
        .map(crate::models::EmbeddingCatalogEntry::embedding_info)
        .ok_or_else(|| {
            EngineError::invalid_argument(format!("unknown embedding model: {reference}"))
        })
}

/// Concrete destinations shared by indexing and remote authorization preflight.
pub(crate) fn embedding_requests(
    existing: Option<&Workspace>,
    options: &IndexOptions,
    default_reference: String,
) -> Result<BTreeMap<String, IndexOptions>, EngineError> {
    let mut plan = EmbeddingPlan::new(existing, options, default_reference)?;
    plan.requests
        .retain(|reference, _| plan.active.contains(reference));
    Ok(plan.requests)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_remote_runtime_merges_settings_and_rejects_conflicts() {
        let reference = "qwen/text-embedding-v4".to_owned();
        let code = bare_spec(reference.clone());
        let mut options = IndexOptions {
            embedding: Some(bare_spec(reference.clone())),
            endpoint: Some("https://one.test/embeddings".into()),
            api_key: Some("default-key".into()),
            embedding_routes: BTreeMap::from([(ContentKind::Code, code)]),
            ..Default::default()
        };
        let requests =
            embedding_requests(None, &options, reference.clone()).expect("shared runtime");
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[&reference].endpoint, options.endpoint);
        options
            .embedding_routes
            .get_mut(&ContentKind::Code)
            .expect("code override")
            .endpoint = Some("https://two.test/embeddings".into());
        assert!(
            embedding_requests(None, &options, reference)
                .expect_err("conflicting destinations")
                .message()
                .contains("conflicting")
        );
    }

    #[test]
    fn override_credentials_and_destinations_are_scoped_to_the_model() {
        let default = "qwen/text-embedding-v4".to_owned();
        let other = "qwen/qwen3-vl-embedding".to_owned();
        let options = IndexOptions {
            embedding: Some(bare_spec(default.clone())),
            endpoint: Some("https://default.test/embeddings".into()),
            api_key: Some("saved-key".into()),
            runtime_api_key: Some("runtime-key".into()),
            embedding_routes: BTreeMap::from([(ContentKind::Code, bare_spec(other.clone()))]),
            ..Default::default()
        };
        let requests = embedding_requests(None, &options, default).expect("distinct models");
        let request = &requests[&other];
        assert!(
            request.api_key.is_none()
                && request.runtime_api_key.is_none()
                && request.endpoint.is_none()
        );
    }

    #[test]
    fn duplicate_updates_and_unindexed_kinds_fail_early() {
        let default = "local/bekko-256-int8".to_owned();
        for clear in [
            vec![ContentKind::Code],
            vec![ContentKind::Text, ContentKind::Text],
            vec![ContentKind::Image],
        ] {
            let options = IndexOptions {
                embedding_routes: BTreeMap::from([(ContentKind::Code, bare_spec(default.clone()))]),
                clear_embedding_routes: clear,
                ..Default::default()
            };
            assert!(embedding_requests(None, &options, default.clone()).is_err());
        }
    }
}
