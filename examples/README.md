# Example Exercises

This directory contains example exercises that demonstrate the mdbook-exercises syntax and features.

## Live Demo

View the rendered examples online: **[guyernest.github.io/mdbook-exercises](https://guyernest.github.io/mdbook-exercises/)**

## Available Examples

| Example | Difficulty | Description |
|---------|------------|-------------|
| [hello-world.md](./hello-world.md) | Beginner | Introduction to Rust functions and the `format!` macro |
| [calculator.md](./calculator.md) | Intermediate | Error handling with `Result<T, E>` and pattern matching |

## Building Examples Locally

### Render a Single Exercise

Use the `render_example` binary to convert an exercise markdown file to HTML:

```bash
cargo run --example render_example -- examples/hello-world.md
```

This generates `hello-world.html` in the current directory. Open it in a browser to view the rendered exercise.

### Render All Examples

```bash
for file in examples/*.md; do
  cargo run --example render_example -- "$file"
done
```

## Creating Your Own Exercises

### Basic Structure

Every exercise starts with metadata and a description, followed by optional sections:

````markdown
# Exercise: Your Title Here

::: exercise
id: unique-id
difficulty: beginner | intermediate | advanced
time: 15 minutes
:::

Your exercise description goes here. Explain what the student will learn.

::: objectives
thinking:
  - Concept they should understand
doing:
  - Task they should complete
:::

::: starter file="src/lib.rs"
```rust
// Starter code for the student to complete
pub fn your_function() {
    todo!()
}
```
:::

::: hint level=1 title="First Hint"
A gentle nudge in the right direction.
:::

::: hint level=2
More specific guidance with code examples.
:::

::: solution
```rust
pub fn your_function() {
    // Complete solution
}
```

### Explanation
Why this solution works...
:::

::: tests mode=playground
```rust
#[test]
fn test_your_function() {
    assert!(your_function());
}
```
:::

::: reflection
- What did you learn?
- How could you extend this?
:::
````

### Exercise Sections Reference

| Section | Required | Purpose |
|---------|----------|---------|
| `exercise` | Yes | Metadata (id, difficulty, time, prerequisites) |
| `objectives` | No | Learning outcomes (thinking/doing) |
| `discussion` | No | Pre-exercise reflection questions |
| `starter` | No | Editable code block for the student |
| `hint` | No | Progressive hints (use level=1, 2, 3...) |
| `solution` | No | Complete solution (hidden by default) |
| `tests` | No | Test code (mode=playground or mode=local) |
| `reflection` | No | Post-exercise questions |

### Tips for Writing Good Exercises

1. **Start simple** - Begin with clear, achievable goals
2. **Progressive hints** - Start vague, get more specific with each level
3. **Explain the "why"** - Solutions should include explanations, not just code
4. **Test edge cases** - Include tests that verify correct handling of edge cases
5. **Encourage reflection** - Ask questions that deepen understanding

### Using with mdBook

To use exercises in an mdBook project:

1. Install the preprocessor:
   ```bash
   cargo install mdbook-exercises
   ```

2. Add to `book.toml`:
   ```toml
   [preprocessor.exercises]
   ```

3. Copy assets to your theme:
   ```bash
   mkdir -p src/theme
   cp /path/to/mdbook-exercises/assets/*.css src/theme/
   cp /path/to/mdbook-exercises/assets/*.js src/theme/
   ```

4. Include assets in `book.toml`:
   ```toml
   [output.html]
   additional-css = ["theme/exercises.css"]
   additional-js = ["theme/exercises.js"]
   ```

5. Write exercises in your markdown files and build:
   ```bash
   mdbook build
   ```

## Contributing Examples

We welcome new example exercises! When contributing:

1. Follow the naming convention: `topic-name.md`
2. Include all standard sections (objectives, hints, solution, tests)
3. Test that the exercise renders correctly
4. Ensure tests pass when the solution is used
