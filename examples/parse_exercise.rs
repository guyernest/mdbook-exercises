use mdbook_exercises::parse_exercise;

fn main() {
    let markdown = r#"
# Example

::: exercise
id: example
difficulty: beginner
:::

Hello world.

::: starter
```rust,filename=src/main.rs
fn main() {}
```
:::
    "#;

    match parse_exercise(markdown) {
        Ok(exercise) => {
            println!("Parsed exercise: {}", exercise.metadata.id);
            if let Some(st) = exercise.starter.as_ref() {
                println!("Starter language: {}", st.language);
                println!("Starter filename: {:?}", st.filename);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
