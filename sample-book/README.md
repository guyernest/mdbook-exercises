# Sample mdBook Using mdbook-exercises (and mdbook-quiz)

This is a minimal mdBook showing how to use mdbook-exercises alongside mdbook-quiz.

## Setup

1. Install mdBook and preprocessors:

```bash
cargo install mdbook
cargo install mdbook-exercises
# Optional (if using mdbook-quiz)
cargo install mdbook-quiz
```

2. Build:

```bash
mdbook build
```

You should see a log like:

```
[INFO] (mdbook-exercises): Running the mdbook-exercises preprocessor (vX.Y.Z)
```

Open `book/index.html` to view.

## Notes

- This sample uses `manage_assets = true`, which installs `exercises.css/js` into `src/theme/` automatically.
- The `exercises.md` page demonstrates including exercises via `{{#exercise ...}}` from the repository’s `examples/` folder. Adjust paths if you move this sample.

