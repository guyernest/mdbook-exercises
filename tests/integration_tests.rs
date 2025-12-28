//! Integration tests for mdbook-exercises.
//!
//! These tests verify the full flow from markdown to HTML.

use mdbook_exercises::{parse_exercise, Difficulty, Exercise, TestMode};

#[cfg(feature = "render")]
use mdbook_exercises::render::{render_exercise, render_exercise_with_config, RenderConfig};

/// Test parsing and rendering a complete exercise.
#[test]
#[cfg(feature = "render")]
fn test_full_exercise_flow() {
    let markdown = r#"# Exercise: Build a Calculator

::: exercise
id: calculator-basic
difficulty: intermediate
time: 30 minutes
prerequisites:
  - hello-world
  - variables
:::

In this exercise, you'll build a simple calculator that can add two numbers.

::: objectives
thinking:
  - Understand function signatures
  - Learn about return types
doing:
  - Define functions with parameters
  - Return values from functions
:::

::: discussion
- What makes a good function name?
- When should you use parameters vs. hardcoded values?
:::

::: starter
```rust,filename=src/main.rs
fn add(a: i32, b: i32) -> i32 {
    todo!("Implement addition")
}

fn main() {
    let result = add(2, 3);
    println!("2 + 3 = {}", result);
}
```
:::

::: hint level=1 title="Think about the operator"
What operator adds two numbers together?
:::

::: hint level=2
Remember that Rust functions return the last expression (no semicolon needed).
:::

::: solution
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add(2, 3);
    println!("2 + 3 = {}", result);
}
```
The key is using the `+` operator and returning the result without a semicolon.
:::

::: tests mode=playground
```rust
#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
    assert_eq!(add(0, 0), 0);
    assert_eq!(add(-1, 1), 0);
}
```
:::

::: reflection
- How would you extend this to support subtraction?
- What happens with very large numbers?
:::
"#;

    // Parse the exercise
    let exercise = parse_exercise(markdown).expect("Failed to parse exercise");

    // Verify metadata
    assert_eq!(exercise.metadata.id, "calculator-basic");
    assert_eq!(exercise.metadata.difficulty, Difficulty::Intermediate);
    assert_eq!(exercise.metadata.time_minutes, Some(30));
    assert_eq!(
        exercise.metadata.prerequisites,
        vec!["hello-world", "variables"]
    );

    // Verify title
    assert_eq!(
        exercise.title,
        Some("Exercise: Build a Calculator".to_string())
    );

    // Verify description
    assert!(exercise.description.contains("simple calculator"));

    // Verify objectives
    let objectives = exercise
        .objectives
        .as_ref()
        .expect("Should have objectives");
    assert_eq!(objectives.thinking.len(), 2);
    assert_eq!(objectives.doing.len(), 2);
    assert!(objectives.thinking[0].contains("function signatures"));

    // Verify discussion
    let discussion = exercise
        .discussion
        .as_ref()
        .expect("Should have discussion");
    assert_eq!(discussion.len(), 2);

    // Verify starter code
    let starter = exercise.starter.as_ref().expect("Should have starter");
    assert_eq!(starter.language, "rust");
    assert!(starter.code.contains("fn add"));

    // Verify hints
    assert_eq!(exercise.hints.len(), 2);
    assert_eq!(exercise.hints[0].level, 1);
    assert_eq!(
        exercise.hints[0].title,
        Some("Think about the operator".to_string())
    );
    assert_eq!(exercise.hints[1].level, 2);
    assert!(exercise.hints[1].title.is_none());

    // Verify solution
    let solution = exercise.solution.as_ref().expect("Should have solution");
    assert!(solution.code.contains("a + b"));
    assert!(solution.explanation.as_ref().unwrap().contains("key"));

    // Verify tests
    let tests = exercise.tests.as_ref().expect("Should have tests");
    assert_eq!(tests.mode, TestMode::Playground);
    assert!(tests.code.contains("#[test]"));

    // Verify reflection
    let reflection = exercise
        .reflection
        .as_ref()
        .expect("Should have reflection");
    assert_eq!(reflection.len(), 2);

    // Now render the exercise
    let html = render_exercise(&exercise).expect("Failed to render");

    // Verify HTML structure
    assert!(html.contains(r#"data-exercise-id="calculator-basic""#));
    assert!(html.contains(r#"data-difficulty="intermediate""#));
    assert!(html.contains("30 min"));
    assert!(html.contains("hello-world"));
    assert!(html.contains("Learning Objectives"));
    assert!(html.contains("Discussion"));
    assert!(html.contains("Hints"));
    assert!(html.contains("Solution"));
    assert!(html.contains("Tests"));
    assert!(html.contains("Reflection"));
}

/// Test parsing a minimal exercise.
#[test]
fn test_minimal_exercise() {
    let markdown = r#"# Simple Exercise

::: exercise
id: simple-01
difficulty: beginner
:::

Write some code.

::: starter
```rust
fn main() {}
```
:::
"#;

    let exercise = parse_exercise(markdown).expect("Failed to parse");

    assert_eq!(exercise.metadata.id, "simple-01");
    assert_eq!(exercise.metadata.difficulty, Difficulty::Beginner);
    assert!(exercise.metadata.time_minutes.is_none());
    assert!(exercise.metadata.prerequisites.is_empty());

    assert!(exercise.hints.is_empty());
    assert!(exercise.solution.is_none());
    assert!(exercise.tests.is_none());
}

/// Test that code blocks in descriptions are not parsed as directives.
#[test]
fn test_code_blocks_in_description() {
    let markdown = r#"# Code Block Test

::: exercise
id: code-block-test
difficulty: beginner
:::

Here's an example of what NOT to do:

```markdown
::: fake-directive
This should NOT be parsed as a directive!
:::
```

The actual starter is below.

::: starter
```rust
fn main() {
    println!("Real starter");
}
```
:::
"#;

    let exercise = parse_exercise(markdown).expect("Failed to parse");

    // The ID should be from the real exercise block, not any fake one
    assert_eq!(exercise.metadata.id, "code-block-test");
    assert_eq!(exercise.metadata.difficulty, Difficulty::Beginner);

    // The starter should be the actual one
    let starter = exercise.starter.as_ref().expect("Should have starter");
    assert!(starter.code.contains("Real starter"));
}

/// Test multiple hints with and without titles.
#[test]
fn test_multiple_hints() {
    let markdown = r#"
::: exercise
id: hints-test
difficulty: beginner
:::

Test exercise.

::: starter
```rust
fn main() {}
```
:::

::: hint level=1 title="First Hint"
First hint content.
:::

::: hint level=2
Second hint content (no title).
:::

::: hint level=3 title="Third Hint"
Third hint with code:
```rust
let x = 5;
```
:::
"#;

    let exercise = parse_exercise(markdown).expect("Failed to parse");

    assert_eq!(exercise.hints.len(), 3);

    assert_eq!(exercise.hints[0].level, 1);
    assert_eq!(exercise.hints[0].title, Some("First Hint".to_string()));

    assert_eq!(exercise.hints[1].level, 2);
    assert!(exercise.hints[1].title.is_none());

    assert_eq!(exercise.hints[2].level, 3);
    assert_eq!(exercise.hints[2].title, Some("Third Hint".to_string()));
    assert!(exercise.hints[2].content.contains("let x = 5"));
}

/// Test local test mode.
#[test]
fn test_local_test_mode() {
    let markdown = r#"
::: exercise
id: local-test
difficulty: beginner
:::

Test with local mode.

::: starter
```rust
fn main() {}
```
:::

::: tests mode=local
```rust
#[test]
fn local_test() {
    assert!(true);
}
```
:::
"#;

    let exercise = parse_exercise(markdown).expect("Failed to parse");

    let tests = exercise.tests.as_ref().expect("Should have tests");
    assert_eq!(tests.mode, TestMode::Local);
}

/// Test render configuration.
#[test]
#[cfg(feature = "render")]
fn test_render_config() {
    let exercise = Exercise {
        metadata: mdbook_exercises::ExerciseMetadata {
            id: "config-test".to_string(),
            difficulty: Difficulty::Beginner,
            ..Default::default()
        },
        hints: vec![mdbook_exercises::Hint {
            level: 1,
            title: None,
            content: "A hint.".to_string(),
        }],
        solution: Some(mdbook_exercises::Solution {
            code: "fn main() {}".to_string(),
            language: "rust".to_string(),
            explanation: None,
            ..Default::default()
        }),
        ..Default::default()
    };

    // With hints revealed
    let config = RenderConfig {
        reveal_hints: true,
        reveal_solution: false,
        ..Default::default()
    };
    let html = render_exercise_with_config(&exercise, &config).expect("Failed to render");
    assert!(html.contains(r#"<details class="hint" data-level="1" open>"#));

    // With solution revealed
    let config = RenderConfig {
        reveal_hints: false,
        reveal_solution: true,
        ..Default::default()
    };
    let html = render_exercise_with_config(&exercise, &config).expect("Failed to render");
    assert!(html.contains(r#"<details class="solution" open>"#));
}

/// Test solution reveal attribute overrides configuration.
#[test]
#[cfg(feature = "render")]
fn test_solution_reveal_policy_overrides_config() {
    // reveal=always should force open even if config says false
    let markdown_always = r#"
::: exercise
id: sol-reveal-always
difficulty: beginner
:::

::: solution reveal=always
```rust
fn main() {}
```
:::
"#;
    let ex_always = parse_exercise(markdown_always).expect("parse");
    let html_always = render_exercise_with_config(&ex_always, &RenderConfig { reveal_solution: false, ..Default::default() }).expect("render");
    if !html_always.contains(r#"<details class="solution" open>"#) {
        eprintln!("HTML(always) =>\n{}", html_always);
    }
    assert!(html_always.contains(r#"<details class="solution" open>"#));

    // reveal=never should keep closed even if config says true
    let markdown_never = r#"
::: exercise
id: sol-reveal-never
difficulty: beginner
:::

::: solution reveal=never
```rust
fn main() {}
```
:::
"#;
    let ex_never = parse_exercise(markdown_never).expect("parse");
    let html_never = render_exercise_with_config(&ex_never, &RenderConfig { reveal_solution: true, ..Default::default() }).expect("render");
    assert!(html_never.contains(r#"<details class="solution">"#));
    assert!(!html_never.contains(r#"<details class="solution" open>"#));

    // on-demand should follow config
    let markdown_default = r#"
::: exercise
id: sol-reveal-default
difficulty: beginner
:::

::: solution
```rust
fn main() {}
```
:::
"#;
    let ex_default = parse_exercise(markdown_default).expect("parse");
    let html_open = render_exercise_with_config(&ex_default, &RenderConfig { reveal_solution: true, ..Default::default() }).expect("render");
    assert!(html_open.contains(r#"<details class="solution" open>"#));
    let html_closed = render_exercise_with_config(&ex_default, &RenderConfig { reveal_solution: false, ..Default::default() }).expect("render");
    assert!(html_closed.contains(r#"<details class="solution">"#));
    assert!(!html_closed.contains(r#"<details class="solution" open>"#));
}

/// Test JSON serialization of exercises.
#[test]
fn test_exercise_serialization() {
    let markdown = r#"
::: exercise
id: serialize-test
difficulty: advanced
time: 60 minutes
:::

A complex exercise.

::: starter
```rust
fn main() {}
```
:::
"#;

    let exercise = parse_exercise(markdown).expect("Failed to parse");

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&exercise).expect("Failed to serialize");

    // Deserialize back
    let deserialized: Exercise = serde_json::from_str(&json).expect("Failed to deserialize");

    assert_eq!(deserialized.metadata.id, "serialize-test");
    assert_eq!(deserialized.metadata.difficulty, Difficulty::Advanced);
    assert_eq!(deserialized.metadata.time_minutes, Some(60));
}

/// Test advanced difficulty level.
#[test]
fn test_difficulty_levels() {
    let cases = vec![
        ("beginner", Difficulty::Beginner),
        ("intermediate", Difficulty::Intermediate),
        ("advanced", Difficulty::Advanced),
        ("Beginner", Difficulty::Beginner),
        ("INTERMEDIATE", Difficulty::Intermediate),
    ];

    for (input, expected) in cases {
        let markdown = format!(
            r#"
::: exercise
id: diff-test
difficulty: {}
:::

Test.

::: starter
```rust
fn main() {{}}
```
:::
"#,
            input
        );

        let exercise = parse_exercise(&markdown).expect("Failed to parse");
        assert_eq!(
            exercise.metadata.difficulty, expected,
            "Failed for input: {}",
            input
        );
    }
}

/// Test time parsing with various formats.
#[test]
fn test_time_parsing() {
    let cases = vec![
        ("10 minutes", Some(10)),
        ("30 min", Some(30)),
        ("1 hour", Some(60)),
        ("2 hours", Some(120)),
        ("45", Some(45)),
    ];

    for (time_str, expected) in cases {
        let markdown = format!(
            r#"
::: exercise
id: time-test
difficulty: beginner
time: {}
:::

Test.
"#,
            time_str
        );

        let exercise = parse_exercise(&markdown).expect("Failed to parse");
        assert_eq!(
            exercise.metadata.time_minutes, expected,
            "Failed for time string: {}",
            time_str
        );
    }
}

/// Test exercise without starter code.
#[test]
fn test_exercise_without_starter() {
    let markdown = r#"
::: exercise
id: no-starter
difficulty: beginner
:::

This is a discussion-only exercise.

::: discussion
- What is Rust?
- Why use it?
:::
"#;

    let exercise = parse_exercise(markdown).expect("Failed to parse");

    assert_eq!(exercise.metadata.id, "no-starter");
    assert!(exercise.starter.is_none());
    assert!(exercise.discussion.is_some());
}
