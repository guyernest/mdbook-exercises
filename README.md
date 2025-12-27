# mdbook-exercises

A preprocessor for [mdBook](https://rust-lang.github.io/mdBook/) that adds interactive exercise blocks with hints, solutions, and optional Rust Playground integration for testing.

## Features

- **Exercise metadata** - Difficulty levels, time estimates, prerequisites
- **Learning objectives** - Structured thinking/doing outcomes
- **Discussion prompts** - Reflection questions before coding
- **Starter code** - Editable code blocks with syntax highlighting
- **Progressive hints** - Collapsible, leveled hints that reveal incrementally
- **Solutions** - Hidden by default, reveal on demand
- **Test integration** - Run tests via Rust Playground or locally
- **Progress tracking** - LocalStorage-based completion tracking
- **Accessible** - Keyboard navigation, screen reader support

## Installation

### From crates.io

```bash
cargo install mdbook-exercises
```

### From source

```bash
git clone https://github.com/YOUR_ORG/mdbook-exercises
cd mdbook-exercises
cargo install --path .
```

## Quick Start

### 1. Add to your book.toml

```toml
[preprocessor.exercises]
```

### 2. Create an exercise in Markdown

```markdown
# Exercise: Hello World

::: exercise
id: hello-world
difficulty: beginner
time: 10 minutes
:::

Write a function that returns a greeting.

::: starter file="src/lib.rs"
```rust
/// Returns a greeting for the given name
pub fn greet(name: &str) -> String {
    // TODO: Return "Hello, {name}!"
    todo!()
}
```
:::

::: hint level=1
Use the `format!` macro to create a formatted string.
:::

::: hint level=2
```rust
format!("Hello, {}!", name)
```
:::

::: solution
```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```
:::

::: tests mode=playground
```rust
#[test]
fn test_greet() {
    assert_eq!(greet("World"), "Hello, World!");
}

#[test]
fn test_greet_name() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}
```
:::
```

### 3. Build your book

```bash
mdbook build
```

## Directive Reference

### Exercise Block

Defines metadata for the exercise:

```markdown
::: exercise
id: unique-exercise-id
difficulty: beginner | intermediate | advanced
time: 20 minutes
prerequisites: [exercise-id-1, exercise-id-2]
:::
```

### Objectives Block

Learning outcomes in two categories:

```markdown
::: objectives
thinking:
  - Understand concept X
  - Recognize pattern Y

doing:
  - Implement function Z
  - Write tests for edge cases
:::
```

### Discussion Block

Pre-exercise reflection prompts:

```markdown
::: discussion
- Why might we want to do X?
- What are the tradeoffs of approach Y?
:::
```

### Starter Block

Editable code for the student to complete:

```markdown
::: starter file="src/main.rs" language=rust
```rust
fn main() {
    // TODO: Your code here
}
```
:::
```

**Attributes:**
- `file` - Suggested filename (displayed in header)
- `language` - Syntax highlighting language (default: rust)

### Hint Block

Progressive hints with levels:

```markdown
::: hint level=1 title="Getting Started"
First, consider...
:::

::: hint level=2
Here's more detail...
:::

::: hint level=3 title="Almost There"
```rust
// Nearly complete solution
```
:::
```

**Attributes:**
- `level` - Hint number (1, 2, 3, etc.)
- `title` - Optional title for the hint

### Solution Block

The complete solution, hidden by default:

```markdown
::: solution
```rust
fn solution() {
    // Complete implementation
}
```

### Explanation

Why this solution works...
:::
```

### Tests Block

Test code that can optionally run in the browser:

```markdown
::: tests mode=playground
```rust
#[test]
fn test_example() {
    assert!(true);
}
```
:::
```

**Attributes:**
- `mode` - Either `playground` (run in browser) or `local` (display only)

When `mode=playground`:
- A "Run Tests" button appears
- User code is combined with test code
- Sent to play.rust-lang.org for execution
- Results displayed inline

### Reflection Block

Post-exercise questions:

```markdown
::: reflection
- What did you learn from this exercise?
- How would you extend this solution?
:::
```

## Browser Features

### Test Execution

When tests have `mode=playground`, the preprocessor generates JavaScript that:

1. Captures the user's code from the editable starter block
2. Combines it with the test code
3. Sends to the Rust Playground API
4. Displays compilation errors or test results

**Limitations:**
- Only works with `std` library (no external crates)
- Subject to playground rate limits
- Requires internet connection
- ~5 second execution timeout

For exercises requiring external crates, use `mode=local` and guide users to run `cargo test` locally.

### Progress Tracking

Exercise completion is tracked in localStorage:

- Checkboxes next to learning objectives
- "Mark Complete" button for exercises
- Progress persists across sessions
- No server required

### Accessibility

- All interactive elements are keyboard-accessible
- Collapsible sections use proper ARIA attributes
- High contrast mode supported
- Screen reader announcements for test results

## Configuration

### book.toml options

```toml
[preprocessor.exercises]
# Show all hints by default (useful for instructor view)
reveal_hints = false

# Show solutions by default
reveal_solutions = false

# Enable playground integration
playground = true

# Custom playground URL (for private instances)
playground_url = "https://play.rust-lang.org"

# Enable progress tracking
progress_tracking = true
```

## Library Usage

`mdbook-exercises` can be used as a library for parsing exercise markdown:

```rust
use mdbook_exercises::{parse_exercise, Exercise};

let markdown = std::fs::read_to_string("exercise.md")?;
let exercise = parse_exercise(&markdown)?;

println!("Exercise: {}", exercise.metadata.id);
println!("Difficulty: {:?}", exercise.metadata.difficulty);
println!("Hints: {}", exercise.hints.len());
```

### Feature Flags

```toml
[dependencies]
# Parser only (no rendering, no mdBook dependency)
mdbook-exercises = { version = "0.1", default-features = false }

# With HTML rendering (no mdBook dependency)
mdbook-exercises = { version = "0.1", default-features = false, features = ["render"] }

# Full mdBook preprocessor (default)
mdbook-exercises = { version = "0.1" }
```

## Integration with MCP Servers

For AI-assisted learning experiences, exercise files can be paired with `.ai.toml` files containing AI-specific instructions. The parser extracts structured data that MCP servers can use:

```rust
use mdbook_exercises::{parse_exercise, Exercise};

// In your MCP server
let exercise = parse_exercise(&markdown)?;

// Access structured data for AI guidance
let starter_code = &exercise.starter.as_ref().unwrap().code;
let hints: Vec<&str> = exercise.hints.iter().map(|h| h.content.as_str()).collect();
let solution = &exercise.solution.as_ref().unwrap().code;
```

See [DESIGN.md](./DESIGN.md) for details on MCP integration patterns.

## Examples

See the [examples](./examples) directory for complete exercise examples:

- `hello-world.md` - Basic exercise structure
- `calculator.md` - Multi-hint exercise with tests
- `code-review.md` - Non-coding exercise format

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md).

## License

MIT OR Apache-2.0
