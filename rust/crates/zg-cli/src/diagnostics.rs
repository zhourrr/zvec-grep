use std::fmt::Write as _;

use clap::{
    CommandFactory,
    builder::StyledStr,
    error::{ContextKind, ContextValue, ErrorFormatter, ErrorKind},
};

/// Prints a parse diagnostic with the CLI's label style and clap's output policy.
pub fn exit_parse_error(error: &clap::Error) -> ! {
    presentation_error(error).exit()
}

struct DiagnosticFormatter;

impl ErrorFormatter for DiagnosticFormatter {
    fn format_error(error: &clap::error::Error<Self>) -> StyledStr {
        match error.get(ContextKind::Custom) {
            Some(ContextValue::StyledStr(message)) => message.clone(),
            _ => StyledStr::new(),
        }
    }
}

fn presentation_error(error: &clap::Error) -> clap::error::Error<DiagnosticFormatter> {
    let message = error.render().ansi().to_string();
    let message = if matches!(
        error.kind(),
        ErrorKind::DisplayHelp
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            | ErrorKind::DisplayVersion
    ) {
        message
    } else {
        capitalize_labels(&message)
    };
    let mut presentation = clap::error::Error::new(error.kind()).with_cmd(&crate::Cli::command());
    presentation.insert(ContextKind::Custom, ContextValue::StyledStr(message.into()));
    presentation
}

fn capitalize_labels(message: &str) -> String {
    let mut output = String::with_capacity(message.len());
    for line in message.split_inclusive('\n') {
        let mut start = 0;
        loop {
            let rest = &line[start..];
            if rest.starts_with([' ', '\t']) {
                start += 1;
            } else if let Some(escape) = rest.strip_prefix("\x1b[")
                && let Some(end) = escape.find('m')
            {
                start += 2 + end + 1;
            } else {
                break;
            }
        }
        output.push_str(&line[..start]);
        let rest = &line[start..];
        if ["error:", "warning:", "tip:"]
            .iter()
            .any(|label| rest.starts_with(label))
        {
            output.push(char::from(rest.as_bytes()[0]).to_ascii_uppercase());
            output.push_str(&rest[1..]);
        } else {
            output.push_str(rest);
        }
    }
    output
}

/// Formats engine diagnostics for the CLI without changing the engine error contract.
#[must_use]
pub fn format_error_report(report: &zg_engine::ErrorReport) -> String {
    let mut output = format!("Error[{}]: {}", report.code, report.message);
    if let Some(help) = &report.help {
        let _ = write!(output, "\nHelp: {help}");
    }
    let _ = write!(output, "\nError location: {}", report.origin);
    if let Some(reported_at) = &report.reported_at {
        let _ = write!(output, "\nReporting location: {reported_at}");
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_diagnostic_preserves_styles_payload_usage_and_exit_status() {
        let error = crate::Cli::try_parse_from(["zg", "--index", "--build"])
            .expect_err("misspelled rebuild option");
        let original = error.render().ansi().to_string();
        let presentation = presentation_error(&error);
        let rendered = presentation.render();
        let plain = rendered.to_string();
        assert!(
            plain.starts_with("Error: unexpected argument '--build' found"),
            "{plain}"
        );
        assert!(
            plain.contains("Tip: a similar argument exists: '--rebuild'"),
            "{plain}"
        );
        assert!(
            plain.contains("Usage: zg --index --rebuild [ROOT]"),
            "{plain}"
        );
        assert_eq!(presentation.exit_code(), error.exit_code());
        assert_eq!(presentation.use_stderr(), error.use_stderr());
        assert_eq!(
            rendered.ansi().to_string().matches('\x1b').count(),
            original.matches('\x1b').count()
        );
        let raw = clap::Error::raw(
            ErrorKind::InvalidValue,
            "invalid value 'error: warning: tip:'",
        );
        assert_eq!(
            presentation_error(&raw).to_string(),
            "Error: invalid value 'error: warning: tip:'"
        );
    }

    #[test]
    fn help_output_is_not_rewritten() {
        let error = clap::Command::new("fixture")
            .after_help("tip: user-supplied help text")
            .try_get_matches_from(["fixture", "--help"])
            .expect_err("help requested");
        let presentation = presentation_error(&error);
        assert_eq!(presentation.to_string(), error.to_string());
        assert_eq!(presentation.exit_code(), 0);
        assert!(!presentation.use_stderr());
    }
}
