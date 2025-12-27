use mdbook_exercises::{parse_exercise, render_exercise};
use std::fs;

fn main() {
    // Read the hello-world example
    let markdown = fs::read_to_string("examples/hello-world.md")
        .expect("Could not read examples/hello-world.md");

    // Parse it
    println!("Parsing exercise...");
    let exercise = parse_exercise(&markdown)
        .expect("Failed to parse exercise");

    println!("Exercise ID: {}", exercise.metadata.id);
    println!("Title: {:?}", exercise.title);

    // Render it
    println!("Rendering to HTML...");
    let html = render_exercise(&exercise)
        .expect("Failed to render exercise");

    // Write to file
    let output_path = "hello-world.html";
    fs::write(output_path, &html)
        .expect("Failed to write HTML output");

    println!("Success! Rendered HTML saved to: {}", output_path);
    println!("\nTo view the result, open {} in your browser.", output_path);
    println!("(Note: The CSS and JS from the assets/ directory are required for full functionality)");
}

