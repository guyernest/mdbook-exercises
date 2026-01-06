use mdbook_exercises::{parse_exercise, ParsedExercise};

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
        Ok(ParsedExercise::Code(exercise)) => {
            println!("Parsed code exercise: {}", exercise.metadata.id);
            if let Some(st) = exercise.starter.as_ref() {
                println!("Starter language: {}", st.language);
                println!("Starter filename: {:?}", st.filename);
            }
        }
        Ok(ParsedExercise::UseCase(exercise)) => {
            println!("Parsed usecase exercise: {}", exercise.metadata.id);
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
