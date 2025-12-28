//! HTML rendering for exercises.
//!
//! This module transforms parsed `Exercise` structs into HTML suitable for
//! display in mdBook. The generated HTML includes:
//!
//! - Exercise metadata (difficulty, time, prerequisites)
//! - Learning objectives with checkboxes
//! - Editable starter code
//! - Collapsible hints
//! - Hidden solution with reveal button
//! - Test code with optional "Run Tests" button
//! - Reflection questions

use crate::types::*;
use pulldown_cmark::{html, Parser};

/// Errors that can occur during rendering.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    #[error("Template error: {0}")]
    TemplateError(String),

    #[error("Missing required field: {0}")]
    MissingField(String),
}

/// Configuration for rendering.
#[derive(Debug, Clone)]
pub struct RenderConfig {
    /// Show all hints expanded by default
    pub reveal_hints: bool,

    /// Show solution expanded by default
    pub reveal_solution: bool,

    /// Enable Rust Playground integration
    pub enable_playground: bool,

    /// Custom playground URL
    pub playground_url: String,

    /// Enable progress tracking via localStorage
    pub enable_progress: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            reveal_hints: false,
            reveal_solution: false,
            enable_playground: true,
            playground_url: "https://play.rust-lang.org".to_string(),
            enable_progress: true,
        }
    }
}

/// Render an exercise to HTML.
pub fn render_exercise(exercise: &Exercise) -> Result<String, RenderError> {
    render_exercise_with_config(exercise, &RenderConfig::default())
}

/// Render an exercise to HTML with custom configuration.
pub fn render_exercise_with_config(
    exercise: &Exercise,
    config: &RenderConfig,
) -> Result<String, RenderError> {
    let mut html = String::new();

    // Opening article tag with data attributes
    html.push_str(&format!(
        r#"<article class="exercise" data-exercise-id="{}" data-difficulty="{}">"#,
        escape_html(&exercise.metadata.id),
        exercise.metadata.difficulty
    ));
    html.push('\n');

    // Header with title and metadata
    html.push_str(&render_header(exercise));

    // Section navigation outline
    html.push_str(&render_navigation(exercise));

    // Description
    if !exercise.description.is_empty() {
        html.push_str(&render_description(&exercise.description, &exercise.metadata.id));
    }

    // Objectives
    if let Some(objectives) = &exercise.objectives {
        html.push_str(&render_objectives(objectives, &exercise.metadata.id));
    }

    // Discussion
    if let Some(discussion) = &exercise.discussion {
        html.push_str(&render_discussion(discussion));
    }

    // Starter code
    if let Some(starter) = &exercise.starter {
        html.push_str(&render_starter(starter, &exercise.metadata.id));
    }

    // Hints
    if !exercise.hints.is_empty() {
        html.push_str(&render_hints(&exercise.hints, config.reveal_hints, &exercise.metadata.id));
    }

    // Solution
    if let Some(solution) = &exercise.solution {
        html.push_str(&render_solution(solution, config.reveal_solution, &exercise.metadata.id));
    }

    // Tests
    if let Some(tests) = &exercise.tests {
        html.push_str(&render_tests(tests, &exercise.metadata.id, config));
    }

    // Reflection
    if let Some(reflection) = &exercise.reflection {
        html.push_str(&render_reflection(reflection, &exercise.metadata.id));
    }

    // Footer with complete button
    if config.enable_progress {
        html.push_str(&render_footer(&exercise.metadata.id));
    }

    // Closing article tag
    html.push_str("</article>\n");

    Ok(html)
}

/// Render section navigation outline
fn render_navigation(exercise: &Exercise) -> String {
    let mut html = String::new();
    let id = &exercise.metadata.id;

    html.push_str(r#"<nav class="exercise-nav" aria-label="Exercise sections">"#);
    html.push('\n');
    html.push_str("  <ul>\n");

    // Always have description
    if !exercise.description.is_empty() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-description\" data-section=\"description\">📖 Overview</a></li>",
            id
        ));
        html.push('\n');
    }

    if exercise.objectives.is_some() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-objectives\" data-section=\"objectives\">🎯 Objectives</a></li>",
            id
        ));
        html.push('\n');
    }

    if exercise.starter.is_some() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-starter\" data-section=\"starter\">💻 Code</a></li>",
            id
        ));
        html.push('\n');
    }

    if !exercise.hints.is_empty() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-hints\" data-section=\"hints\">💡 Hints</a></li>",
            id
        ));
        html.push('\n');
    }

    if exercise.solution.is_some() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-solution\" data-section=\"solution\">✅ Solution</a></li>",
            id
        ));
        html.push('\n');
    }

    if exercise.tests.is_some() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-tests\" data-section=\"tests\">🧪 Tests</a></li>",
            id
        ));
        html.push('\n');
    }

    if exercise.reflection.is_some() {
        html.push_str(&format!(
            "    <li><a href=\"#{}-reflection\" data-section=\"reflection\">🤔 Reflect</a></li>",
            id
        ));
        html.push('\n');
    }

    html.push_str("  </ul>\n");
    html.push_str("</nav>\n");

    html
}

/// Render the exercise header with title and metadata.
fn render_header(exercise: &Exercise) -> String {
    let mut html = String::new();

    html.push_str(r#"<header class="exercise-header">"#);
    html.push('\n');

    // Title
    if let Some(title) = &exercise.title {
        html.push_str(&format!(
            r#"  <h2 class="exercise-title">{}</h2>"#,
            escape_html(title)
        ));
        html.push('\n');
    }

    // Metadata badges
    html.push_str(r#"  <div class="exercise-meta">"#);
    html.push('\n');

    // Difficulty badge
    let difficulty_class = match exercise.metadata.difficulty {
        Difficulty::Beginner => "beginner",
        Difficulty::Intermediate => "intermediate",
        Difficulty::Advanced => "advanced",
    };
    let difficulty_icon = match exercise.metadata.difficulty {
        Difficulty::Beginner => "⭐",
        Difficulty::Intermediate => "⭐⭐",
        Difficulty::Advanced => "⭐⭐⭐",
    };
    html.push_str(&format!(
        r#"    <span class="badge difficulty {}">{} {}</span>"#,
        difficulty_class, difficulty_icon, exercise.metadata.difficulty
    ));
    html.push('\n');

    // Time estimate
    if let Some(minutes) = exercise.metadata.time_minutes {
        let time_str = if minutes >= 60 {
            format!("{}h {}m", minutes / 60, minutes % 60)
        } else {
            format!("{} min", minutes)
        };
        html.push_str(&format!(
            r#"    <span class="badge time">⏱️ {}</span>"#,
            time_str
        ));
        html.push('\n');
    }

    // Prerequisites
    if !exercise.metadata.prerequisites.is_empty() {
        let prereqs: Vec<String> = exercise
            .metadata
            .prerequisites
            .iter()
            .map(|p| format!("<a href=\"#{}\">{}</a>", escape_html(p), escape_html(p)))
            .collect();
        html.push_str(&format!(
            r#"    <span class="badge prerequisites">📚 Requires: {}</span>"#,
            prereqs.join(", ")
        ));
        html.push('\n');
    }

    html.push_str("  </div>\n");
    html.push_str("</header>\n");

    html
}

/// Render the description section.
fn render_description(description: &str, exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-description" id="{}-description">"#,
        exercise_id
    ));
    html.push('\n');

    // Convert markdown to HTML
    let parser = Parser::new(description);
    let mut description_html = String::new();
    html::push_html(&mut description_html, parser);

    html.push_str(&description_html);
    html.push_str("</section>\n");

    html
}

/// Render the objectives section.
fn render_objectives(objectives: &Objectives, exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-objectives" id="{}-objectives">"#,
        exercise_id
    ));
    html.push('\n');
    html.push_str("  <h3>🎯 Learning Objectives</h3>\n");
    html.push_str(r#"  <div class="objectives-grid">"#);
    html.push('\n');

    // Thinking objectives
    if !objectives.thinking.is_empty() {
        html.push_str(r#"    <div class="objectives-thinking">"#);
        html.push('\n');
        html.push_str("      <h4>Thinking</h4>\n");
        html.push_str("      <ul>\n");
        for (i, obj) in objectives.thinking.iter().enumerate() {
            let id = format!("{}-thinking-{}", exercise_id, i);
            html.push_str(&format!(
                r#"        <li><input type="checkbox" id="{}" class="objective-checkbox"><label for="{}">{}</label></li>"#,
                id, id, escape_html(obj)
            ));
            html.push('\n');
        }
        html.push_str("      </ul>\n");
        html.push_str("    </div>\n");
    }

    // Doing objectives
    if !objectives.doing.is_empty() {
        html.push_str(r#"    <div class="objectives-doing">"#);
        html.push('\n');
        html.push_str("      <h4>Doing</h4>\n");
        html.push_str("      <ul>\n");
        for (i, obj) in objectives.doing.iter().enumerate() {
            let id = format!("{}-doing-{}", exercise_id, i);
            html.push_str(&format!(
                r#"        <li><input type="checkbox" id="{}" class="objective-checkbox"><label for="{}">{}</label></li>"#,
                id, id, escape_html(obj)
            ));
            html.push('\n');
        }
        html.push_str("      </ul>\n");
        html.push_str("    </div>\n");
    }

    html.push_str("  </div>\n");
    html.push_str("</section>\n");

    html
}

/// Render the discussion section.
fn render_discussion(discussion: &[String]) -> String {
    let mut html = String::new();

    html.push_str(r#"<section class="exercise-discussion">"#);
    html.push('\n');
    html.push_str("  <h3>💬 Discussion</h3>\n");
    html.push_str("  <ul>\n");
    for item in discussion {
        html.push_str(&format!("    <li>{}</li>\n", escape_html(item)));
    }
    html.push_str("  </ul>\n");
    html.push_str("</section>\n");

    html
}

/// Render the starter code section.
fn render_starter(starter: &StarterCode, exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-starter" id="{}-starter">"#,
        exercise_id
    ));
    html.push('\n');

    // Header with filename and buttons
    html.push_str(r#"  <div class="code-header">"#);
    html.push('\n');
    if let Some(filename) = &starter.filename {
        html.push_str(&format!(
            r#"    <span class="filename">{}</span>"#,
            escape_html(filename)
        ));
        html.push('\n');
    }
    html.push_str(r#"    <div class="code-actions">"#);
    html.push('\n');
    html.push_str(&format!(
        r#"      <button class="btn btn-copy" data-target="code-{}" title="Copy code">📋 Copy</button>"#,
        exercise_id
    ));
    html.push('\n');
    html.push_str(&format!(
        r#"      <button class="btn btn-reset" data-target="code-{}" title="Reset to original">↺ Reset</button>"#,
        exercise_id
    ));
    html.push('\n');
    html.push_str("    </div>\n");
    html.push_str("  </div>\n");

    // Editable code area - leave body empty, JS will populate from data-original
    // This avoids mdBook's markdown processor corrupting the content
    html.push_str(&format!(
        r#"  <textarea class="code-editor" id="code-{}" data-language="{}" data-original="{}" spellcheck="false"></textarea>"#,
        exercise_id,
        escape_html(&starter.language),
        escape_html_attr(&starter.code)
    ));
    html.push('\n');

    html.push_str("</section>\n");

    html
}

/// Render the hints section.
fn render_hints(hints: &[Hint], reveal: bool, exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-hints" id="{}-hints">"#,
        exercise_id
    ));
    html.push('\n');
    html.push_str("  <h3>💡 Hints</h3>\n");

    for hint in hints {
        let open_attr = if reveal { " open" } else { "" };
        let title = hint
            .title
            .as_ref()
            .map(|t| format!("Hint {}: {}", hint.level, t))
            .unwrap_or_else(|| format!("Hint {}", hint.level));

        html.push_str(&format!(
            r#"  <details class="hint" data-level="{}"{}>
    <summary>{}</summary>
    <div class="hint-content">
"#,
            hint.level,
            open_attr,
            escape_html(&title)
        ));

        // Render hint content as markdown
        let parser = Parser::new(&hint.content);
        let mut hint_html = String::new();
        html::push_html(&mut hint_html, parser);
        html.push_str(&hint_html);

        html.push_str("    </div>\n");
        html.push_str("  </details>\n");
    }

    html.push_str("</section>\n");

    html
}

/// Render the solution section.
fn render_solution(solution: &Solution, reveal: bool, exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-solution" id="{}-solution">"#,
        exercise_id
    ));
    html.push('\n');

    let open_attr = if reveal { " open" } else { "" };
    html.push_str(&format!(
        r#"  <details class="solution"{}>
    <summary>
      <span class="solution-warning">⚠️ Try the exercise first!</span>
      <span class="solution-toggle">Show Solution</span>
    </summary>
    <div class="solution-content">
"#,
        open_attr
    ));

    // Solution code
    html.push_str(&format!(
        r#"      <pre><code class="language-{}">{}</code></pre>"#,
        escape_html(&solution.language),
        escape_html(&solution.code)
    ));
    html.push('\n');

    // Explanation
    if let Some(explanation) = &solution.explanation {
        html.push_str(r#"      <div class="solution-explanation">"#);
        html.push('\n');
        html.push_str("        <h4>Explanation</h4>\n");

        let parser = Parser::new(explanation);
        let mut explanation_html = String::new();
        html::push_html(&mut explanation_html, parser);
        html.push_str(&explanation_html);

        html.push_str("      </div>\n");
    }

    html.push_str("    </div>\n");
    html.push_str("  </details>\n");
    html.push_str("</section>\n");

    html
}

/// Render the tests section.
fn render_tests(tests: &TestBlock, exercise_id: &str, config: &RenderConfig) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-tests" id="{}-tests" data-mode="{}">"#,
        exercise_id,
        tests.mode
    ));
    html.push('\n');
    html.push_str("  <h3>🧪 Tests</h3>\n");

    // Actions first (Run button at top)
    html.push_str(r#"  <div class="test-actions">"#);
    html.push('\n');

    if tests.mode == TestMode::Playground && config.enable_playground {
        html.push_str(&format!(
            r#"    <button class="btn btn-run-tests" data-exercise-id="{}" data-playground-url="{}">▶ Run Tests</button>"#,
            exercise_id,
            escape_html(&config.playground_url)
        ));
        html.push('\n');
    } else {
        html.push_str(r#"    <div class="local-test-info">"#);
        html.push('\n');
        html.push_str("      <p>Run these tests locally with:</p>\n");
        html.push_str("      <pre><code>cargo test</code></pre>\n");
        html.push_str("    </div>\n");
    }

    html.push_str("  </div>\n");

    // Results area (populated by JavaScript)
    html.push_str(&format!(
        r#"  <div class="test-results" id="results-{}" hidden></div>"#,
        exercise_id
    ));
    html.push('\n');

    // Test code in collapsible details (collapsed by default to avoid spoilers)
    html.push_str(r#"  <details class="tests-code">"#);
    html.push('\n');
    html.push_str("    <summary>View Test Code</summary>\n");
    html.push_str(&format!(
        r#"    <pre><code class="language-{}">{}</code></pre>"#,
        escape_html(&tests.language),
        escape_html(&tests.code)
    ));
    html.push('\n');
    html.push_str("  </details>\n");

    html.push_str("</section>\n");

    html
}

/// Render the reflection section.
fn render_reflection(reflection: &[String], exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<section class="exercise-reflection" id="{}-reflection">"#,
        exercise_id
    ));
    html.push('\n');
    html.push_str("  <h3>🤔 Reflection</h3>\n");
    html.push_str("  <ul>\n");
    for item in reflection {
        html.push_str(&format!("    <li>{}</li>\n", escape_html(item)));
    }
    html.push_str("  </ul>\n");
    html.push_str("</section>\n");

    html
}

/// Render the footer with completion button.
fn render_footer(exercise_id: &str) -> String {
    let mut html = String::new();

    html.push_str(r#"<footer class="exercise-footer">"#);
    html.push('\n');
    html.push_str(&format!(
        r#"  <button class="btn btn-complete" data-exercise-id="{}">✓ Mark Complete</button>"#,
        exercise_id
    ));
    html.push('\n');
    html.push_str("</footer>\n");

    html
}

/// Escape HTML special characters.
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Escape HTML for use in attributes (more aggressive escaping).
fn escape_html_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
        .replace('\n', "&#10;")
        .replace('\r', "&#13;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple_exercise() {
        let exercise = Exercise {
            metadata: ExerciseMetadata {
                id: "test-exercise".to_string(),
                difficulty: Difficulty::Beginner,
                time_minutes: Some(15),
                prerequisites: vec![],
            },
            title: Some("Test Exercise".to_string()),
            description: "A simple test exercise.".to_string(),
            ..Default::default()
        };

        let html = render_exercise(&exercise).unwrap();

        assert!(html.contains(r#"data-exercise-id="test-exercise""#));
        assert!(html.contains("Test Exercise"));
        assert!(html.contains("beginner"));
        assert!(html.contains("15 min"));
    }

    #[test]
    fn test_render_with_hints() {
        let exercise = Exercise {
            metadata: ExerciseMetadata {
                id: "hint-test".to_string(),
                difficulty: Difficulty::Intermediate,
                ..Default::default()
            },
            hints: vec![
                Hint {
                    level: 1,
                    title: Some("First Hint".to_string()),
                    content: "This is hint 1.".to_string(),
                },
                Hint {
                    level: 2,
                    title: None,
                    content: "This is hint 2.".to_string(),
                },
            ],
            ..Default::default()
        };

        let html = render_exercise(&exercise).unwrap();

        assert!(html.contains("Hint 1: First Hint"));
        assert!(html.contains("Hint 2"));
        assert!(html.contains(r#"data-level="1""#));
        assert!(html.contains(r#"data-level="2""#));
    }

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>"), "&lt;script&gt;");
        assert_eq!(escape_html("a & b"), "a &amp; b");
        assert_eq!(escape_html(r#""quoted""#), "&quot;quoted&quot;");
    }
}
