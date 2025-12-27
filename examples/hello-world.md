# Exercise: Hello World

::: exercise
id: hello-world
difficulty: beginner
time: 10 minutes
:::

Welcome to your first Rust exercise! You'll write a simple function that
creates a personalized greeting.

This exercise teaches you:
- How to define a function in Rust
- How to use the `format!` macro for string interpolation
- How to work with string slices (`&str`)

::: objectives
thinking:
  - Understand the difference between `&str` and `String`
  - Learn how Rust's `format!` macro works

doing:
  - Implement a function that takes a string parameter
  - Return a formatted greeting string
:::

::: discussion
Before we start coding, let's think about the problem:

- How do you create formatted strings in other programming languages?
- Why might Rust use a macro (`format!`) instead of a method?
- What's the difference between printing a string and returning one?
:::

## Your Task

Complete the `greet` function below to return the string `"Hello, {name}!"`
where `{name}` is replaced with the input parameter.

For example:
- `greet("World")` should return `"Hello, World!"`
- `greet("Alice")` should return `"Hello, Alice!"`

::: starter file="src/lib.rs"
```rust
/// Returns a personalized greeting.
///
/// # Arguments
///
/// * `name` - The name to include in the greeting
///
/// # Examples
///
/// ```
/// let greeting = greet("World");
/// assert_eq!(greeting, "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    // TODO: Return a greeting that includes the name
    // Hint: Use the format! macro
    todo!("Implement the greet function")
}
```
:::

## Hints

If you get stuck, try these hints one at a time:

::: hint level=1 title="String Formatting in Rust"
Rust uses the `format!` macro to create formatted strings. It works similarly
to `println!`, but instead of printing to the console, it returns a `String`.

```rust
let s = format!("Hello, {}!", "World");
// s is now "Hello, World!"
```

The `{}` is a placeholder that gets replaced with the argument.
:::

::: hint level=2 title="Using the name Parameter"
You need to use the `name` parameter inside the `format!` macro:

```rust
format!("some text with {}", name)
```

Remember, you need to include "Hello, " at the start and "!" at the end.
:::

::: hint level=3 title="The Complete Solution"
Here's the full implementation:

```rust
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

That's it! The `format!` macro handles creating the `String` for you.
:::

## Solution

::: solution
```rust
/// Returns a personalized greeting.
///
/// # Arguments
///
/// * `name` - The name to include in the greeting
///
/// # Examples
///
/// ```
/// let greeting = greet("World");
/// assert_eq!(greeting, "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Explanation

The solution uses the `format!` macro, which is part of Rust's standard library.

**Why `format!` instead of string concatenation?**

In some languages, you might write:
```javascript
"Hello, " + name + "!"  // JavaScript
```

But Rust's `+` operator for strings has specific ownership requirements. The
`format!` macro is more flexible and idiomatic for creating formatted strings.

**How does `format!` work?**

1. It takes a format string with placeholders (`{}`)
2. Each `{}` is replaced with the corresponding argument
3. It returns a new `String` (owned data on the heap)

**Why does the function return `String` but take `&str`?**

- `&str` is a borrowed reference to string data - we just need to read the name
- `String` is an owned, growable string - we're creating new data
- This pattern (borrow input, return owned) is very common in Rust

You could also write it as:
```rust
pub fn greet(name: &str) -> String {
    let mut greeting = String::from("Hello, ");
    greeting.push_str(name);
    greeting.push('!');
    greeting
}
```

But `format!` is cleaner and more readable for this use case.
:::

## Tests

Run these tests to verify your solution:

::: tests mode=playground
```rust
// Note: The greet function should be defined above

#[test]
fn test_greet_world() {
    assert_eq!(greet("World"), "Hello, World!");
}

#[test]
fn test_greet_name() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}

#[test]
fn test_greet_with_spaces() {
    assert_eq!(greet("John Doe"), "Hello, John Doe!");
}

#[test]
fn test_greet_empty() {
    // Edge case: what happens with an empty name?
    assert_eq!(greet(""), "Hello, !");
}
```
:::

## Reflection

::: reflection
Now that you've completed the exercise, consider these questions:

1. **String types**: What would happen if the function returned `&str` instead
   of `String`? Why doesn't that work?

2. **Ownership**: The `format!` macro creates a new `String`. Who owns that
   `String` after the function returns?

3. **Alternatives**: Can you think of other ways to solve this problem?
   What are the tradeoffs?

4. **Edge cases**: The test includes an empty string. Are there other edge
   cases you'd want to test for in a production system?

5. **Extension**: How would you modify this function to support different
   greeting styles (formal, casual, etc.)?
:::
