//! Parser for exercise markdown with directive blocks.
//!
//! This module handles parsing markdown files that contain exercise directives
//! like `::: exercise`, `::: hint`, `::: solution`, etc.

use crate::types::*;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::collections::HashMap;
use std::ops::Range;
use thiserror::Error;

/// Errors that can occur during parsing.
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Missing required field '{field}' in {block} block")]
    MissingField { block: String, field: String },

    #[error("Invalid attribute value '{value}' for '{attribute}'")]
    InvalidAttribute { attribute: String, value: String },

    #[error("Unclosed directive block '{block}' starting at line {line}")]
    UnclosedBlock { block: String, line: usize },

    #[error("Duplicate block type '{block_type}' (only one allowed)")]
    DuplicateBlock { block_type: String },

    #[error("YAML parse error in {block} block: {source}")]
    YamlError {
        block: String,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Invalid hint level: {0}")]
    InvalidHintLevel(String),
}

/// Result type for parsing operations.
pub type ParseResult<T> = Result<T, ParseError>;

/// A parsed directive with its type and attributes.
#[derive(Debug)]
struct Directive {
    /// The directive name (e.g., "exercise", "hint", "solution")
    name: String,

    /// Inline attributes (e.g., level=1, file="src/main.rs")
    attributes: HashMap<String, String>,

    /// The line number where this directive started
    line: usize,
}

/// Parse a markdown file containing exercise directives.
///
/// # Arguments
///
/// * `markdown` - The markdown content to parse
///
/// # Returns
///
/// A fully parsed `Exercise` struct, or an error if parsing fails.
///
/// # Example
///
/// ```rust
/// use mdbook_exercises::parse_exercise;
///
/// let markdown = r#"
/// # My Exercise
///
/// ::: exercise
/// id: my-exercise
/// difficulty: beginner
/// :::
///
/// Some description here.
/// "#;
///
/// let exercise = parse_exercise(markdown).unwrap();
/// assert_eq!(exercise.metadata.id, "my-exercise");
/// ```
pub fn parse_exercise(markdown: &str) -> ParseResult<Exercise> {
    let mut exercise = Exercise::default();
    let mut current_directive: Option<Directive> = None;
    let mut block_content = String::new();
    let mut description_buffer = String::new();
    let mut in_description = true;

    // Identify ranges that should be ignored (inside code blocks, etc.)
    let excluded_ranges = find_excluded_ranges(markdown);

    let mut current_offset = 0;
    // We use split_inclusive to track offsets correctly, but we need to handle the line content carefully
    for (line_num, line_raw) in markdown.split_inclusive('\n').enumerate() {
        let line_number = line_num + 1; // 1-indexed for error messages
        let line_len = line_raw.len();
        let line_range = current_offset..(current_offset + line_len);

        // Remove trailing newline for processing
        let line = line_raw.trim_end_matches(['\n', '\r']);

        // Check if this line is inside an excluded range (code block)
        // We check if the *start* of the line (plus indentation) is excluded.
        // A directive must start with :::, so we care if the ::: token is excluded.
        // But simply checking if the line overlaps with an excluded range is usually sufficient
        // for block-level exclusions.
        let is_excluded = is_range_excluded(&line_range, &excluded_ranges);

        // Update offset for next iteration
        current_offset += line_len;

        // Check for directive start, but ONLY if not excluded
        if !is_excluded {
            if let Some(directive) = parse_directive_start(line, line_number) {
                // Finish previous block
                if let Some(prev_directive) = current_directive.take() {
                    process_block(&mut exercise, &prev_directive, &block_content)?;
                } else if in_description && directive.name != "exercise" {
                    // We were collecting description - save it when we hit a content directive
                    // (but not when we hit the exercise metadata block itself)
                    exercise.description = description_buffer.trim().to_string();
                    in_description = false;
                }

                current_directive = Some(directive);
                block_content.clear();
                continue;
            }

            // Check for directive end
            if line.trim() == ":::" {
                if let Some(directive) = current_directive.take() {
                    process_block(&mut exercise, &directive, &block_content)?;
                    block_content.clear();
                }
                continue;
            }
        }

        // Collect content
        if current_directive.is_some() {
            block_content.push_str(line_raw); // Push original line with newline
        } else if in_description {
            // Extract title from first heading
            if exercise.title.is_none() && line.starts_with('#') && !is_excluded {
                let title = line.trim_start_matches('#').trim();
                if !title.is_empty() {
                    exercise.title = Some(title.to_string());
                    continue;
                }
            }
            description_buffer.push_str(line_raw); // Push original line with newline
        }
    }

    // Handle unclosed directive
    if let Some(directive) = current_directive {
        return Err(ParseError::UnclosedBlock {
            block: directive.name,
            line: directive.line,
        });
    }

    // Finalize description if we never hit a directive
    if in_description && !description_buffer.is_empty() {
        exercise.description = description_buffer.trim().to_string();
    }

    Ok(exercise)
}

/// Find ranges in the markdown that should be excluded from directive parsing
/// (e.g., inside code blocks, HTML blocks).
fn find_excluded_ranges(markdown: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let parser = Parser::new(markdown).into_offset_iter();

    // We need to capture the full range of code blocks.
    // pulldown-cmark emits Start(CodeBlock), Text/Code/etc., End(CodeBlock).
    // We want to treat everything between Start and End as excluded.

    let mut block_start: Option<usize> = None;

    for (event, range) in parser {
        match event {
            Event::Start(Tag::CodeBlock(_)) | Event::Start(Tag::HtmlBlock) => {
                if block_start.is_none() {
                    block_start = Some(range.start);
                }
            }
            Event::End(TagEnd::CodeBlock) | Event::End(TagEnd::HtmlBlock) => {
                if let Some(start) = block_start {
                    ranges.push(start..range.end);
                    block_start = None;
                }
            }
            Event::Code(_) | Event::Html(_) => {
                // Inline code or inline HTML - exclude these ranges too
                // But only if we are not already inside a block (though pulldown-cmark
                // usually handles nesting by flattening or erroring, we just take the range)
                if block_start.is_none() {
                    ranges.push(range);
                }
            }
            _ => {}
        }
    }

    ranges
}

/// Check if a line range overlaps significantly with any excluded range.
fn is_range_excluded(line_range: &Range<usize>, excluded: &[Range<usize>]) -> bool {
    // We consider a line excluded if its first non-whitespace character (approx)
    // falls within an excluded range.
    // Since we don't scan for non-whitespace here easily, we check if the start
    // of the line is in an excluded range.
    // However, the `line_range.start` is the beginning of the line.
    // If a code block starts mid-line? (Not possible for CodeBlock, but possible for Code/Html).
    // For Directives, we only care if the line START (where ::: would be) is excluded.

    for range in excluded {
        if range.contains(&line_range.start) {
            return true;
        }
        // Also check if the line is fully inside the range (e.g. range starts before line and ends after line)
        if range.start <= line_range.start && range.end >= line_range.end {
            return true;
        }
    }
    false
}

/// Parse the opening line of a directive.
fn parse_directive_start(line: &str, line_number: usize) -> Option<Directive> {
    let trimmed = line.trim();

    if !trimmed.starts_with(":::") {
        return None;
    }

    let rest = trimmed[3..].trim();

    if rest.is_empty() || rest.starts_with(":::") {
        return None; // This is a closing ::: or ::::
    }

    // Split into name and attributes
    let mut parts = rest.splitn(2, |c: char| c.is_whitespace());
    let name = parts.next()?.to_string();
    let attrs_str = parts.next().unwrap_or("");

    let attributes = parse_inline_attributes(attrs_str);

    Some(Directive {
        name,
        attributes,
        line: line_number,
    })
}

/// Parse inline attributes like `level=1 file="src/main.rs"`
fn parse_inline_attributes(attrs_str: &str) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let mut remaining = attrs_str.trim();

    while !remaining.is_empty() {
        // Skip whitespace
        remaining = remaining.trim_start();

        if remaining.is_empty() {
            break;
        }

        // Find the key
        let key_end = remaining
            .find(|c: char| c == '=' || c.is_whitespace())
            .unwrap_or(remaining.len());

        let key = &remaining[..key_end];
        remaining = &remaining[key_end..];

        if remaining.starts_with('=') {
            remaining = &remaining[1..];

            // Parse the value
            let value = if remaining.starts_with('"') {
                // Quoted value
                remaining = &remaining[1..];
                let end = remaining.find('"').unwrap_or(remaining.len());
                let val = &remaining[..end];
                remaining = &remaining[(end + 1).min(remaining.len())..];
                val.to_string()
            } else {
                // Unquoted value (until whitespace)
                let end = remaining
                    .find(char::is_whitespace)
                    .unwrap_or(remaining.len());
                let val = &remaining[..end];
                remaining = &remaining[end..];
                val.to_string()
            };

            attrs.insert(key.to_string(), value);
        } else {
            // Boolean flag (no value)
            attrs.insert(key.to_string(), "true".to_string());
        }
    }

    attrs
}

/// Parse a code fence info string like "rust,filename=src/main.rs,ignore" into
/// (language, attribute map). The first non key=value token is treated as language.
fn parse_fence_info(info: &str) -> (String, HashMap<String, String>) {
    let mut lang = String::new();
    let mut attrs = HashMap::new();
    for (i, raw) in info.split(',').enumerate() {
        let token = raw.trim();
        if token.is_empty() {
            continue;
        }
        if i == 0 && !token.contains('=') {
            lang = token.to_string();
            continue;
        }
        if let Some(eq) = token.find('=') {
            let (k, v) = token.split_at(eq);
            attrs.insert(k.trim().to_string(), v[1..].trim().to_string());
        } else {
            // boolean flag
            attrs.insert(token.to_string(), "true".to_string());
        }
    }
    (lang, attrs)
}

/// Process a completed directive block.
fn process_block(exercise: &mut Exercise, directive: &Directive, content: &str) -> ParseResult<()> {
    match directive.name.as_str() {
        "exercise" => parse_exercise_block(exercise, content)?,
        "objectives" => parse_objectives_block(exercise, content)?,
        "discussion" => parse_discussion_block(exercise, content)?,
        "starter" => parse_starter_block(exercise, &directive.attributes, content)?,
        "hint" => parse_hint_block(exercise, &directive.attributes, content)?,
        "solution" => parse_solution_block(exercise, &directive.attributes, content)?,
        "tests" => parse_tests_block(exercise, &directive.attributes, content)?,
        "reflection" => parse_reflection_block(exercise, content)?,
        _ => {
            // Unknown directive - ignore silently for forward compatibility
        }
    }

    Ok(())
}

/// Parse the exercise metadata block.
fn parse_exercise_block(exercise: &mut Exercise, content: &str) -> ParseResult<()> {
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(content).map_err(|e| ParseError::YamlError {
            block: "exercise".to_string(),
            source: e,
        })?;

    if let Some(id) = yaml.get("id").and_then(|v| v.as_str()) {
        exercise.metadata.id = id.to_string();
    } else {
        return Err(ParseError::MissingField {
            block: "exercise".to_string(),
            field: "id".to_string(),
        });
    }

    if let Some(difficulty) = yaml.get("difficulty").and_then(|v| v.as_str()) {
        exercise.metadata.difficulty =
            difficulty
                .parse()
                .map_err(|_| ParseError::InvalidAttribute {
                    attribute: "difficulty".to_string(),
                    value: difficulty.to_string(),
                })?;
    }

    if let Some(time_value) = yaml.get("time") {
        // Handle both string ("30 minutes") and integer (30) formats
        if let Some(time_str) = time_value.as_str() {
            exercise.metadata.time_minutes = parse_time_string(time_str);
        } else if let Some(time_int) = time_value.as_u64() {
            exercise.metadata.time_minutes = Some(time_int as u32);
        }
    }

    if let Some(prereqs) = yaml.get("prerequisites") {
        if let Some(arr) = prereqs.as_sequence() {
            exercise.metadata.prerequisites = arr
                .iter()
                .filter_map(|v| v.as_str())
                .map(String::from)
                .collect();
        }
    }

    Ok(())
}

/// Parse a time string like "20 minutes" into minutes.
fn parse_time_string(time: &str) -> Option<u32> {
    let parts: Vec<&str> = time.split_whitespace().collect();
    if parts.is_empty() {
        return None;
    }

    let number: u32 = parts[0].parse().ok()?;

    if parts.len() > 1 {
        let unit = parts[1].to_lowercase();
        if unit.starts_with("hour") {
            return Some(number * 60);
        }
    }

    Some(number)
}

/// Parse the objectives block.
fn parse_objectives_block(exercise: &mut Exercise, content: &str) -> ParseResult<()> {
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(content).map_err(|e| ParseError::YamlError {
            block: "objectives".to_string(),
            source: e,
        })?;

    let mut objectives = Objectives::default();

    if let Some(thinking) = yaml.get("thinking").and_then(|v| v.as_sequence()) {
        objectives.thinking = thinking
            .iter()
            .filter_map(|v| v.as_str())
            .map(String::from)
            .collect();
    }

    if let Some(doing) = yaml.get("doing").and_then(|v| v.as_sequence()) {
        objectives.doing = doing
            .iter()
            .filter_map(|v| v.as_str())
            .map(String::from)
            .collect();
    }

    exercise.objectives = Some(objectives);
    Ok(())
}

/// Parse the discussion block.
fn parse_discussion_block(exercise: &mut Exercise, content: &str) -> ParseResult<()> {
    let items = parse_markdown_list(content);
    if !items.is_empty() {
        exercise.discussion = Some(items);
    }
    Ok(())
}

/// Parse the starter code block.
fn parse_starter_block(
    exercise: &mut Exercise,
    attrs: &HashMap<String, String>,
    content: &str,
) -> ParseResult<()> {
    let (language_raw, code) = extract_code_block(content);

    if code.trim().is_empty() {
        let id = if !exercise.metadata.id.is_empty() { &exercise.metadata.id } else { "<unknown-id>" };
        eprintln!(
            "[WARN] (mdbook-exercises): Starter block has no fenced code; ignored for exercise '{}'",
            id
        );
        return Ok(());
    }

    // Pull filename and language from directive attrs and/or fence info
    let mut filename = attrs.get("file").cloned();
    let mut language = attrs.get("language").cloned();

    // Prefer info from extract_code_block; fallback to scanning first fence line
    let mut info_opt = language_raw;
    if info_opt.is_none() {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("```") {
                let info = t.trim_start_matches('`').trim().to_string();
                if !info.is_empty() { info_opt = Some(info); }
                break;
            }
        }
    }

    if let Some(info) = info_opt {
        let (lang_clean, fence_attrs) = parse_fence_info(&info);
        if language.is_none() && !lang_clean.is_empty() {
            language = Some(lang_clean);
        }
        if filename.is_none() {
            if let Some(f) = fence_attrs.get("filename") {
                filename = Some(f.clone());
            } else if let Some(f) = fence_attrs.get("file") {
                filename = Some(f.clone());
            }
        }
    }

    exercise.starter = Some(StarterCode {
        filename,
        language: language.unwrap_or_else(|| "rust".to_string()),
        code,
    });

    Ok(())
}

/// Parse a hint block.
fn parse_hint_block(
    exercise: &mut Exercise,
    attrs: &HashMap<String, String>,
    content: &str,
) -> ParseResult<()> {
    let level = attrs
        .get("level")
        .ok_or_else(|| ParseError::MissingField {
            block: "hint".to_string(),
            field: "level".to_string(),
        })?
        .parse::<u8>()
        .map_err(|_| ParseError::InvalidHintLevel(attrs.get("level").unwrap().clone()))?;

    let title = attrs.get("title").cloned();

    exercise.hints.push(Hint {
        level,
        title,
        content: content.trim().to_string(),
    });

    // Keep hints sorted by level
    exercise.hints.sort_by_key(|h| h.level);

    Ok(())
}

/// Parse the solution block.
fn parse_solution_block(exercise: &mut Exercise, attrs: &HashMap<String, String>, content: &str) -> ParseResult<()> {
    // Split content into code and explanation
    let (language, code) = extract_code_block(content);

    // Look for explanation after the code block
    let explanation = extract_explanation(content);

    let mut sol = Solution {
        code,
        language: language.unwrap_or_else(|| "rust".to_string()),
        explanation,
        ..Default::default()
    };

    if sol.code.trim().is_empty() {
        let id = if !exercise.metadata.id.is_empty() { &exercise.metadata.id } else { "<unknown-id>" };
        eprintln!(
            "[WARN] (mdbook-exercises): Solution block has no fenced code; ignored for exercise '{}'",
            id
        );
        return Ok(());
    }

    if let Some(reveal) = attrs.get("reveal").map(|s| s.to_lowercase()) {
        sol.reveal = match reveal.as_str() {
            "always" => SolutionReveal::Always,
            "never" => SolutionReveal::Never,
            _ => SolutionReveal::OnDemand,
        };
    }

    exercise.solution = Some(sol);

    Ok(())
}

/// Parse the tests block.
fn parse_tests_block(
    exercise: &mut Exercise,
    attrs: &HashMap<String, String>,
    content: &str,
) -> ParseResult<()> {
    let (language_raw, code) = extract_code_block(content);

    // If the tests block has no code content, ignore it so no empty
    // "Tests" section is rendered.
    if code.trim().is_empty() {
        // Emit a helpful warning during builds so authors notice.
        let id = if !exercise.metadata.id.is_empty() { exercise.metadata.id.clone() } else { "<unknown-id>".to_string() };
        eprintln!(
            "[WARN] (mdbook-exercises): Empty tests block ignored for exercise '{}'",
            id
        );
        return Ok(());
    }

    let mode = attrs
        .get("mode")
        .map(|m| m.parse().unwrap_or(TestMode::Playground))
        .unwrap_or(TestMode::Playground);

    let mut language = attrs.get("language").cloned();
    if let Some(info) = language_raw {
        let (lang_clean, _fa) = parse_fence_info(&info);
        if language.is_none() && !lang_clean.is_empty() {
            language = Some(lang_clean);
        }
    }

    exercise.tests = Some(TestBlock { language: language.unwrap_or_else(|| "rust".to_string()), code, mode });

    Ok(())
}

/// Parse the reflection block.
fn parse_reflection_block(exercise: &mut Exercise, content: &str) -> ParseResult<()> {
    let items = parse_markdown_list(content);
    if !items.is_empty() {
        exercise.reflection = Some(items);
    }
    Ok(())
}

/// Extract a code block from content, returning (language, code).
fn extract_code_block(content: &str) -> (Option<String>, String) {
    let lines: Vec<&str> = content.lines().collect();

    let mut in_code_block = false;
    let mut language = None;
    let mut code_lines = Vec::new();

    for line in lines {
        if line.trim().starts_with("```") {
            if in_code_block {
                // End of code block
                break;
            } else {
                // Start of code block
                in_code_block = true;
                let lang = line.trim().trim_start_matches('`').trim();
                if !lang.is_empty() {
                    language = Some(lang.to_string());
                }
            }
        } else if in_code_block {
            code_lines.push(line);
        }
    }

    (language, code_lines.join("\n"))
}

/// Extract explanation section from content (after code block).
fn extract_explanation(content: &str) -> Option<String> {
    let mut in_code_block = false;
    let mut found_code_block = false;
    let mut explanation_lines = Vec::new();

    for line in content.lines() {
        if line.trim().starts_with("```") {
            if in_code_block {
                in_code_block = false;
                found_code_block = true;
            } else {
                in_code_block = true;
            }
        } else if found_code_block && !in_code_block {
            explanation_lines.push(line);
        }
    }

    let explanation = explanation_lines.join("\n").trim().to_string();

    // Remove the "### Explanation" heading if present
    let explanation = explanation
        .strip_prefix("### Explanation")
        .unwrap_or(&explanation)
        .trim()
        .to_string();

    if explanation.is_empty() {
        None
    } else {
        Some(explanation)
    }
}

/// Parse a markdown list into items.
fn parse_markdown_list(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('-') || trimmed.starts_with('*') {
                Some(trimmed[1..].trim().to_string())
            } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed.contains('.') {
                // Numbered list: "1. item"
                let dot_pos = trimmed.find('.')?;
                Some(trimmed[dot_pos + 1..].trim().to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_exercise() {
        let markdown = r#"
# Hello World

::: exercise
id: hello-world
difficulty: beginner
time: 10 minutes
:::

Write a greeting function.
"#;

        let exercise = parse_exercise(markdown).unwrap();
        assert_eq!(exercise.metadata.id, "hello-world");
        assert_eq!(exercise.metadata.difficulty, Difficulty::Beginner);
        assert_eq!(exercise.metadata.time_minutes, Some(10));
        assert_eq!(exercise.title, Some("Hello World".to_string()));
    }

    #[test]
    fn test_parse_hint() {
        let markdown = r#"
::: exercise
id: test
difficulty: beginner
:::

::: hint level=1 title="First Hint"
This is hint 1.
:::

::: hint level=2
This is hint 2.
:::
"#;

        let exercise = parse_exercise(markdown).unwrap();
        assert_eq!(exercise.hints.len(), 2);
        assert_eq!(exercise.hints[0].level, 1);
        assert_eq!(exercise.hints[0].title, Some("First Hint".to_string()));
        assert_eq!(exercise.hints[1].level, 2);
    }

    #[test]
    fn test_parse_inline_attributes() {
        let attrs = parse_inline_attributes(r#"level=1 file="src/main.rs" readonly"#);
        assert_eq!(attrs.get("level"), Some(&"1".to_string()));
        assert_eq!(attrs.get("file"), Some(&"src/main.rs".to_string()));
        assert_eq!(attrs.get("readonly"), Some(&"true".to_string()));
    }

    #[test]
    fn test_ignore_code_blocks() {
        let markdown = r#"
# Test

::: exercise
id: test
difficulty: beginner
:::

Here is an example of an exercise block:

```markdown
::: exercise
id: fake
difficulty: advanced
:::
```
"#;
        let exercise = parse_exercise(markdown).unwrap();
        assert_eq!(exercise.metadata.id, "test");
        assert_eq!(exercise.metadata.difficulty, Difficulty::Beginner);
        // The second block should be ignored, so difficulty should remain Beginner
    }

    #[test]
    fn test_starter_filename_from_fence_info() {
        let markdown = r#"
::: exercise
id: fence-file-test
difficulty: beginner
:::

::: starter
```rust,filename=src/main.rs
fn main() {}
```
:::
"#;

        let exercise = parse_exercise(markdown).unwrap();
        let starter = exercise.starter.as_ref().expect("starter missing");
        assert_eq!(starter.language, "rust");
        assert_eq!(starter.filename.as_deref(), Some("src/main.rs"));
    }

    #[test]
    fn test_starter_filename_attr_precedence_over_fence() {
        let markdown = r#"
::: exercise
id: fence-vs-attr
difficulty: beginner
:::

::: starter file="src/lib.rs"
```rust,filename=src/main.rs
fn main() {}
```
:::
"#;

        let exercise = parse_exercise(markdown).unwrap();
        let starter = exercise.starter.as_ref().expect("starter missing");
        // Attribute should take precedence over fence info
        assert_eq!(starter.filename.as_deref(), Some("src/lib.rs"));
        assert_eq!(starter.language, "rust");
    }

    #[test]
    fn test_tests_language_from_fence_info() {
        let markdown = r#"
::: exercise
id: tests-lang-fence
difficulty: beginner
:::

::: tests
```rust
#[test]
fn it_works() { assert!(true); }
```
:::
"#;

        let exercise = parse_exercise(markdown).unwrap();
        let tests = exercise.tests.as_ref().expect("tests missing");
        assert_eq!(tests.language, "rust");
    }

    #[test]
    fn test_tests_language_attr_precedence_over_fence() {
        let markdown = r#"
::: exercise
id: tests-lang-attr
difficulty: beginner
:::

::: tests language=python
```rust
#[test]
fn it_works() { assert!(true); }
```
:::
"#;

        let exercise = parse_exercise(markdown).unwrap();
        let tests = exercise.tests.as_ref().expect("tests missing");
        // Attribute should override fence info
        assert_eq!(tests.language, "python");
    }
}
