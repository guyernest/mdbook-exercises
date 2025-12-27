use mdbook_exercises::parse_exercise;

fn main() {
    let markdown = r#"
# Example

::: exercise
id: example
difficulty: beginner
:::

Hello world.
    "#;

    match parse_exercise(markdown) {
        Ok(exercise) => println!("Parsed exercise: {}", exercise.metadata.id),
        Err(e) => eprintln!("Error: {}", e),
    }
}
