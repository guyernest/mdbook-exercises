# Organizing and Integrating Exercises in mdBook

This guide describes a clean, reusable way to author and integrate interactive exercises in mdBook using `mdbook-exercises`. It focuses on folder structure, authoring conventions, and the most maintainable integration pattern for course developers.


## Recommended Approach (Include-Only)

Keep exercise content in a dedicated `exercises/` folder and include it from thin chapter pages. This mirrors common quiz layouts and avoids duplication.

- Benefits:
  - No double-rendering (raw + UI).
  - Easy reuse across chapters/books/tools.
  - Cleaner diffs and simpler maintenance.


## Folder Layout

Under your book source (e.g., `src/`), add an `exercises/` folder organized by chapter/topic:

- `src/exercises/ch02/hello-mcp.md`
- `src/exercises/ch02/calculator.md`
- `src/exercises/ch02/code-review.md`

Keep chapter pages as thin wrappers under your existing structure (for links and context):

- `src/part1-foundations/ch02-ex01-hello-mcp.md`
- `src/part1-foundations/ch02-ex02-calculator.md`
- `src/part1-foundations/ch02-ex03-code-review.md`

Your chapter-level index page (e.g., `src/part1-foundations/ch02-exercises.md`) can continue to link to each exercise page.


## Authoring an Exercise File

An exercise file in `exercises/` contains only directive blocks — no front matter or top-level heading. Typical sections:

- `::: exercise` … `:::`
- `::: objectives` … `:::`
- `::: discussion` … `:::`
- `::: starter file="..." language=...` … `:::`
- `::: hint level=...` … `:::` (one or more)
- `::: solution [reveal=on-demand|always|never]` … `:::`
- `::: tests mode=playground|local [language=...]` … `:::`
- `::: reflection` … `:::`

Example skeleton:

````markdown
::: exercise
id: ch02-01-hello-mcp
difficulty: beginner
time: 20 minutes
:::

::: objectives
thinking:
  - Key conceptual goal
doing:
  - Key hands-on goal
:::

::: starter file="src/main.rs" language=rust
```rust
// starter code
```
:::

::: hint level=1
First hint.
:::

::: solution reveal=on-demand
```rust
// solution code
```
:::

::: tests mode=playground
```rust
#[test]
fn test_example() { assert!(true); }
```
:::

::: reflection
- What did you learn?
:::
````


## Including an Exercise in a Chapter Page

Use a thin chapter page that includes a single exercise file. Optional title/intro is fine, but do not paste the `:::` blocks if you include the exercise — pick one style only.

````markdown
# Exercise: Your First MCP Server (intro optional)

{{#exercise ../exercises/ch02/hello-mcp.md}}
````

- Do: one `{{#exercise ...}}` include per page.
- Don’t: mix the include with the raw `:::` blocks on the same page.


## book.toml Configuration

Ensure the preprocessor runs and assets are available:

```toml
[preprocessor.exercises]
# Enable or disable processing
enabled = true

# Copy CSS/JS into src/theme/
manage_assets = true

# Optional defaults
reveal_hints = false
reveal_solution = false
playground = true
progress_tracking = true

[output.html]
# Load installed assets from your theme directory
additional-css = ["theme/exercises.css"]
additional-js  = ["theme/exercises.js"]
```

Build logs should show:

```
[INFO] (mdbook-exercises): Running the mdbook-exercises preprocessor (v0.1.3)
```

If assets are missing and `manage_assets = false`, you’ll see a hint telling you exactly how to add them to `additional-css`/`additional-js`.


## Preprocessor Ordering

If you use other preprocessors that also utilize `:::` blocks (e.g., admonitions/quizzes), ensure the ordering in your build environment does not cause transformations that confuse exercise region replacement. The simplest approach is to:

- Include-only pages for exercises (as above).
- Keep one integration style per page (avoid mixing inline + include).
- Clean builds (`mdbook clean`) when changing preprocessor versions.


## Troubleshooting Duplication

- Symptom: both the raw `:::` directives and a rendered exercise appear.
- Common causes:
  - Mixing include and inline `:::` blocks in the same page.
  - Stale output: run `mdbook clean && mdbook build`.
  - Preprocessor not running or an older binary on CI (check the INFO log and version).

With `mdbook-exercises` v0.1.3+, the preprocessor replaces the full directive region when using inline style, and the include-only pattern eliminates duplication entirely.

### Linting: Empty Tests Blocks

To help authors catch accidental empty `::: tests` blocks (which are ignored and don’t render), you can add a CI step that fails when found. This repository includes an example linter:

```bash
cargo run --example lint_empty_tests -- path/to/your/book/src
```

It scans Markdown files under the given path and exits with a non‑zero status if it finds a `::: tests` block with no code content.

### CI Integration Example (GitHub Actions)

Add a job step before your `mdbook build` to catch empty tests early:

```yaml
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - name: Checkout
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Run mdbook-exercises linter (empty tests)
        run: |
          # Adjust the path to your mdBook source directory
          cargo run --example lint_empty_tests -- pmcp-course/src

      - name: Build book
        run: |
          mdbook build pmcp-course
```

This fails the workflow if any empty `::: tests` blocks are present.


## Conventions and Tips

- IDs: Use stable, URL-safe identifiers (e.g., `ch02-01-hello-mcp`).
- Starter: Keep `file="..."` and `language=...` accurate for a better editor experience.
- Solution visibility: Use `reveal=on-demand|always|never` to control UI behavior.
- Tests:
  - `mode=playground`: uses the Rust Playground (std only, rate-limited).
  - `mode=local`: show instructions to run locally (e.g., `cargo test`, `python -m unittest`, `node`).
- Accessibility: Screen readers announce test results; Ctrl/Cmd+Enter runs tests when focused inside an exercise.


## Example Repos and Files

- See `sample-book/` in this repository for a minimal mdBook using the include syntax.
- See `examples/` for multi-language exercises (Python, JavaScript) and solution reveal policy demos.


## Summary

- Author exercises in `src/exercises/` using directive blocks only.
- Include them from thin chapter pages via `{{#exercise ...}}`.
- Ensure the preprocessor and assets are configured in `book.toml`.
- Avoid mixing include and inline `:::` blocks in the same page to prevent duplication.
