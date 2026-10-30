# PR #338 screenshots

These images illustrate https://github.com/zvec-ai/zvec-grep/pull/338 at CLI revision `ed074ff5ee3940cb628da4883c03731ef2e3cd0f`.

The CLI was run on macOS ARM64 in a real 112-column PTY with `TERM=xterm-256color`. Original ANSI output was replayed in xterm.js 5.5.0 and captured with the browser screenshot tool. These are browser screenshots of a terminal replay, not native Terminal application screenshots. Titles, command captions, and provenance notes are outside the terminal. Display height and screenshot bounds were fitted to the output; the CLI text and ANSI stream were not edited.

The demo workspace contained 1,202 generated files. Indexing used the locally cached `local/potion-code-16m-v2` model and completed successfully. Optimization and saving images are actual intermediate frames from that same run. Static images demonstrate labels and elapsed-time presentation, not animation or performance guarantees. The model picker was cancelled after recording its frame (exit 130). The incompatible-index scenario used an isolated manifest fixture with `indexVersion: 1`.

The image branch is separate from the code PR. Images are linked from the PR using this branch's immutable commit SHA.

| Screenshot | Executed command | JPEG SHA-256 |
| --- | --- | --- |
| `model-picker` | `zg version --mode direct` | `a65a426dea649876d47a4be18996cc472e9f9525c9977833ea7d40bbcb689f2b` |
| `search-short` | `zg --fts authenticateUser --preview short --limit 1 --mode direct --refresh off` | `19244e38b7ce4a3f3a22b39e6cf5d4ecb6dc535ab33c68e50b95c30155eefe15` |
| `search-detail` | `zg --fts authenticateUser --preview detail --limit 1 --mode direct --refresh off` | `54150f6be0bd09416b2be3ee01e57cf7c4a19129d5766938ebf95cd15e5bdc0e` |
| `incompatible` | `zg --fts token --mode direct` | `cd9c334bffc059176c8ee9d70aa7fea4aa062e9b51e9d16a7686def93fca9583` |
| `parse-error` | `zg --index --build` | `727c6515efd66d0a56353813bce7cf4688a6f1f3f1d927354656127cfcfde870` |
| `index-optimizing` | `zg --index --embedding local/potion-code-16m-v2 --mode direct` | `02a2f2689b66047dd65ac9e40e3cebf9e640bfe2e0bf65ac815ab45fea928ba9` |
| `index-saving` | `zg --index --embedding local/potion-code-16m-v2 --mode direct` | `20ebc71424298611492a56c2afc4cc301f6c46cfca1e871944a0ec77719eb798` |
| `index-complete` | `zg --index --embedding local/potion-code-16m-v2 --mode direct` | `1c1179573b95bf0120dcd03b4ac8c148cd661c2962366f736edadab2d87c052d` |
