# mdbook-exercises Design Document

This document describes the technical architecture and design decisions for `mdbook-exercises`.

## Goals

1. **Single source of truth** - Exercise content lives in Markdown files
2. **Graceful degradation** - Exercises render reasonably in any Markdown viewer
3. **Channel-appropriate experience** - Browser gets interactive UI, MCP servers get structured data
4. **General purpose** - Not tied to any specific course or technology
5. **Extensible** - New block types can be added without breaking changes

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         exercise.md                                  │
│                    (Markdown with directives)                        │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          Parser                                      │
│                     (always available)                               │
│                                                                      │
│   Input: Markdown text                                               │
│   Output: Exercise struct with typed fields                          │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │                               │
              ▼                               ▼
┌─────────────────────────┐     ┌─────────────────────────────────────┐
│   Renderer              │     │   External Consumers                │
│   (feature: render)     │     │                                     │
│                         │     │   • MCP Servers (AI-guided learning)│
│   • HTML generation     │     │   • CLI tools (validation)          │
│   • CSS injection       │     │   • IDE extensions                  │
│   • JS for interactivity│     │   • Export tools                    │
└───────────┬─────────────┘     └─────────────────────────────────────┘
            │
            ▼
┌─────────────────────────┐
│   Preprocessor          │
│   (feature: preprocessor)│
│                         │
│   • mdBook integration  │
│   • Asset copying       │
│   • Chapter processing  │
└───────────┬─────────────┘
            │
            ▼
┌─────────────────────────┐
│   Browser               │
│                         │
│   • Interactive hints   │
│   • Editable code       │
│   • Test execution      │
│   • Progress tracking   │
└─────────────────────────┘
```

## Module Structure

```
src/
├── lib.rs              # Public API, re-exports
├── parser.rs           # Markdown parsing, directive extraction
├── types.rs            # Exercise, Hint, Solution, etc.
├── render.rs           # HTML generation (feature-gated)
├── preprocessor.rs     # mdBook integration (feature-gated)
└── playground.rs       # Rust Playground API types (feature-gated)

assets/
├── exercises.css       # Styling for rendered exercises
└── exercises.js        # Interactivity (hints, tests, progress)
```

## Core Types

### Exercise

The central data structure representing a parsed exercise:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    /// Exercise metadata (id, difficulty, time, prerequisites)
    pub metadata: ExerciseMetadata,

    /// Title extracted from the first heading
    pub title: Option<String>,

    /// Description content (markdown between metadata and first directive)
    pub description: String,

    /// Learning objectives (thinking and doing)
    pub objectives: Option<Objectives>,

    /// Discussion prompts before the exercise
    pub discussion: Option<Vec<String>>,

    /// Starter code for the student to complete
    pub starter: Option<StarterCode>,

    /// Progressive hints (level 1, 2, 3, etc.)
    pub hints: Vec<Hint>,

    /// Complete solution
    pub solution: Option<Solution>,

    /// Test code
    pub tests: Option<TestBlock>,

    /// Reflection questions after the exercise
    pub reflection: Option<Vec<String>>,
}
```

### Metadata

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseMetadata {
    pub id: String,
    pub difficulty: Difficulty,
    pub time_minutes: Option<u32>,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}
```

### Code Blocks

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterCode {
    /// Suggested filename (e.g., "src/main.rs")
    pub filename: Option<String>,

    /// Programming language for syntax highlighting
    pub language: String,

    /// The code content
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    /// The complete solution code
    pub code: String,

    /// Programming language
    pub language: String,

    /// Optional explanation (markdown)
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestBlock {
    /// Programming language
    pub language: String,

    /// The test code
    pub code: String,

    /// Execution mode
    pub mode: TestMode,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TestMode {
    /// Run tests in browser via Rust Playground
    Playground,

    /// Display only, run locally with cargo test
    Local,
}
```

### Hints

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    /// Hint level (1, 2, 3, etc.)
    pub level: u8,

    /// Optional title for the hint
    pub title: Option<String>,

    /// Hint content (markdown, may include code blocks)
    pub content: String,
}
```

## Parsing

### Directive Syntax

We use a fenced directive syntax inspired by [CommonMark Generic Directives](https://talk.commonmark.org/t/generic-directives-plugins-syntax/444):

```markdown
::: directive-name attribute1=value1 attribute2
content here
:::
```

This syntax:
- Is valid Markdown (rendered as text in non-supporting viewers)
- Is familiar from other systems (remark-directive, Docusaurus, etc.)
- Supports both inline attributes and YAML-style content

### Parser Implementation

```rust
pub fn parse_exercise(markdown: &str) -> Result<Exercise, ParseError> {
    let mut exercise = Exercise::default();
    let mut current_block: Option<BlockType> = None;
    let mut block_content = String::new();
    let mut block_attrs = HashMap::new();

    for line in markdown.lines() {
        if let Some(directive) = parse_directive_start(line) {
            // Finish previous block
            if let Some(block_type) = current_block.take() {
                process_block(&mut exercise, block_type, &block_content, &block_attrs)?;
            }

            current_block = Some(directive.block_type);
            block_attrs = directive.attributes;
            block_content.clear();
        } else if line.trim() == ":::" && current_block.is_some() {
            // End of block
            let block_type = current_block.take().unwrap();
            process_block(&mut exercise, block_type, &block_content, &block_attrs)?;
            block_content.clear();
            block_attrs.clear();
        } else if current_block.is_some() {
            // Inside a block
            block_content.push_str(line);
            block_content.push('\n');
        } else {
            // Regular content (description)
            exercise.description.push_str(line);
            exercise.description.push('\n');
        }
    }

    Ok(exercise)
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Missing required field '{field}' in {block} block")]
    MissingField { block: String, field: String },

    #[error("Invalid attribute value '{value}' for '{attribute}'")]
    InvalidAttribute { attribute: String, value: String },

    #[error("Unclosed directive block starting at line {line}")]
    UnclosedBlock { line: usize },

    #[error("Duplicate block type '{block_type}' (only one allowed)")]
    DuplicateBlock { block_type: String },

    #[error("YAML parse error in {block} block: {source}")]
    YamlError { block: String, source: serde_yaml::Error },
}
```

## Rendering

### HTML Structure

The renderer generates semantic HTML with data attributes for JavaScript:

```html
<article class="exercise" data-exercise-id="hello-world" data-difficulty="beginner">
  <header class="exercise-header">
    <h2 class="exercise-title">Exercise: Hello World</h2>
    <div class="exercise-meta">
      <span class="difficulty beginner">Beginner</span>
      <span class="time">10 minutes</span>
    </div>
  </header>

  <section class="exercise-description">
    <p>Write a function that returns a greeting.</p>
  </section>

  <section class="exercise-objectives">
    <h3>Learning Objectives</h3>
    <div class="objectives-grid">
      <div class="objectives-thinking">
        <h4>Thinking</h4>
        <ul>
          <li><input type="checkbox" id="obj-1"><label for="obj-1">Understand X</label></li>
        </ul>
      </div>
      <div class="objectives-doing">
        <h4>Doing</h4>
        <ul>
          <li><input type="checkbox" id="obj-2"><label for="obj-2">Implement Y</label></li>
        </ul>
      </div>
    </div>
  </section>

  <section class="exercise-starter">
    <div class="code-header">
      <span class="filename">src/lib.rs</span>
      <div class="code-actions">
        <button class="btn-copy" title="Copy code">📋</button>
        <button class="btn-reset" title="Reset to original">↺</button>
      </div>
    </div>
    <textarea class="code-editor" id="code-hello-world" data-original="...">
pub fn greet(name: &amp;str) -> String {
    todo!()
}
    </textarea>
  </section>

  <section class="exercise-hints">
    <h3>Hints</h3>
    <details class="hint" data-level="1">
      <summary>Hint 1: Getting Started</summary>
      <div class="hint-content">
        <p>Use the <code>format!</code> macro...</p>
      </div>
    </details>
    <details class="hint" data-level="2">
      <summary>Hint 2</summary>
      <div class="hint-content">
        <pre><code class="language-rust">format!("Hello, {}!", name)</code></pre>
      </div>
    </details>
  </section>

  <section class="exercise-solution">
    <details class="solution">
      <summary>
        <span class="solution-warning">⚠️ Try the exercise first!</span>
        <span class="solution-toggle">Show Solution</span>
      </summary>
      <div class="solution-content">
        <pre><code class="language-rust">pub fn greet(name: &amp;str) -> String {
    format!("Hello, {}!", name)
}</code></pre>
        <div class="solution-explanation">
          <h4>Explanation</h4>
          <p>The <code>format!</code> macro...</p>
        </div>
      </div>
    </details>
  </section>

  <section class="exercise-tests" data-mode="playground">
    <h3>Tests</h3>
    <pre><code class="language-rust">#[test]
fn test_greet() {
    assert_eq!(greet("World"), "Hello, World!");
}</code></pre>
    <div class="test-actions">
      <button class="btn-run-tests" data-exercise-id="hello-world">▶ Run Tests</button>
    </div>
    <div class="test-results" id="results-hello-world" hidden>
      <!-- Populated by JavaScript -->
    </div>
  </section>

  <section class="exercise-reflection">
    <h3>Reflection</h3>
    <ul>
      <li>What did you learn?</li>
      <li>How would you extend this?</li>
    </ul>
  </section>

  <footer class="exercise-footer">
    <button class="btn-complete" data-exercise-id="hello-world">
      Mark Complete
    </button>
  </footer>
</article>
```

### JavaScript Functionality

```javascript
// exercises.js

class ExerciseManager {
  constructor() {
    this.exercises = new Map();
    this.progress = this.loadProgress();
    this.initializeAll();
  }

  initializeAll() {
    document.querySelectorAll('.exercise').forEach(el => {
      const id = el.dataset.exerciseId;
      this.exercises.set(id, {
        element: el,
        originalCode: el.querySelector('.code-editor')?.dataset.original,
      });

      this.restoreProgress(id);
      this.attachEventListeners(el, id);
    });
  }

  attachEventListeners(el, id) {
    // Copy button
    el.querySelector('.btn-copy')?.addEventListener('click', () => {
      this.copyCode(id);
    });

    // Reset button
    el.querySelector('.btn-reset')?.addEventListener('click', () => {
      this.resetCode(id);
    });

    // Run tests button
    el.querySelector('.btn-run-tests')?.addEventListener('click', () => {
      this.runTests(id);
    });

    // Mark complete button
    el.querySelector('.btn-complete')?.addEventListener('click', () => {
      this.toggleComplete(id);
    });

    // Objective checkboxes
    el.querySelectorAll('.objectives input[type="checkbox"]').forEach(cb => {
      cb.addEventListener('change', () => this.saveProgress(id));
    });
  }

  async runTests(id) {
    const exercise = this.exercises.get(id);
    const editor = exercise.element.querySelector('.code-editor');
    const testsSection = exercise.element.querySelector('.exercise-tests');
    const resultsEl = exercise.element.querySelector('.test-results');

    if (testsSection.dataset.mode !== 'playground') {
      this.showLocalTestInstructions(id);
      return;
    }

    const userCode = editor.value;
    const testCode = testsSection.querySelector('code').textContent;

    // Combine user code with tests
    const fullCode = `${userCode}\n\n${testCode}`;

    resultsEl.innerHTML = '<div class="loading">Running tests...</div>';
    resultsEl.hidden = false;

    try {
      const response = await fetch('https://play.rust-lang.org/execute', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          code: fullCode,
          edition: '2021',
          channel: 'stable',
          mode: 'debug',
          crateType: 'lib',
          tests: true,
        }),
      });

      const result = await response.json();
      this.displayTestResults(id, result);
    } catch (error) {
      resultsEl.innerHTML = `<div class="error">Failed to run tests: ${error.message}</div>`;
    }
  }

  displayTestResults(id, result) {
    const resultsEl = this.exercises.get(id).element.querySelector('.test-results');

    if (result.success) {
      // Parse test output
      const output = result.stdout || result.stderr;
      const passed = (output.match(/test .+ \.\.\. ok/g) || []).length;
      const failed = (output.match(/test .+ \.\.\. FAILED/g) || []).length;

      resultsEl.innerHTML = `
        <div class="test-summary ${failed > 0 ? 'has-failures' : 'all-passed'}">
          <span class="passed">✓ ${passed} passed</span>
          ${failed > 0 ? `<span class="failed">✗ ${failed} failed</span>` : ''}
        </div>
        <pre class="test-output">${this.escapeHtml(output)}</pre>
      `;
    } else {
      // Compilation error
      resultsEl.innerHTML = `
        <div class="test-summary compilation-error">
          <span class="error">Compilation Error</span>
        </div>
        <pre class="error-output">${this.escapeHtml(result.stderr)}</pre>
      `;
    }
  }

  // Progress tracking via localStorage
  loadProgress() {
    try {
      return JSON.parse(localStorage.getItem('mdbook-exercises-progress') || '{}');
    } catch {
      return {};
    }
  }

  saveProgress(id) {
    const exercise = this.exercises.get(id);
    const objectives = [...exercise.element.querySelectorAll('.objectives input')]
      .map(cb => cb.checked);

    this.progress[id] = {
      completed: exercise.element.classList.contains('completed'),
      objectives,
      code: exercise.element.querySelector('.code-editor')?.value,
    };

    localStorage.setItem('mdbook-exercises-progress', JSON.stringify(this.progress));
  }

  restoreProgress(id) {
    const saved = this.progress[id];
    if (!saved) return;

    const exercise = this.exercises.get(id);

    if (saved.completed) {
      exercise.element.classList.add('completed');
    }

    if (saved.objectives) {
      exercise.element.querySelectorAll('.objectives input').forEach((cb, i) => {
        cb.checked = saved.objectives[i] || false;
      });
    }

    if (saved.code) {
      const editor = exercise.element.querySelector('.code-editor');
      if (editor) editor.value = saved.code;
    }
  }

  escapeHtml(text) {
    return text
      .replace(/&/g, '&amp;')
      .replace(/</g, '&lt;')
      .replace(/>/g, '&gt;');
  }
}

// Initialize when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  window.exerciseManager = new ExerciseManager();
});
```

## MCP Server Integration

### Use Case

An AI-assisted learning platform can use `mdbook-exercises` to:

1. Parse exercise content from Markdown files
2. Load AI-specific instructions from companion `.ai.toml` files
3. Provide structured exercise data to AI assistants
4. Guide students through exercises with hints and feedback

### Architecture

```
exercises/
├── ch02-01-hello-world.md       # Exercise content (human + AI readable)
└── ch02-01-hello-world.ai.toml  # AI-specific instructions
```

### AI Instructions File Format

```toml
# ch02-01-hello-world.ai.toml

[metadata]
# Reference to the exercise file
exercise_file = "ch02-01-hello-world.md"

[instructions]
# Role for the AI assistant
role = """
You are a patient programming tutor helping a beginner learn Rust.
Focus on building understanding, not just getting the right answer.
"""

# Approach for guiding the student
approach = """
1. Start with discussion prompts - ensure they understand the concept
2. Let them try before offering hints
3. If stuck for >3 minutes, offer first hint
4. Celebrate small wins
"""

[policies]
# When to reveal hints
hint_policy = "progressive"  # level N requires viewing N-1

# When to reveal solution
solution_policy = "after_attempt"  # must submit wrong answer first

# Time-based hint unlocking
stuck_threshold_minutes = 5

[watch_for]
# Common mistakes to catch
common_mistakes = [
    "Forgetting the exclamation mark in format!",
    "Using + for string concatenation instead of format!",
    "Missing the & in &str parameter",
]

[feedback]
# Responses for common situations
on_success = "Excellent! You've got it. Let's look at the reflection questions."
on_partial = "Good progress! You're on the right track."
on_stuck = "No worries, this is tricky. Would you like a hint?"
```

### MCP Server Implementation

```rust
use mdbook_exercises::{parse_exercise, Exercise};
use std::collections::HashMap;
use std::path::Path;

pub struct ExerciseServer {
    exercises: HashMap<String, Exercise>,
    ai_instructions: HashMap<String, AiInstructions>,
    student_progress: HashMap<String, StudentProgress>,
}

impl ExerciseServer {
    pub fn load_from_dir(dir: &Path) -> Result<Self, Error> {
        let mut exercises = HashMap::new();
        let mut ai_instructions = HashMap::new();

        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();

            if path.extension() == Some("md".as_ref()) {
                let content = std::fs::read_to_string(&path)?;
                let exercise = parse_exercise(&content)?;
                let id = exercise.metadata.id.clone();

                exercises.insert(id.clone(), exercise);

                // Load companion AI instructions
                let ai_path = path.with_extension("ai.toml");
                if ai_path.exists() {
                    let ai_content = std::fs::read_to_string(&ai_path)?;
                    let instructions: AiInstructions = toml::from_str(&ai_content)?;
                    ai_instructions.insert(id, instructions);
                }
            }
        }

        Ok(Self {
            exercises,
            ai_instructions,
            student_progress: HashMap::new(),
        })
    }

    // MCP Tool: List available exercises
    pub fn list_exercises(&self) -> Vec<ExerciseSummary> {
        self.exercises.values()
            .map(|e| ExerciseSummary {
                id: e.metadata.id.clone(),
                title: e.title.clone(),
                difficulty: e.metadata.difficulty,
                time_minutes: e.metadata.time_minutes,
                completed: self.is_completed(&e.metadata.id),
            })
            .collect()
    }

    // MCP Tool: Get exercise details
    pub fn get_exercise(&self, id: &str) -> Option<ExerciseDetails> {
        let exercise = self.exercises.get(id)?;
        let instructions = self.ai_instructions.get(id);

        Some(ExerciseDetails {
            exercise: exercise.clone(),
            ai_role: instructions.map(|i| i.instructions.role.clone()),
            ai_approach: instructions.map(|i| i.instructions.approach.clone()),
        })
    }

    // MCP Tool: Get hint (with policy enforcement)
    pub fn get_hint(&self, id: &str, level: u8) -> Result<&Hint, HintError> {
        let exercise = self.exercises.get(id)
            .ok_or(HintError::ExerciseNotFound)?;

        let instructions = self.ai_instructions.get(id);
        let progress = self.student_progress.get(id);

        // Check if hint should be unlocked
        if let Some(inst) = instructions {
            if inst.policies.hint_policy == "progressive" {
                if level > 1 {
                    let prev_viewed = progress
                        .map(|p| p.hints_viewed.contains(&(level - 1)))
                        .unwrap_or(false);

                    if !prev_viewed {
                        return Err(HintError::PreviousHintRequired(level - 1));
                    }
                }
            }
        }

        exercise.hints.iter()
            .find(|h| h.level == level)
            .ok_or(HintError::HintNotFound(level))
    }

    // MCP Tool: Submit solution attempt
    pub fn submit_attempt(&mut self, id: &str, code: &str) -> AttemptResult {
        // Record the attempt
        let progress = self.student_progress.entry(id.to_string())
            .or_insert_with(StudentProgress::default);
        progress.attempts.push(code.to_string());

        // In a full implementation, you might:
        // - Compare with solution
        // - Run tests
        // - Provide feedback based on common_mistakes

        AttemptResult {
            attempt_number: progress.attempts.len(),
            // ... other feedback
        }
    }
}
```

## Test Execution Flow

### Browser (Playground Mode)

```
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  User edits  │     │  Click "Run      │     │  Combine user   │
│  starter     │ ──▶ │  Tests" button   │ ──▶ │  code + tests   │
│  code        │     │                  │     │                 │
└──────────────┘     └──────────────────┘     └────────┬────────┘
                                                       │
                                                       ▼
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Display     │     │  Parse stdout    │     │  POST to        │
│  results     │ ◀── │  for test        │ ◀── │  play.rust-     │
│  in UI       │     │  results         │     │  lang.org       │
└──────────────┘     └──────────────────┘     └─────────────────┘
```

### MCP Server (Full Capabilities)

```
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Student     │     │  MCP Server      │     │  Load exercise  │
│  requests    │ ──▶ │  receives tool   │ ──▶ │  + AI           │
│  exercise    │     │  call            │     │  instructions   │
└──────────────┘     └──────────────────┘     └────────┬────────┘
                                                       │
      ┌────────────────────────────────────────────────┘
      │
      ▼
┌──────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  AI guides   │     │  Student         │     │  MCP validates  │
│  discussion  │ ──▶ │  writes code     │ ──▶ │  with cargo     │
│  phase       │     │                  │     │  test           │
└──────────────┘     └──────────────────┘     └────────┬────────┘
                                                       │
                                                       ▼
                                              ┌─────────────────┐
                                              │  AI provides    │
                                              │  feedback based │
                                              │  on results     │
                                              └─────────────────┘
```

## Future Considerations

### Potential Extensions

1. **Multi-language support** - Python, JavaScript, Go exercises
2. **Collaborative features** - Share progress, peer review
3. **Analytics** - Track common mistakes, time spent
4. **Adaptive difficulty** - Adjust based on performance
5. **Offline mode** - Service worker for offline access

### Breaking Change Policy

- Major version: Breaking changes to directive syntax or types
- Minor version: New directive types, new optional attributes
- Patch version: Bug fixes, rendering improvements

### Compatibility

- mdBook: 0.4.x and later
- Rust: 1.70+ (for async traits if needed)
- Browsers: ES2020+ (async/await, optional chaining)
