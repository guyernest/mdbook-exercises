# mdbook-exercises Directive Syntax Specification

This document defines the syntax for exercise directives in Markdown files.

## Overview

Exercise blocks use a **fenced directive** syntax:

```
::: directive-name [attributes]
[content]
:::
```

This syntax:
- Renders as text in standard Markdown viewers (graceful degradation)
- Is processed by mdbook-exercises into interactive HTML
- Is inspired by [CommonMark Generic Directives](https://talk.commonmark.org/t/generic-directives-plugins-syntax/444)

## General Rules

### Opening a Directive

```
::: directive-name
```

or with inline attributes:

```
::: directive-name attr1=value1 attr2="value with spaces"
```

or with a mix:

```
::: directive-name attr1=value1
yaml-key: yaml-value
another-key: another-value
```

### Closing a Directive

```
:::
```

The closing `:::` must be on its own line.

### Nesting

Directives **cannot** be nested. A directive must be closed before another can begin.

```markdown
<!-- INVALID -->
::: exercise
::: hint level=1
content
:::
:::

<!-- VALID -->
::: exercise
id: example
:::

::: hint level=1
content
:::
```

### Whitespace

- Leading/trailing whitespace in content is trimmed
- Blank lines within content are preserved
- Indentation is preserved (important for code blocks)

## Directive Reference

### `::: exercise`

**Purpose:** Defines metadata for the exercise.

**Location:** Should appear near the beginning of the file.

**Syntax:**
```markdown
::: exercise
id: unique-exercise-id
difficulty: beginner
time: 20 minutes
prerequisites: [prereq-1, prereq-2]
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Unique identifier (alphanumeric, hyphens, underscores) |
| `difficulty` | Yes | enum | One of: `beginner`, `intermediate`, `advanced` |
| `time` | No | string | Estimated time (e.g., "20 minutes", "1 hour") |
| `prerequisites` | No | array | List of exercise IDs that should be completed first |

**Example:**
```markdown
::: exercise
id: ch02-01-hello-mcp
difficulty: beginner
time: 20 minutes
prerequisites: [ch01-setup]
:::
```

---

### `::: objectives`

**Purpose:** Lists learning outcomes.

**Syntax:**
```markdown
::: objectives
thinking:
  - First thinking objective
  - Second thinking objective

doing:
  - First doing objective
  - Second doing objective
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `thinking` | No | array | Conceptual understanding goals |
| `doing` | No | array | Practical skill goals |

At least one of `thinking` or `doing` should be provided.

**Rendering:** Displayed as a two-column grid with checkboxes.

---

### `::: discussion`

**Purpose:** Pre-exercise reflection prompts.

**Syntax:**
```markdown
::: discussion
- First discussion question?
- Second discussion question?
- Third prompt or statement.
:::
```

**Content:** A markdown list of discussion items.

**Rendering:** Displayed as a styled list, optionally collapsible.

---

### `::: starter`

**Purpose:** Provides editable starter code for the student.

**Inline Attributes:**

| Attribute | Required | Default | Description |
|-----------|----------|---------|-------------|
| `file` | No | none | Suggested filename (e.g., `src/main.rs`) |
| `language` | No | `rust` | Syntax highlighting language |

**Syntax:**
```markdown
::: starter file="src/main.rs" language=rust
```rust
fn main() {
    // TODO: Your code here
    todo!()
}
```
:::
```

**Content:** A fenced code block with the starter code.

**Rendering:**
- Displayed in an editable textarea (not just a code block)
- Copy button to copy to clipboard
- Reset button to restore original code
- Filename displayed in header if provided

---

### `::: hint`

**Purpose:** Provides progressive hints to help stuck students.

**Inline Attributes:**

| Attribute | Required | Default | Description |
|-----------|----------|---------|-------------|
| `level` | Yes | - | Hint number (1, 2, 3, ...) |
| `title` | No | `Hint {level}` | Display title for the hint |

**Syntax:**
```markdown
::: hint level=1 title="Getting Started"
Start by defining the function signature...
:::

::: hint level=2
Here's a partial implementation:
```rust
fn example() {
    // partial code
}
```
:::

::: hint level=3 title="Full Solution Approach"
```rust
fn example() {
    // nearly complete
}
```
:::
```

**Content:** Markdown content, may include code blocks.

**Rendering:**
- Displayed as collapsible `<details>` elements
- Numbered in the summary
- Initially collapsed
- May be progressively revealed (policy-dependent)

**Ordering:** Hints should be numbered sequentially starting from 1.

---

### `::: solution`

**Purpose:** Provides the complete solution.

**Inline Attributes:**

| Attribute | Required | Default | Description |
|-----------|----------|---------|-------------|
| `reveal` | No | `on-demand` | When to show: `on-demand`, `always`, `never` |

**Syntax:**
```markdown
::: solution
```rust
fn main() {
    println!("Complete solution");
}
```

### Explanation

The solution uses `println!` because...
:::
```

**Content:**
- A fenced code block with the solution
- Optionally followed by a `### Explanation` section with markdown

**Rendering:**
- Hidden by default with a "Show Solution" button
- Warning text encouraging attempt first
- When revealed, shows code + explanation

---

### `::: tests`

**Purpose:** Provides test code that verifies the solution.

**Inline Attributes:**

| Attribute | Required | Default | Description |
|-----------|----------|---------|-------------|
| `mode` | No | `playground` | Execution mode: `playground` or `local` |
| `language` | No | `rust` | Programming language |

**Syntax:**
```markdown
::: tests mode=playground
```rust
#[test]
fn test_basic() {
    assert_eq!(add(2, 2), 4);
}

#[test]
fn test_negative() {
    assert_eq!(add(-1, 1), 0);
}
```
:::
```

**Content:** A fenced code block with test code.

**Mode Behavior:**

| Mode | Description |
|------|-------------|
| `playground` | Shows "Run Tests" button, executes via Rust Playground |
| `local` | Display only, with instructions to run `cargo test` locally |

**Rendering:**
- Code block with syntax highlighting
- "Run Tests" button (if mode=playground)
- Results area for displaying pass/fail output

**Playground Limitations:**
- Only `std` library available
- ~5 second timeout
- Requires internet connection
- Rate limited

---

### `::: reflection`

**Purpose:** Post-exercise questions for deeper understanding.

**Syntax:**
```markdown
::: reflection
- What did you learn from this exercise?
- How would you extend this solution?
- What edge cases did you consider?
:::
```

**Content:** A markdown list of reflection questions.

**Rendering:** Displayed at the end of the exercise, styled as a reflection section.

---

## Complete Example

```markdown
# Exercise: Hello World

::: exercise
id: hello-world
difficulty: beginner
time: 10 minutes
:::

Write a function that returns a personalized greeting.

::: objectives
thinking:
  - Understand Rust string formatting
  - Learn function syntax

doing:
  - Implement a function with a string parameter
  - Use the format! macro
:::

::: discussion
- How do other languages handle string formatting?
- Why might Rust use a macro instead of a method?
:::

## Your Task

Complete the `greet` function to return "Hello, {name}!" where `{name}` is the input.

::: starter file="src/lib.rs"
```rust
/// Returns a greeting for the given name.
///
/// # Examples
///
/// ```
/// assert_eq!(greet("World"), "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    // TODO: Return a greeting string
    todo!()
}
```
:::

::: hint level=1 title="String Formatting in Rust"
Rust uses the `format!` macro for string interpolation.
It works like `println!` but returns a String instead of printing.
:::

::: hint level=2 title="Format Macro Syntax"
```rust
let result = format!("template with {}", variable);
```
:::

::: hint level=3 title="Almost There"
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

### Explanation

The `format!` macro creates a new `String` by interpolating values into a
template. The `{}` is a placeholder that gets replaced with the `name` argument.

Unlike `println!`, which outputs to stdout, `format!` returns the formatted
string, which we then return from the function.
:::

::: tests mode=playground
```rust
#[test]
fn test_greet_world() {
    assert_eq!(greet("World"), "Hello, World!");
}

#[test]
fn test_greet_name() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}

#[test]
fn test_greet_empty() {
    assert_eq!(greet(""), "Hello, !");
}
```
:::

::: reflection
- What happens if you use `println!` instead of `format!`?
- How would you modify this to support different greeting styles?
- What's the difference between `&str` and `String` in the function signature?
:::
```

## Parsing Rules

### Directive Detection

A line starts a directive if it matches:
```regex
^:::[ ]+([a-z][a-z0-9-]*)(.*)$
```

Where:
- Group 1: directive name
- Group 2: optional inline attributes

### Attribute Parsing

Inline attributes are parsed as:
```
key=value key="quoted value" bare-flag
```

YAML-style attributes in the content body are parsed according to YAML 1.2 spec.

### Code Block Handling

Code blocks inside directives are preserved exactly, including:
- Language annotation (```rust, ```python, etc.)
- All whitespace and indentation
- Nested fenced blocks (if using different fence characters)

### Escape Handling

To include literal `:::` in content, use a longer fence:
```markdown
:::: exercise
id: example
::::

Content with literal ::: preserved

::::
```

## Error Handling

The parser should produce helpful errors for:

| Error | Example | Message |
|-------|---------|---------|
| Unclosed directive | `:::` without closing | "Unclosed directive 'exercise' starting at line 5" |
| Missing required field | exercise without id | "Missing required field 'id' in exercise block" |
| Invalid field value | `difficulty: super-hard` | "Invalid value 'super-hard' for 'difficulty' (expected: beginner, intermediate, advanced)" |
| Duplicate directive | Two `:::exercise` blocks | "Duplicate 'exercise' block (only one allowed per file)" |

## Versioning

This syntax specification follows semantic versioning:
- **Major**: Breaking changes to existing directive syntax
- **Minor**: New optional directives or attributes
- **Patch**: Clarifications, typo fixes
