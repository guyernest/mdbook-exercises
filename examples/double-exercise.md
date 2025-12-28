# Two Exercises in One Chapter

Some intro text here to demonstrate content before and between exercises.

## Exercise A

::: exercise
id: double-a
difficulty: beginner
time: 5 minutes
:::

Implement `add(a, b)`.

::: starter file="a.rs" language=rust
```rust
pub fn add(a: i32, b: i32) -> i32 {
    todo!("impl")
}
```
:::

::: solution reveal=on-demand
```rust
pub fn add(a: i32, b: i32) -> i32 { a + b }
```
:::

::: tests mode=playground
```rust
#[test]
fn add_works() { assert_eq!(add(2,3), 5); }
```
:::

Some text between exercises.

## Exercise B

::: exercise
id: double-b
difficulty: intermediate
time: 10 minutes
:::

Implement `factorial(n)`.

::: starter file="b.rs" language=rust
```rust
pub fn factorial(n: u64) -> u64 {
    // iterative or recursive
    todo!()
}
```
:::

::: solution reveal=on-demand
```rust
pub fn factorial(n: u64) -> u64 {
    (1..=n).product()
}
```
:::

::: tests mode=playground
```rust
#[test]
fn fact_small() { assert_eq!(factorial(5), 120); }
```
:::

Concluding remarks after both exercises.

