//! Core types for representing parsed exercises.

use serde::{Deserialize, Serialize};

/// A parsed exercise with all its components.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Metadata about an exercise.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExerciseMetadata {
    /// Unique identifier for the exercise
    pub id: String,

    /// Difficulty level
    pub difficulty: Difficulty,

    /// Estimated time in minutes
    pub time_minutes: Option<u32>,

    /// List of prerequisite exercise IDs
    pub prerequisites: Vec<String>,
}

/// Difficulty level of an exercise.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    #[default]
    Beginner,
    Intermediate,
    Advanced,
}

impl std::fmt::Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "beginner"),
            Difficulty::Intermediate => write!(f, "intermediate"),
            Difficulty::Advanced => write!(f, "advanced"),
        }
    }
}

impl std::str::FromStr for Difficulty {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "beginner" => Ok(Difficulty::Beginner),
            "intermediate" => Ok(Difficulty::Intermediate),
            "advanced" => Ok(Difficulty::Advanced),
            _ => Err(format!("Invalid difficulty: {}", s)),
        }
    }
}

/// Learning objectives for an exercise.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Objectives {
    /// Conceptual understanding goals
    pub thinking: Vec<String>,

    /// Practical skill goals
    pub doing: Vec<String>,
}

/// Starter code for the student to complete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarterCode {
    /// Suggested filename (e.g., "src/main.rs")
    pub filename: Option<String>,

    /// Programming language for syntax highlighting
    pub language: String,

    /// The code content
    pub code: String,
}

impl Default for StarterCode {
    fn default() -> Self {
        Self {
            filename: None,
            language: "rust".to_string(),
            code: String::new(),
        }
    }
}

/// A hint to help students who are stuck.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    /// Hint level (1, 2, 3, etc.)
    pub level: u8,

    /// Optional title for the hint
    pub title: Option<String>,

    /// Hint content (markdown, may include code blocks)
    pub content: String,
}

/// The complete solution for an exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Solution {
    /// The complete solution code
    pub code: String,

    /// Programming language
    pub language: String,

    /// Optional explanation (markdown)
    pub explanation: Option<String>,

    /// Reveal policy for this solution (on-demand/always/never)
    #[serde(skip)]
    pub reveal: SolutionReveal,
}

impl Default for Solution {
    fn default() -> Self {
        Self {
            code: String::new(),
            language: "rust".to_string(),
            explanation: None,
            reveal: SolutionReveal::OnDemand,
        }
    }
}

/// When to reveal a solution in the UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SolutionReveal {
    /// Hidden behind a toggle
    OnDemand,
    /// Shown expanded
    Always,
    /// Never revealed (UI hides the toggle)
    Never,
}

impl Default for SolutionReveal {
    fn default() -> Self { SolutionReveal::OnDemand }
}

/// Test code for verifying solutions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestBlock {
    /// Programming language
    pub language: String,

    /// The test code
    pub code: String,

    /// Execution mode
    pub mode: TestMode,
}

impl Default for TestBlock {
    fn default() -> Self {
        Self {
            language: "rust".to_string(),
            code: String::new(),
            mode: TestMode::Playground,
        }
    }
}

/// How tests should be executed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestMode {
    /// Run tests in browser via Rust Playground
    #[default]
    Playground,

    /// Display only, run locally with cargo test
    Local,
}

impl std::fmt::Display for TestMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestMode::Playground => write!(f, "playground"),
            TestMode::Local => write!(f, "local"),
        }
    }
}

impl std::str::FromStr for TestMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "playground" => Ok(TestMode::Playground),
            "local" => Ok(TestMode::Local),
            _ => Err(format!("Invalid test mode: {}", s)),
        }
    }
}
