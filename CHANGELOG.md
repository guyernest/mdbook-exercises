# Changelog

All notable changes to this project will be documented in this file.

## [0.1.4] - 2025-12-28

### Features
- Setup exercises: Added `docs/setup-exercises.md` (ID suffix `-setup`, recommended position `00`) and example `examples/ch02-environment-setup.md`, plus `sample-book` “Setup” page.
- Include-only authoring guide: Added `docs/exercises-integration.md` (folder structure, include pattern, config, troubleshooting).
- New examples: multilang Python and JavaScript, solution reveal demo, and a two-exercise chapter example; `sample-book` updated to include examples via `{{#exercise ...}}`.

### Preprocessor/UI
- Robust solution reveal: Ensures content is visible when `<details>` is open; improved toggle to switch text between “Show Solution” / “Hide Solution”.
- Conditional tests rendering: Empty tests blocks are ignored; no “Tests” section appears.
- Empty tests warning: Emits `[WARN]` when a tests block has no code content.
- Inline region replacement: Replaces only the exercise directive region, preserving surrounding chapter content.
- Startup log + helpful hints: Logs version and provides asset setup hints (`manage_assets` or manual `additional-css/js`).

### Config and ergonomics
- New flags: `[preprocessor.exercises] enabled` (default true) and `manage_assets` (optional; installs theme assets).
- README improvements: “Examples and Live Demo” section moved higher; links to developer guides and `sample-book`.
- Linter example: `examples/lint_empty_tests.rs` to catch empty tests blocks in CI; docs include a GitHub Actions snippet.

### Compatibility
- Recommended mdBook `v0.4.52+` (version mismatch is a non-fatal warning).
- No breaking changes to directive syntax.

### Upgrade notes
- Ensure `book.toml` enables the preprocessor and loads assets:
  - `[preprocessor.exercises] enabled = true`, `manage_assets = true`
  - `[output.html]` `additional-css = ["theme/exercises.css"]`, `additional-js  = ["theme/exercises.js"]`
- Prefer include-only pages to avoid duplication:
  - `{{#exercise ../exercises/chXX/your-exercise.md}}`
