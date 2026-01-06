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

    #[error("Unknown exercise type. Must contain either '::: exercise' or '::: usecase'")]
    UnknownExerciseType,
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
pub fn parse_exercise(markdown: &str) -> ParseResult<ParsedExercise> {
    // Detect exercise type based on the presence of specific directives
    // This is a simple heuristic: scan for ::: exercise vs ::: usecase
    // We ignore code blocks for this check to avoid false positives in examples

    let excluded = find_excluded_ranges(markdown);
    
    // Check for usecase directive
    if contains_directive(markdown, "usecase", &excluded) {
        return parse_usecase_exercise(markdown, excluded).map(ParsedExercise::UseCase);
    }
    
    // Check for exercise directive
    if contains_directive(markdown, "exercise", &excluded) {
        return parse_code_exercise(markdown, excluded).map(ParsedExercise::Code);
    }

    // Default to error if neither is found
    Err(ParseError::UnknownExerciseType)
}

/// Check if the markdown contains a specific directive, ignoring excluded ranges.
fn contains_directive(markdown: &str, directive: &str, excluded: &[Range<usize>]) -> bool {
    let pattern = format!("::: {}", directive);
    let mut offset = 0;
    
    for line in markdown.lines() {
        let line_len = line.len() + 1; // Approx +1 for newline
        let range = offset..(offset + line.len());
        offset += line_len;

        if line.trim().starts_with(&pattern) {
            if !is_range_excluded(&range, excluded) {
                return true;
            }
        }
    }
    false
}

/// Parse a code exercise (original format).
fn parse_code_exercise(markdown: &str, excluded_ranges: Vec<Range<usize>>) -> ParseResult<Exercise> {
    let mut exercise = Exercise::default();
    let mut current_directive: Option<Directive> = None;
    let mut block_content = String::new();
    let mut description_buffer = String::new();
    let mut in_description = true;

    let mut current_offset = 0;
    for (line_num, line_raw) in markdown.split_inclusive('\n').enumerate() {
        let line_number = line_num + 1;
        let line_len = line_raw.len();
        let line_range = current_offset..(current_offset + line_len);

        let line = line_raw.trim_end_matches(['\n', '\r']);
        let is_excluded = is_range_excluded(&line_range, &excluded_ranges);
        current_offset += line_len;

        if !is_excluded {
            if let Some(directive) = parse_directive_start(line, line_number) {
                if let Some(prev_directive) = current_directive.take() {
                    process_code_block(&mut exercise, &prev_directive, &block_content)?;
                } else if in_description && directive.name != "exercise" {
                    exercise.description = description_buffer.trim().to_string();
                    in_description = false;
                }

                current_directive = Some(directive);
                block_content.clear();
                continue;
            }

            if line.trim() == ":::" {
                if let Some(directive) = current_directive.take() {
                    process_code_block(&mut exercise, &directive, &block_content)?;
                    block_content.clear();
                }
                continue;
            }
        }

        if current_directive.is_some() {
            block_content.push_str(line_raw);
        } else if in_description {
            if exercise.title.is_none() && line.starts_with('#') && !is_excluded {
                let title = line.trim_start_matches('#').trim();
                if !title.is_empty() {
                    exercise.title = Some(title.to_string());
                    continue;
                }
            }
            description_buffer.push_str(line_raw);
        }
    }

    if let Some(directive) = current_directive {
        return Err(ParseError::UnclosedBlock {
            block: directive.name,
            line: directive.line,
        });
    }

    if in_description && !description_buffer.is_empty() {
        exercise.description = description_buffer.trim().to_string();
    }

    Ok(exercise)
}

/// Parse a UseCase exercise.
fn parse_usecase_exercise(markdown: &str, excluded_ranges: Vec<Range<usize>>) -> ParseResult<UseCaseExercise> {
    let mut exercise = UseCaseExercise::default();
    let mut current_directive: Option<Directive> = None;
    let mut block_content = String::new();
    let mut description_buffer = String::new();
    let mut in_description = true;

    let mut current_offset = 0;
    for (line_num, line_raw) in markdown.split_inclusive('\n').enumerate() {
        let line_number = line_num + 1;
        let line_len = line_raw.len();
        let line_range = current_offset..(current_offset + line_len);

        let line = line_raw.trim_end_matches(['\n', '\r']);
        let is_excluded = is_range_excluded(&line_range, &excluded_ranges);
        current_offset += line_len;

        if !is_excluded {
            if let Some(directive) = parse_directive_start(line, line_number) {
                if let Some(prev_directive) = current_directive.take() {
                    process_usecase_block(&mut exercise, &prev_directive, &block_content)?;
                } else if in_description && directive.name != "usecase" {
                    exercise.description = description_buffer.trim().to_string();
                    in_description = false;
                }

                current_directive = Some(directive);
                block_content.clear();
                continue;
            }

            if line.trim() == ":::" {
                if let Some(directive) = current_directive.take() {
                    process_usecase_block(&mut exercise, &directive, &block_content)?;
                    block_content.clear();
                }
                continue;
            }
        }

        if current_directive.is_some() {
            block_content.push_str(line_raw);
        } else if in_description {
            if exercise.title.is_none() && line.starts_with('#') && !is_excluded {
                let title = line.trim_start_matches('#').trim();
                if !title.is_empty() {
                    exercise.title = Some(title.to_string());
                    continue;
                }
            }
            description_buffer.push_str(line_raw);
        }
    }

    if let Some(directive) = current_directive {
        return Err(ParseError::UnclosedBlock {
            block: directive.name,
            line: directive.line,
        });
    }

    if in_description && !description_buffer.is_empty() {
        exercise.description = description_buffer.trim().to_string();
    }

    Ok(exercise)
}

/// Process a directive block for code exercises.
fn process_code_block(exercise: &mut Exercise, directive: &Directive, content: &str) -> ParseResult<()> {
    match directive.name.as_str() {
        "exercise" => parse_exercise_block(exercise, content)?,
        "objectives" => parse_objectives_block(&mut exercise.objectives, content)?,
        "discussion" => parse_discussion_block(exercise, content)?,
        "starter" => parse_starter_block(exercise, &directive.attributes, content)?,
        "hint" => parse_hint_block(&mut exercise.hints, &directive.attributes, content)?,
        "solution" => parse_solution_block(exercise, &directive.attributes, content)?,
        "tests" => parse_tests_block(exercise, &directive.attributes, content)?,
        "reflection" => parse_reflection_block(exercise, content)?,
        _ => {
            // Unknown directive - ignore
        }
    }
    Ok(())
}

/// Process a directive block for UseCase exercises.
fn process_usecase_block(exercise: &mut UseCaseExercise, directive: &Directive, content: &str) -> ParseResult<()> {
    match directive.name.as_str() {
        "usecase" => parse_usecase_meta_block(exercise, content)?,
        "scenario" => parse_scenario_block(exercise, &directive.attributes, content)?,
        "prompt" => parse_prompt_block(exercise, content)?,
        "evaluation" => parse_evaluation_block(exercise, content)?,
        "sample-answer" => parse_sample_answer_block(exercise, &directive.attributes, content)?,
        "context" => parse_context_block(exercise, content)?,
        "objectives" => parse_objectives_block(&mut exercise.objectives, content)?,
        "hint" => parse_hint_block(&mut exercise.hints, &directive.attributes, content)?,
        _ => {
            // Unknown directive - ignore
        }
    }
    Ok(())
}

// --- Common Parsers ---

fn parse_objectives_block(objectives_opt: &mut Option<Objectives>, content: &str) -> ParseResult<()> {
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

    *objectives_opt = Some(objectives);
    Ok(())
}

fn parse_hint_block(
    hints: &mut Vec<Hint>,
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

    hints.push(Hint {
        level,
        title,
        content: content.trim().to_string(),
    });

    hints.sort_by_key(|h| h.level);

    Ok(())
}

// --- Code Exercise Specific Parsers ---

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

fn parse_discussion_block(exercise: &mut Exercise, content: &str) -> ParseResult<()> {
    let items = parse_markdown_list(content);
    if !items.is_empty() {
        exercise.discussion = Some(items);
    }
    Ok(())
}

fn parse_starter_block(
    exercise: &mut Exercise,
    attrs: &HashMap<String, String>,
    content: &str,
) -> ParseResult<()> {
    let (language_raw, code) = extract_code_block(content);

    if code.trim().is_empty() {
        return Ok(());
    }

    let mut filename = attrs.get("file").cloned();
    let mut language = attrs.get("language").cloned();

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

fn parse_solution_block(exercise: &mut Exercise, attrs: &HashMap<String, String>, content: &str) -> ParseResult<()> {
    let (language, code) = extract_code_block(content);
    let explanation = extract_explanation(content);

    let mut sol = Solution {
        code,
        language: language.unwrap_or_else(|| "rust".to_string()),
        explanation,
        ..Default::default()
    };

    if sol.code.trim().is_empty() {
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

fn parse_tests_block(
    exercise: &mut Exercise,
    attrs: &HashMap<String, String>,
    content: &str,
) -> ParseResult<()> {
    let (language_raw, code) = extract_code_block(content);

    if code.trim().is_empty() {
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

fn parse_reflection_block(exercise: &mut Exercise, content: &str) -> ParseResult<()> {
    let items = parse_markdown_list(content);
    if !items.is_empty() {
        exercise.reflection = Some(items);
    }
    Ok(())
}

// --- UseCase Exercise Specific Parsers ---

fn parse_usecase_meta_block(exercise: &mut UseCaseExercise, content: &str) -> ParseResult<()> {
    let yaml: serde_yaml::Value =
        serde_yaml::from_str(content).map_err(|e| ParseError::YamlError {
            block: "usecase".to_string(),
            source: e,
        })?;

    if let Some(id) = yaml.get("id").and_then(|v| v.as_str()) {
        exercise.metadata.id = id.to_string();
    } else {
        return Err(ParseError::MissingField {
            block: "usecase".to_string(),
            field: "id".to_string(),
        });
    }

    if let Some(difficulty) = yaml.get("difficulty").and_then(|v| v.as_str()) {
        exercise.metadata.difficulty = difficulty.parse().unwrap_or_default();
    }

    if let Some(domain) = yaml.get("domain").and_then(|v| v.as_str()) {
        exercise.metadata.domain = domain.parse().unwrap_or_default();
    }

    if let Some(time_value) = yaml.get("time") {
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

fn parse_scenario_block(
    exercise: &mut UseCaseExercise,
    _attrs: &HashMap<String, String>,
    content: &str,
) -> ParseResult<()> {
    let mut scenario = Scenario::default();

    // Split content into YAML header lines and markdown content
    // YAML lines look like "key: value" or "key:" followed by list items
    let mut yaml_lines = Vec::new();
    let mut content_lines = Vec::new();
    let mut in_yaml = true;
    let mut in_yaml_list = false;

    for line in content.lines() {
        if in_yaml {
            let trimmed = line.trim();
            // Check if this looks like a YAML key-value line
            if trimmed.contains(':') && !trimmed.starts_with('-') && !trimmed.starts_with('#') {
                yaml_lines.push(line);
                in_yaml_list = trimmed.ends_with(':');
            } else if in_yaml_list && trimmed.starts_with('-') {
                yaml_lines.push(line);
            } else if trimmed.is_empty() && yaml_lines.is_empty() {
                // Skip leading blank lines
            } else {
                // This line doesn't look like YAML, start content
                in_yaml = false;
                in_yaml_list = false;
                content_lines.push(line);
            }
        } else {
            content_lines.push(line);
        }
    }

    if !yaml_lines.is_empty() {
        let yaml_str = yaml_lines.join("\n");
        if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_str) {
            if let Some(org) = yaml.get("organization").and_then(|v| v.as_str()) {
                scenario.organization = Some(org.to_string());
            }
            if let Some(constraints) = yaml.get("constraints").and_then(|v| v.as_sequence()) {
                scenario.constraints = constraints.iter().filter_map(|v| v.as_str()).map(String::from).collect();
            }
        }
    }

    scenario.content = content_lines.join("\n").trim().to_string();
    exercise.scenario = scenario;
    Ok(())
}

fn parse_prompt_block(exercise: &mut UseCaseExercise, content: &str) -> ParseResult<()> {
    // YAML header (aspects) + markdown body
    let mut prompt = UseCasePrompt::default();

    // Split content into YAML header lines and markdown content
    let mut yaml_lines = Vec::new();
    let mut content_lines = Vec::new();
    let mut in_yaml = true;
    let mut in_yaml_list = false;

    for line in content.lines() {
        if in_yaml {
            let trimmed = line.trim();
            if trimmed.contains(':') && !trimmed.starts_with('-') && !trimmed.starts_with('#') {
                yaml_lines.push(line);
                in_yaml_list = trimmed.ends_with(':');
            } else if in_yaml_list && trimmed.starts_with('-') {
                yaml_lines.push(line);
            } else if trimmed.is_empty() && yaml_lines.is_empty() {
                // Skip leading blank lines
            } else {
                in_yaml = false;
                in_yaml_list = false;
                content_lines.push(line);
            }
        } else {
            content_lines.push(line);
        }
    }

    if !yaml_lines.is_empty() {
        let yaml_str = yaml_lines.join("\n");
        if let Ok(yaml) = serde_yaml::from_str::<serde_yaml::Value>(&yaml_str) {
            if let Some(aspects) = yaml.get("aspects").and_then(|v| v.as_sequence()) {
                prompt.aspects = aspects.iter().filter_map(|v| v.as_str()).map(String::from).collect();
            }
        }
    }

    prompt.prompt = content_lines.join("\n").trim().to_string();
    exercise.prompt = prompt;
    Ok(())
}

fn parse_evaluation_block(exercise: &mut UseCaseExercise, content: &str) -> ParseResult<()> {
    // Evaluation block is pure YAML
    let yaml: serde_yaml::Value = serde_yaml::from_str(content).map_err(|e| ParseError::YamlError {
        block: "evaluation".to_string(),
        source: e,
    })?;
    
    let mut eval = EvaluationCriteria::default();
    
    if let Some(min) = yaml.get("min_words").and_then(|v| v.as_u64()) {
        eval.min_words = Some(min as u32);
    }
    if let Some(max) = yaml.get("max_words").and_then(|v| v.as_u64()) {
        eval.max_words = Some(max as u32);
    }
    if let Some(pass) = yaml.get("pass_threshold").and_then(|v| v.as_f64()) {
        eval.pass_threshold = Some(pass as f32);
    }
    
    if let Some(pts) = yaml.get("key_points").and_then(|v| v.as_sequence()) {
        eval.key_points = pts.iter().filter_map(|v| v.as_str()).map(String::from).collect();
    }
    
    if let Some(crit) = yaml.get("criteria").and_then(|v| v.as_sequence()) {
        for c in crit {
            let name = c.get("name").and_then(|v| v.as_str()).unwrap_or("Unknown").to_string();
            let weight = c.get("weight").and_then(|v| v.as_u64()).unwrap_or(0) as u32;
            let desc = c.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
            
            eval.criteria.push(Criterion { name, weight, description: desc });
        }
    }
    
    exercise.evaluation = eval;
    Ok(())
}

fn parse_sample_answer_block(
    exercise: &mut UseCaseExercise, 
    attrs: &HashMap<String, String>, 
    content: &str
) -> ParseResult<()> {
    // Check for expected_score in header
    // Content is markdown
    
    let mut answer = SampleAnswer {
        content: String::new(),
        expected_score: None,
        reveal: SolutionReveal::OnDemand, // Default
    };
    
    // Parse reveal attr
    if let Some(reveal) = attrs.get("reveal").map(|s| s.to_lowercase()) {
        answer.reveal = match reveal.as_str() {
            "always" => SolutionReveal::Always,
            "never" => SolutionReveal::Never,
            _ => SolutionReveal::OnDemand,
        };
    }
    
    // Try to find expected_score in content (YAML-ish header)
    let lines: Vec<&str> = content.lines().collect();
    let mut start_idx = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("expected_score:") {
            if let Some(val_str) = line.split(':').nth(1) {
                if let Ok(val) = val_str.trim().parse::<f32>() {
                    answer.expected_score = Some(val);
                }
            }
            start_idx = i + 1;
        } else if line.trim().is_empty() {
             // skip blank lines at top
             if start_idx == i { start_idx += 1; }
        } else {
            break;
        }
    }
    
    answer.content = lines[start_idx..].join("\n").trim().to_string();
    
    exercise.sample_answer = Some(answer);
    Ok(())
}

fn parse_context_block(exercise: &mut UseCaseExercise, content: &str) -> ParseResult<()> {
    exercise.context = Some(content.trim().to_string());
    Ok(())
}


// --- Helper Functions ---

fn find_excluded_ranges(markdown: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let parser = Parser::new(markdown).into_offset_iter();
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
                if block_start.is_none() {
                    ranges.push(range);
                }
            }
            _ => {}
        }
    }
    ranges
}

fn is_range_excluded(line_range: &Range<usize>, excluded: &[Range<usize>]) -> bool {
    for range in excluded {
        if range.contains(&line_range.start) {
            return true;
        }
        if range.start <= line_range.start && range.end >= line_range.end {
            return true;
        }
    }
    false
}

fn parse_directive_start(line: &str, line_number: usize) -> Option<Directive> {
    let trimmed = line.trim();
    if !trimmed.starts_with(":::") {
        return None;
    }
    let rest = trimmed[3..].trim();
    if rest.is_empty() || rest.starts_with(":::") {
        return None;
    }

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

fn parse_inline_attributes(attrs_str: &str) -> HashMap<String, String> {
    let mut attrs = HashMap::new();
    let mut remaining = attrs_str.trim();

    while !remaining.is_empty() {
        remaining = remaining.trim_start();
        if remaining.is_empty() { break; }

        let key_end = remaining
            .find(|c: char| c == '=' || c.is_whitespace())
            .unwrap_or(remaining.len());

        let key = &remaining[..key_end];
        remaining = &remaining[key_end..];

        if remaining.starts_with('=') {
            remaining = &remaining[1..];
            let value = if remaining.starts_with('"') {
                remaining = &remaining[1..];
                let end = remaining.find('"').unwrap_or(remaining.len());
                let val = &remaining[..end];
                remaining = &remaining[(end + 1).min(remaining.len())..];
                val.to_string()
            } else {
                let end = remaining
                    .find(char::is_whitespace)
                    .unwrap_or(remaining.len());
                let val = &remaining[..end];
                remaining = &remaining[end..];
                val.to_string()
            };
            attrs.insert(key.to_string(), value);
        } else {
            attrs.insert(key.to_string(), "true".to_string());
        }
    }
    attrs
}

fn parse_fence_info(info: &str) -> (String, HashMap<String, String>) {
    let mut lang = String::new();
    let mut attrs = HashMap::new();
    for (i, raw) in info.split(',').enumerate() {
        let token = raw.trim();
        if token.is_empty() { continue; }
        if i == 0 && !token.contains('=') {
            lang = token.to_string();
            continue;
        }
        if let Some(eq) = token.find('=') {
            let (k, v) = token.split_at(eq);
            attrs.insert(k.trim().to_string(), v[1..].trim().to_string());
        } else {
            attrs.insert(token.to_string(), "true".to_string());
        }
    }
    (lang, attrs)
}

fn extract_code_block(content: &str) -> (Option<String>, String) {
    let lines: Vec<&str> = content.lines().collect();
    let mut in_code_block = false;
    let mut language = None;
    let mut code_lines = Vec::new();

    for line in lines {
        if line.trim().starts_with("```") {
            if in_code_block {
                break;
            } else {
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

fn parse_markdown_list(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with('-') || trimmed.starts_with('*') {
                Some(trimmed[1..].trim().to_string())
            } else if trimmed.starts_with(|c: char| c.is_ascii_digit()) && trimmed.contains('.') {
                let dot_pos = trimmed.find('.')?;
                Some(trimmed[dot_pos + 1..].trim().to_string())
            } else {
                None
            }
        })
        .filter(|s| !s.is_empty())
        .collect()
}

fn parse_time_string(time: &str) -> Option<u32> {
    let parts: Vec<&str> = time.split_whitespace().collect();
    if parts.is_empty() { return None; }
    let number: u32 = parts[0].parse().ok()?;
    if parts.len() > 1 {
        let unit = parts[1].to_lowercase();
        if unit.starts_with("hour") {
            return Some(number * 60);
        }
    }
    Some(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_code_exercise() {
        let markdown = r#"
# Hello World

::: exercise
id: hello-world
difficulty: beginner
time: 10 minutes
:::

Write a greeting function.
"#;
        match parse_exercise(markdown).unwrap() {
            ParsedExercise::Code(exercise) => {
                assert_eq!(exercise.metadata.id, "hello-world");
                assert_eq!(exercise.metadata.difficulty, Difficulty::Beginner);
                assert_eq!(exercise.metadata.time_minutes, Some(10));
                assert_eq!(exercise.title, Some("Hello World".to_string()));
            }
            _ => panic!("Expected Code exercise"),
        }
    }
    
    #[test]
    fn test_parse_usecase_exercise() {
        let markdown = r#"
# Security Analysis

::: usecase
id: sec-01
domain: healthcare
difficulty: intermediate
:::

::: scenario
organization: HealthCorp
The scenario text.
:::

::: prompt
Analyze the security.
:::
"#;
        match parse_exercise(markdown).unwrap() {
            ParsedExercise::UseCase(exercise) => {
                assert_eq!(exercise.metadata.id, "sec-01");
                assert_eq!(exercise.metadata.domain, UseCaseDomain::Healthcare);
                assert_eq!(exercise.scenario.organization, Some("HealthCorp".to_string()));
                assert!(exercise.scenario.content.contains("The scenario text"));
            }
            _ => panic!("Expected UseCase exercise"),
        }
    }
}
