//! HTML rendering for exercises.
//!
//! This module transforms parsed exercises into HTML suitable for
//! display in mdBook.

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

    /// Enable or disable this preprocessor (checked in preprocessor run)
    pub enabled: bool,

    /// If true, copy CSS/JS assets into the book's theme directory
    pub manage_assets: bool,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            reveal_hints: false,
            reveal_solution: false,
            enable_playground: true,
            playground_url: "https://play.rust-lang.org".to_string(),
            enable_progress: true,
            enabled: true,
            manage_assets: false,
        }
    }
}

/// Render an exercise to HTML.
pub fn render_exercise(parsed: &ParsedExercise) -> Result<String, RenderError> {
    render_exercise_with_config(parsed, &RenderConfig::default())
}

/// Render an exercise to HTML with custom configuration.
pub fn render_exercise_with_config(
    parsed: &ParsedExercise,
    config: &RenderConfig,
) -> Result<String, RenderError> {
    match parsed {
        ParsedExercise::Code(exercise) => render_code_exercise(exercise, config),
        ParsedExercise::UseCase(exercise) => render_usecase_exercise(exercise, config),
    }
}

// --- Code Exercise Renderer ---

fn render_code_exercise(
    exercise: &Exercise,
    config: &RenderConfig,
) -> Result<String, RenderError> {
    let mut html = String::new();

    html.push_str(&format!(
        r#"<article class="exercise" data-exercise-id="{}" data-difficulty="{}">"#,
        escape_html(&exercise.metadata.id),
        exercise.metadata.difficulty
    ));
    html.push('\n');

    html.push_str(&render_code_header(exercise));
    html.push_str(&render_code_navigation(exercise));

    if !exercise.description.is_empty() {
        html.push_str(&render_description(&exercise.description, &exercise.metadata.id));
    }

    if let Some(objectives) = &exercise.objectives {
        html.push_str(&render_objectives(objectives, &exercise.metadata.id));
    }

    if let Some(discussion) = &exercise.discussion {
        html.push_str(&render_discussion(discussion));
    }

    if let Some(starter) = &exercise.starter {
        html.push_str(&render_starter(starter, &exercise.metadata.id));
    }

    if !exercise.hints.is_empty() {
        html.push_str(&render_hints(&exercise.hints, config.reveal_hints, &exercise.metadata.id));
    }

    if let Some(solution) = &exercise.solution {
        html.push_str(&render_solution(solution, config.reveal_solution, &exercise.metadata.id));
    }

    if let Some(tests) = &exercise.tests {
        html.push_str(&render_tests(tests, &exercise.metadata.id, config));
    }

    if let Some(reflection) = &exercise.reflection {
        html.push_str(&render_reflection(reflection, &exercise.metadata.id));
    }

    if config.enable_progress {
        html.push_str(&render_footer(&exercise.metadata.id));
    }

    html.push_str("</article>\n");
    Ok(html)
}

// --- UseCase Exercise Renderer ---

fn render_usecase_exercise(
    exercise: &UseCaseExercise,
    config: &RenderConfig,
) -> Result<String, RenderError> {
    let mut html = String::new();
    let id = &exercise.metadata.id;

    html.push_str(&format!(
        r#"<article class="usecase-exercise" data-exercise-id="{}" data-domain="{}" data-difficulty="{}">"#,
        escape_html(id),
        exercise.metadata.domain,
        exercise.metadata.difficulty
    ));
    html.push('\n');

    html.push_str(&render_usecase_header(exercise));

    if !exercise.description.is_empty() {
        html.push_str(&render_description(&exercise.description, id));
    }

    if let Some(objectives) = &exercise.objectives {
        html.push_str(&render_objectives(objectives, id));
    }

    // Scenario
    html.push_str(&render_scenario(&exercise.scenario, id));

    // Prompt
    html.push_str(&render_prompt(&exercise.prompt, id));

    // Hints
    if !exercise.hints.is_empty() {
        html.push_str(&render_hints(&exercise.hints, config.reveal_hints, id));
    }

    // Response Area
    html.push_str(&render_response_area(&exercise.evaluation, id));

    // Evaluation Results (hidden initially)
    html.push_str(&render_evaluation_placeholder(id));

    // Context (hidden initially)
    if let Some(context) = &exercise.context {
        html.push_str(&render_context(context, id));
    }

    if config.enable_progress {
        html.push_str(&render_footer(id));
    }

    html.push_str("</article>\n");
    Ok(html)
}

// --- Shared Components ---

fn render_description(description: &str, exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(&format!(
        r#"<section class="exercise-description" id="{}-description">"#,
        exercise_id
    ));
    html.push('\n');
    let parser = Parser::new(description);
    let mut description_html = String::new();
    html::push_html(&mut description_html, parser);
    html.push_str(&description_html);
    html.push_str("</section>\n");
    html
}

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

// --- Code Exercise Specific Components ---

fn render_code_header(exercise: &Exercise) -> String {
    let mut html = String::new();
    html.push_str(r#"<header class="exercise-header">"#);
    html.push('\n');
    if let Some(title) = &exercise.title {
        html.push_str(&format!(r#"  <h2 class="exercise-title">{}</h2>"#, escape_html(title)));
        html.push('\n');
    }
    html.push_str(&format!(r#"  <code class=\"exercise-id\">{}</code>"#, escape_html(&exercise.metadata.id)));
    html.push('\n');
    html.push_str(r#"  <div class="exercise-meta">"#);
    html.push('\n');
    
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

    if let Some(minutes) = exercise.metadata.time_minutes {
        let time_str = if minutes >= 60 {
            format!("{}h {}m", minutes / 60, minutes % 60)
        } else {
            format!("{} min", minutes)
        };
        html.push_str(&format!(r#"    <span class="badge time">⏱️ {}</span>"#, time_str));
        html.push('\n');
    }

    if !exercise.metadata.prerequisites.is_empty() {
        let prereqs: Vec<String> = exercise
            .metadata
            .prerequisites
            .iter()
            .map(|p| format!("<a href=\"#{}\">{}</a>", escape_html(p), escape_html(p)))
            .collect();
        html.push_str(&format!(r#"    <span class="badge prerequisites">📚 Requires: {}</span>"#, prereqs.join(", ")));
        html.push('\n');
    }
    html.push_str("  </div>\n");
    html.push_str("</header>\n");
    html
}

fn render_code_navigation(exercise: &Exercise) -> String {
    let mut html = String::new();
    let id = &exercise.metadata.id;
    html.push_str(r#"<nav class="exercise-nav" aria-label="Exercise sections"><ul>"#);
    if !exercise.description.is_empty() {
        html.push_str(&format!(r##"<li><a href="#{}-description" data-section="description">📖 Overview</a></li>"##, id));
    }
    if exercise.objectives.is_some() {
        html.push_str(&format!(r##"<li><a href="#{}-objectives" data-section="objectives">🎯 Objectives</a></li>"##, id));
    }
    if exercise.starter.is_some() {
        html.push_str(&format!(r##"<li><a href="#{}-starter" data-section="starter">💻 Code</a></li>"##, id));
    }
    if !exercise.hints.is_empty() {
        html.push_str(&format!(r##"<li><a href="#{}-hints" data-section="hints">💡 Hints</a></li>"##, id));
    }
    if exercise.solution.is_some() {
        html.push_str(&format!(r##"<li><a href="#{}-solution" data-section="solution">✅ Solution</a></li>"##, id));
    }
    if exercise.tests.is_some() {
        html.push_str(&format!(r##"<li><a href="#{}-tests" data-section="tests">🧪 Tests</a></li>"##, id));
    }
    if exercise.reflection.is_some() {
        html.push_str(&format!(r##"<li><a href="#{}-reflection" data-section="reflection">🤔 Reflect</a></li>"##, id));
    }
    html.push_str("</ul></nav>\n");
    html
}

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

fn render_starter(starter: &StarterCode, exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(&format!(r#"<section class="exercise-starter" id="{}-starter">"#, exercise_id));
    html.push('\n');
    html.push_str(r#"  <div class="code-header">"#);
    html.push('\n');
    if let Some(filename) = &starter.filename {
        html.push_str(&format!(r#"    <span class="filename">{}</span>"#, escape_html(filename)));
        html.push('\n');
    }
    html.push_str(r#"    <div class="code-actions">"#);
    html.push('\n');
    html.push_str(&format!(r#"      <button class="btn btn-copy" data-target="code-{}" title="Copy code">📋 Copy</button>"#, exercise_id));
    html.push('\n');
    html.push_str(&format!(r#"      <button class="btn btn-reset" data-target="code-{}" title="Reset to original">↺ Reset</button>"#, exercise_id));
    html.push('\n');
    html.push_str("    </div>\n");
    html.push_str("  </div>\n");
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

fn render_solution(solution: &Solution, reveal: bool, exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(&format!(r#"<section class="exercise-solution" id="{}-solution">"#, exercise_id));
    html.push('\n');
    let should_open = match solution.reveal {
        SolutionReveal::Always => true,
        SolutionReveal::Never => false,
        SolutionReveal::OnDemand => reveal,
    };
    let open_attr = if should_open { " open" } else { "" };
    html.push_str(&format!(r#"  <details class="solution"{}>"#, open_attr));
    html.push('\n');
    html.push_str(r#"    <summary><span class="solution-warning">⚠️ Try the exercise first!</span><span class="solution-toggle">Show Solution</span></summary>"#);
    html.push('\n');
    html.push_str(r#"    <div class="solution-content">"#);
    html.push('\n');
    html.push_str(&format!(r#"      <pre><code class="language-{}">{}</code></pre>"#, escape_html(&solution.language), escape_html(&solution.code)));
    html.push('\n');
    if let Some(explanation) = &solution.explanation {
        html.push_str(r#"      <div class="solution-explanation"><h4>Explanation</h4>"#);
        let parser = Parser::new(explanation);
        let mut explanation_html = String::new();
        html::push_html(&mut explanation_html, parser);
        html.push_str(&explanation_html);
        html.push_str("</div>\n");
    }
    html.push_str("    </div>\n  </details>\n</section>\n");
    html
}

fn render_tests(tests: &TestBlock, exercise_id: &str, config: &RenderConfig) -> String {
    let mut html = String::new();
    html.push_str(&format!(r#"<section class="exercise-tests" id="{}-tests" data-mode="{}">"#, exercise_id, tests.mode));
    html.push('\n');
    html.push_str("  <h3>🧪 Tests</h3>\n");
    html.push_str(r#"  <div class="test-actions">"#);
    html.push('\n');
    if tests.mode == TestMode::Playground && config.enable_playground {
        html.push_str(&format!(
            r#"    <button class="btn btn-run-tests" data-exercise-id="{}" data-playground-url="{}">▶ Run Tests</button>"#,
            exercise_id, escape_html(&config.playground_url)
        ));
        html.push('\n');
    } else {
        html.push_str(r#"    <div class="local-test-info"><p>Run these tests locally with:</p><pre><code>cargo test</code></pre></div>"#);
        html.push('\n');
    }
    html.push_str("  </div>\n");
    html.push_str(&format!(r#"  <div class="test-results" id="results-{}" hidden></div>"#, exercise_id));
    html.push('\n');
    html.push_str(r#"  <details class="tests-code"><summary>View Test Code</summary>"#);
    html.push('\n');
    html.push_str(&format!(r#"    <pre><code class="language-{}">{}</code></pre>"#, escape_html(&tests.language), escape_html(&tests.code)));
    html.push('\n');
    html.push_str("  </details>\n</section>\n");
    html
}

fn render_reflection(reflection: &[String], exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(&format!(r#"<section class="exercise-reflection" id="{}-reflection">"#, exercise_id));
    html.push('\n');
    html.push_str("  <h3>🤔 Reflection</h3>\n");
    html.push_str("  <ul>\n");
    for item in reflection {
        html.push_str(&format!("    <li>{}</li>\n", escape_html(item)));
    }
    html.push_str("  </ul>\n</section>\n");
    html
}

// --- UseCase Exercise Specific Components ---

fn render_usecase_header(exercise: &UseCaseExercise) -> String {
    let mut html = String::new();
    html.push_str(r#"<header class="exercise-header">"#);
    html.push('\n');

    if let Some(title) = &exercise.title {
        html.push_str(&format!(r#"  <h2 class="exercise-title">{}</h2>"#, escape_html(title)));
        html.push('\n');
    }

    html.push_str(r#"  <div class="exercise-meta">"#);
    html.push('\n');
    
    // Domain badge
    let domain_class = exercise.metadata.domain.to_string();
    html.push_str(&format!(
        r#"    <span class="badge domain {}">{}</span>"#,
        domain_class, exercise.metadata.domain
    ));
    html.push('\n');

    // Difficulty
    let difficulty_class = match exercise.metadata.difficulty {
        Difficulty::Beginner => "beginner",
        Difficulty::Intermediate => "intermediate",
        Difficulty::Advanced => "advanced",
    };
    html.push_str(&format!(
        r#"    <span class="badge difficulty {}">{}</span>"#,
        difficulty_class, exercise.metadata.difficulty
    ));
    html.push('\n');

    if let Some(minutes) = exercise.metadata.time_minutes {
        html.push_str(&format!(r#"    <span class="badge time">{} min</span>"#, minutes));
        html.push('\n');
    }
    html.push_str("  </div>\n</header>\n");
    html
}

fn render_scenario(scenario: &Scenario, _exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(r#"<section class="exercise-scenario">"#);
    html.push('\n');
    html.push_str("  <h3>Scenario</h3>\n");

    // Scenario header with organization
    if let Some(org) = &scenario.organization {
        html.push_str(r#"  <div class="scenario-header">"#);
        html.push_str(&format!(r#"    <span class="organization">{}</span>"#, escape_html(org)));
        html.push_str("  </div>\n");
    }

    // Scenario content
    html.push_str(r#"  <div class="scenario-content">"#);
    let parser = Parser::new(&scenario.content);
    let mut content_html = String::new();
    html::push_html(&mut content_html, parser);
    html.push_str(&content_html);
    html.push_str("  </div>\n");

    // Constraints
    if !scenario.constraints.is_empty() {
        html.push_str(r#"  <div class="scenario-constraints">"#);
        html.push('\n');
        html.push_str("    <h4>Constraints</h4>\n");
        html.push_str("    <ul>\n");
        for c in &scenario.constraints {
            html.push_str(&format!("      <li>{}</li>\n", escape_html(c)));
        }
        html.push_str("    </ul>\n");
        html.push_str("  </div>\n");
    }

    html.push_str("</section>\n");
    html
}

fn render_prompt(prompt: &UseCasePrompt, _exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(r#"<section class="exercise-prompt">"#);
    html.push('\n');
    html.push_str("  <h3>Your Task</h3>\n");
    
    html.push_str(r#"  <div class="prompt-content">"#);
    let parser = Parser::new(&prompt.prompt);
    let mut prompt_html = String::new();
    html::push_html(&mut prompt_html, parser);
    html.push_str(&prompt_html);
    html.push_str("  </div>\n");
    
    if !prompt.aspects.is_empty() {
        html.push_str(r#"  <div class="prompt-aspects">"#);
        html.push('\n');
        html.push_str("    <h4>Address these aspects:</h4>\n");
        html.push_str("    <ul>\n");
        for aspect in &prompt.aspects {
            html.push_str(&format!("      <li>{}</li>\n", escape_html(aspect)));
        }
        html.push_str("    </ul>\n");
        html.push_str("  </div>\n");
    }
    
    html.push_str("</section>\n");
    html
}

fn render_response_area(eval: &EvaluationCriteria, exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(r#"<section class="exercise-response">"#);
    html.push('\n');
    html.push_str("  <h3>Your Response</h3>\n");
    
    // Requirements display
    let min_words = eval.min_words.unwrap_or(0);
    let max_words = eval.max_words.unwrap_or(10000);
    
    let range_display = if eval.max_words.is_some() {
        format!("{}-{} words", min_words, max_words)
    } else if eval.min_words.is_some() {
        format!("{}+ words", min_words)
    } else {
        "No word count limit".to_string()
    };

    html.push_str(r#"  <div class="response-requirements">"#);
    html.push_str(&format!(r#"    <span class="word-count-req">{}</span>"#, range_display));
    html.push_str("  </div>\n");

    // Textarea
    html.push_str(&format!(
        r#"  <textarea class="response-editor" id="response-{}" placeholder="Enter your analysis here..." data-min-words="{}" data-max-words="{}"></textarea>"#,
        exercise_id, min_words, max_words
    ));
    html.push('\n');

    html.push_str(r#"  <div class="response-meta">"#);
    html.push_str(r#"    <span class="current-word-count">0 words</span>"#);
    html.push_str("  </div>\n");

    html.push_str(r#"  <div class="response-actions">"#);
    html.push_str(&format!(
        r#"    <button class="btn-submit" data-exercise-id="{}">Submit for Evaluation</button>"#,
        exercise_id
    ));
    html.push_str("  </div>\n");
    
    html.push_str("</section>\n");
    html
}

fn render_evaluation_placeholder(exercise_id: &str) -> String {
    format!(
        r#"<section class="exercise-evaluation" id="evaluation-{}" hidden><h3>Evaluation Results</h3><div class="overall-score"></div><div class="criterion-scores"></div><div class="feedback"></div></section>"#,
        exercise_id
    )
}

fn render_context(context: &str, exercise_id: &str) -> String {
    let mut html = String::new();
    html.push_str(&format!(r#"<section class="exercise-context" id="context-{}" hidden>"#, exercise_id));
    html.push('\n');
    html.push_str("  <h3>Key Learning Points</h3>\n");
    html.push_str(r#"  <div class="context-content">"#);
    let parser = Parser::new(context);
    let mut context_html = String::new();
    html::push_html(&mut context_html, parser);
    html.push_str(&context_html);
    html.push_str("  </div>\n");
    html.push_str("</section>\n");
    html
}

// --- Utils ---

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

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
    fn test_render_simple_code_exercise() {
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

        let parsed = ParsedExercise::Code(exercise);
        let html = render_exercise(&parsed).unwrap();

        assert!(html.contains(r#"data-exercise-id="test-exercise""#));
        assert!(html.contains("Test Exercise"));
        assert!(html.contains("beginner"));
    }
    
    #[test]
    fn test_render_usecase_exercise() {
        let exercise = UseCaseExercise {
            metadata: UseCaseMetadata {
                id: "uc-001".to_string(),
                difficulty: Difficulty::Intermediate,
                domain: UseCaseDomain::Healthcare,
                ..Default::default()
            },
            title: Some("HIPAA Analysis".to_string()),
            scenario: Scenario {
                organization: Some("Hospital".to_string()),
                content: "Scenario text".to_string(),
                ..Default::default()
            },
            prompt: UseCasePrompt {
                prompt: "Analyze it".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };
        
        let parsed = ParsedExercise::UseCase(exercise);
        let html = render_exercise(&parsed).unwrap();
        
        assert!(html.contains("usecase-exercise"));
        assert!(html.contains("data-domain=\"healthcare\""));
        assert!(html.contains("HIPAA Analysis"));
        assert!(html.contains("Hospital"));
    }
}
