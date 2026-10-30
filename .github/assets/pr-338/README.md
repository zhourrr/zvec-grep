# PR #338 screenshots

These images illustrate https://github.com/zvec-ai/zvec-grep/pull/338 at CLI revision `509c7f88bb2305522c5c9431d6c9367496865180`.

The packaged release CLI was run on macOS ARM64 in a real 112-column PTY with `TERM=xterm-256color`. Original ANSI output was replayed in xterm.js 5.5.0 and captured with the browser screenshot tool. These are browser screenshots of a terminal replay, not native Terminal application screenshots. Titles, command captions, and provenance notes are outside the terminal. Display height and screenshot bounds were fitted to the output; the CLI text and ANSI stream were not edited.

The demo workspace contained 1,202 generated files. Indexing used the locally cached `local/potion-code-16m-v2` model and completed successfully. Optimization and saving images are actual intermediate frames from that same run. Static images demonstrate labels and elapsed-time presentation, not animation or performance guarantees. The model picker was cancelled after recording its frame (exit 130). The incompatible-index scenario used an isolated manifest fixture with `indexVersion: 1`.

The image branch is separate from the code PR. Images are linked from the PR using this branch's immutable commit SHA.

| Screenshot | Executed command | JPEG SHA-256 |
| --- | --- | --- |
| `incompatible` | `zg --fts token --mode direct` | `5ab93efc16273c6c57fcee4197387bf2055b2e646169974937d8b91391a5efa3` |
| `index-complete` | `zg --index --embedding local/potion-code-16m-v2 --mode direct` | `c3f22cb4ca9060803fe59f8b390158ebc8db38ad86664fdebee667daa647e658` |
| `index-optimizing` | `zg --index --embedding local/potion-code-16m-v2 --mode direct` | `bf04973c03b6cb174fcf965138e4a5d108723a844a07e96245b1e4877b694cf9` |
| `index-saving` | `zg --index --embedding local/potion-code-16m-v2 --mode direct` | `9d96773733f6e8cf9afe2b3ca908ef4860abb41169d3f0bd42a88a17d802965b` |
| `model-picker` | `zg version --mode direct` | `be0d67a70ad3a76cba67974dafe4ff54ba2e32379a49f55a23261d50060e9b73` |
| `parse-error` | `zg --index --build` | `206d308ce48edf1b31145430e6fce52902918d9a0c47c7bfae05d2402d32f2bf` |
| `search-detail` | `zg --fts authenticateUser --preview detail --limit 1 --mode direct --refresh off` | `67ab3137b77f9e328d4bd9c4b20c61e507005df78cd7aea9eda7aac5cabb6035` |
| `search-short` | `zg --fts authenticateUser --preview short --limit 1 --mode direct --refresh off` | `5f31c34eea02cb0f5e7a24bc38a2ae47add1ced7493ae0c1b41e5d123ab0878f` |
