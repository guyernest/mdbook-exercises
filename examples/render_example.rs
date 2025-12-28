use mdbook_exercises::{parse_exercise, render_exercise};
use std::fs;
use std::path::Path;

fn main() {
    // Get input file from command line args or use default
    let args: Vec<String> = std::env::args().collect();
    let input_path = if args.len() > 1 {
        &args[1]
    } else {
        "examples/hello-world.md"
    };

    // Read the markdown file
    let markdown = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Could not read {}", input_path));

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

    // Determine output path (same name as input, but .html extension)
    let input = Path::new(input_path);
    let output_name = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let output_path = format!("{}.html", output_name);

    fs::write(&output_path, &html)
        .expect("Failed to write HTML output");

    println!("Success! Rendered HTML saved to: {}", output_path);
    println!("\nTo view the result, open {} in your browser.", output_path);
    println!("(Note: The CSS and JS from the assets/ directory are required for full functionality)");
}

