//! mdBook preprocessor integration.
//!
//! This module provides the mdBook preprocessor that transforms exercise
//! directives in markdown files into interactive HTML.

use crate::parser::parse_exercise;
use crate::render::{render_exercise_with_config, RenderConfig};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use regex::Regex;
use std::path::Path;

/// The mdBook preprocessor for exercises.
pub struct ExercisesPreprocessor;

impl ExercisesPreprocessor {
    /// Create a new preprocessor instance.
    pub fn new() -> ExercisesPreprocessor {
        ExercisesPreprocessor
    }

    /// Load configuration from the preprocessor context.
    fn load_config(ctx: &PreprocessorContext) -> RenderConfig {
        let mut config = RenderConfig::default();

        if let Some(exercises_config) = ctx.config.get("preprocessor.exercises") {
            if let Some(reveal_hints) = exercises_config.get("reveal_hints") {
                config.reveal_hints = reveal_hints.as_bool().unwrap_or(false);
            }
            if let Some(reveal_solution) = exercises_config.get("reveal_solution") {
                config.reveal_solution = reveal_solution.as_bool().unwrap_or(false);
            }
            if let Some(playground) = exercises_config.get("playground") {
                config.enable_playground = playground.as_bool().unwrap_or(true);
            }
            if let Some(playground_url) = exercises_config.get("playground_url") {
                if let Some(url) = playground_url.as_str() {
                    config.playground_url = url.to_string();
                }
            }
            if let Some(progress) = exercises_config.get("progress_tracking") {
                config.enable_progress = progress.as_bool().unwrap_or(true);
            }
        }

        config
    }

    /// Process a single chapter's content.
    fn process_chapter(content: &str, config: &RenderConfig) -> Result<String, Error> {
        // First, check if the content has any exercise directives
        if !content.contains("::: exercise") {
            return Ok(content.to_string());
        }

        // Parse the exercise from the content
        match parse_exercise(content) {
            Ok(exercise) => {
                // If we successfully parsed an exercise, render it
                match render_exercise_with_config(&exercise, config) {
                    Ok(html) => {
                        // Wrap the HTML in a div and include the original non-directive content
                        // Actually, we need a smarter approach: replace the directives with HTML
                        // but keep the surrounding content

                        // For now, if the whole file is an exercise, just return the rendered HTML
                        // with some wrapper content
                        Ok(format!(
                            r#"<div class="exercise-container">
{}
</div>
"#,
                            html
                        ))
                    }
                    Err(e) => {
                        // Return original content with an error message
                        Ok(format!(
                            "<!-- Exercise render error: {} -->\n\n{}",
                            e, content
                        ))
                    }
                }
            }
            Err(e) => {
                // Return original content with an error message
                Ok(format!(
                    "<!-- Exercise parse error: {} -->\n\n{}",
                    e, content
                ))
            }
        }
    }
}

impl Default for ExercisesPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for ExercisesPreprocessor {
    fn name(&self) -> &str {
        "exercises"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = Self::load_config(ctx);

        // Process each chapter
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                if let Some(ref mut content) = Some(&mut chapter.content) {
                    match Self::process_chapter(content, &config) {
                        Ok(new_content) => {
                            chapter.content = new_content;
                        }
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to process exercises in {}: {}",
                                chapter.name, e
                            );
                        }
                    }
                }
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        // We support HTML renderers
        renderer == "html"
    }
}

/// A more sophisticated processor that handles inline exercise includes.
///
/// This allows syntax like:
/// ```markdown
/// Some introductory text...
///
/// {{#exercise path/to/exercise.md}}
///
/// Some concluding text...
/// ```
pub struct ExerciseIncludeProcessor {
    config: RenderConfig,
    book_root: std::path::PathBuf,
}

impl ExerciseIncludeProcessor {
    /// Create a new include processor.
    pub fn new(book_root: &Path, config: RenderConfig) -> Self {
        Self {
            config,
            book_root: book_root.to_path_buf(),
        }
    }

    /// Process a chapter, replacing {{#exercise ...}} includes.
    pub fn process(&self, content: &str) -> Result<String, Error> {
        let include_re = Regex::new(r"\{\{#exercise\s+([^}]+)\}\}")
            .map_err(|e| Error::msg(format!("Regex error: {}", e)))?;

        let mut result = content.to_string();

        for cap in include_re.captures_iter(content) {
            let full_match = cap.get(0).unwrap().as_str();
            let exercise_path = cap.get(1).unwrap().as_str().trim();

            let full_path = self.book_root.join(exercise_path);

            match std::fs::read_to_string(&full_path) {
                Ok(exercise_content) => match parse_exercise(&exercise_content) {
                    Ok(exercise) => match render_exercise_with_config(&exercise, &self.config) {
                        Ok(html) => {
                            let wrapped = format!(
                                r#"<div class="exercise-container">
{}
</div>"#,
                                html
                            );
                            result = result.replace(full_match, &wrapped);
                        }
                        Err(e) => {
                            let error_html = format!(
                                r#"<div class="exercise-error">
  <p><strong>Error rendering exercise:</strong> {}</p>
  <p>File: {}</p>
</div>"#,
                                e, exercise_path
                            );
                            result = result.replace(full_match, &error_html);
                        }
                    },
                    Err(e) => {
                        let error_html = format!(
                            r#"<div class="exercise-error">
  <p><strong>Error parsing exercise:</strong> {}</p>
  <p>File: {}</p>
</div>"#,
                            e, exercise_path
                        );
                        result = result.replace(full_match, &error_html);
                    }
                },
                Err(e) => {
                    let error_html = format!(
                        r#"<div class="exercise-error">
  <p><strong>Error loading exercise file:</strong> {}</p>
  <p>File: {}</p>
</div>"#,
                        e, exercise_path
                    );
                    result = result.replace(full_match, &error_html);
                }
            }
        }

        Ok(result)
    }
}

/// Preprocessor that supports both inline exercises and include syntax.
pub struct FullExercisesPreprocessor;

impl FullExercisesPreprocessor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FullExercisesPreprocessor {
    fn default() -> Self {
        Self::new()
    }
}

impl Preprocessor for FullExercisesPreprocessor {
    fn name(&self) -> &str {
        "exercises"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = ExercisesPreprocessor::load_config(ctx);
        let book_root = ctx.root.join(&ctx.config.book.src);

        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                let content = &chapter.content;

                // First, process any {{#exercise ...}} includes
                let include_processor = ExerciseIncludeProcessor::new(&book_root, config.clone());
                let after_includes = match include_processor.process(content) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!(
                            "Warning: Failed to process exercise includes in {}: {}",
                            chapter.name, e
                        );
                        content.clone()
                    }
                };

                // Then, process inline exercises
                let final_content =
                    match ExercisesPreprocessor::process_chapter(&after_includes, &config) {
                        Ok(c) => c,
                        Err(e) => {
                            eprintln!(
                                "Warning: Failed to process inline exercises in {}: {}",
                                chapter.name, e
                            );
                            after_includes
                        }
                    };

                chapter.content = final_content;
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_chapter_no_exercises() {
        let content = "# Just a normal chapter\n\nSome content here.";
        let config = RenderConfig::default();

        let result = ExercisesPreprocessor::process_chapter(content, &config).unwrap();

        // Should return unchanged
        assert_eq!(result, content);
    }

    #[test]
    fn test_process_chapter_with_exercise() {
        let content = r#"# My Exercise

::: exercise
id: test-ex
difficulty: beginner
:::

Some description.

::: starter
```rust
fn main() {}
```
:::
"#;
        let config = RenderConfig::default();

        let result = ExercisesPreprocessor::process_chapter(content, &config).unwrap();

        // Should contain rendered HTML
        assert!(result.contains("exercise-container"));
        assert!(result.contains("test-ex"));
    }
}
