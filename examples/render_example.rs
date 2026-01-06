use mdbook_exercises::{parse_exercise, render_exercise, ParsedExercise, ParseError};
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
    println!("Parsing {}...", input_path);
    let parsed = match parse_exercise(&markdown) {
        Ok(p) => p,
        Err(ParseError::UnknownExerciseType) => {
            println!("Skipping {} - not an exercise file (no ::: exercise or ::: usecase directive)", input_path);
            return;
        }
        Err(e) => panic!("Failed to parse exercise: {}", e),
    };

    // Extract info based on exercise type
    let (id, title_opt) = match &parsed {
        ParsedExercise::Code(ex) => (ex.metadata.id.clone(), ex.title.clone()),
        ParsedExercise::UseCase(ex) => (ex.metadata.id.clone(), ex.title.clone()),
    };

    println!("Exercise ID: {}", id);
    println!("Title: {:?}", title_opt);

    // Render it
    println!("Rendering to HTML...");
    let exercise_html = render_exercise(&parsed)
        .expect("Failed to render exercise");

    // Get title for the page
    let title = title_opt.as_deref().unwrap_or("Exercise");

    // Wrap in a full HTML page
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title}</title>
  <link rel="stylesheet" href="assets/exercises.css">
  <style>
    body {{
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
      max-width: 900px;
      margin: 0 auto;
      padding: 2rem;
      background: var(--bg, #fafafa);
      color: var(--fg, #333);
    }}
    .back-link {{
      display: inline-block;
      margin-bottom: 1rem;
      color: var(--links, #4183c4);
      text-decoration: none;
    }}
    .back-link:hover {{
      text-decoration: underline;
    }}
  </style>
</head>
<body>
  <a href="index.html" class="back-link">← Back to Examples</a>
  {exercise_html}
  <script src="assets/exercises.js"></script>
</body>
</html>
"#,
        title = title,
        exercise_html = exercise_html
    );

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
}

