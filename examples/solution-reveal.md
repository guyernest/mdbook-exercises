# Exercise: Squares Sum

::: exercise
id: squares-sum
difficulty: beginner
time: 5 minutes
:::

Implement `sum_squares(n)` which returns the sum of squares from 1..=n.

::: starter file="src/lib.rs"
```rust
pub fn sum_squares(n: u64) -> u64 {
    // TODO: implement
    todo!()
}
```
:::

::: solution reveal=always
```rust
pub fn sum_squares(n: u64) -> u64 {
    (1..=n).map(|x| x*x).sum()
}
```
:::

::: tests mode=playground
```rust
#[test]
fn small() {
    assert_eq!(sum_squares(3), 14);
}
```
:::

::: reflection
- Why might `reveal=always` be useful for quick-reference exercises?
:::
