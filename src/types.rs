//! Core types for representing parsed exercises.

use serde::{Deserialize, Serialize};

/// A top-level wrapper for any type of parsed exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ParsedExercise {
    Code(Exercise),
    UseCase(UseCaseExercise),
}

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

/// A UseCase exercise - scenario-based analysis with LLM evaluation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UseCaseExercise {
    /// Exercise metadata
    pub metadata: UseCaseMetadata,

    /// Title from first heading
    pub title: Option<String>,

    /// Brief description (before scenario)
    pub description: String,

    /// The scenario context (multi-paragraph business situation)
    pub scenario: Scenario,

    /// The analysis prompt (what to address in the response)
    pub prompt: UseCasePrompt,

    /// Strategic hints (not code hints)
    pub hints: Vec<Hint>,

    /// Evaluation criteria for LLM-as-Judge
    pub evaluation: EvaluationCriteria,

    /// Sample answer for calibration
    pub sample_answer: Option<SampleAnswer>,

    /// Educational context shown after submission
    pub context: Option<String>,

    /// Learning objectives
    pub objectives: Option<Objectives>,
}

/// Metadata specific to UseCase exercises.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UseCaseMetadata {
    /// Unique identifier
    pub id: String,

    /// Difficulty level
    pub difficulty: Difficulty,

    /// Estimated time in minutes
    pub time_minutes: Option<u32>,

    /// Domain category
    pub domain: UseCaseDomain,

    /// Prerequisites
    pub prerequisites: Vec<String>,
}

/// Domain categories for UseCase exercises.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UseCaseDomain {
    #[default]
    General,
    Healthcare,
    Defense,
    Financial,
}

impl std::fmt::Display for UseCaseDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UseCaseDomain::Healthcare => write!(f, "healthcare"),
            UseCaseDomain::Defense => write!(f, "defense"),
            UseCaseDomain::Financial => write!(f, "financial"),
            UseCaseDomain::General => write!(f, "general"),
        }
    }
}

impl std::str::FromStr for UseCaseDomain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "healthcare" => Ok(UseCaseDomain::Healthcare),
            "defense" => Ok(UseCaseDomain::Defense),
            "financial" => Ok(UseCaseDomain::Financial),
            "general" => Ok(UseCaseDomain::General),
            _ => Err(format!("Invalid domain: {}", s)),
        }
    }
}

/// The scenario context for a UseCase exercise.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Scenario {
    /// Organization name (fictional)
    pub organization: Option<String>,

    /// The full scenario text (markdown)
    pub content: String,

    /// Constraints or requirements
    pub constraints: Vec<String>,
}

/// The prompt asking for analysis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UseCasePrompt {
    /// The main question/prompt
    pub prompt: String,

    /// Specific aspects to address
    pub aspects: Vec<String>,
}

/// Evaluation criteria for LLM-as-Judge.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    /// Weighted rubric criteria
    pub criteria: Vec<Criterion>,

    /// Key points the response should cover
    pub key_points: Vec<String>,

    /// Minimum word count
    pub min_words: Option<u32>,

    /// Maximum word count
    pub max_words: Option<u32>,

    /// Passing threshold (0.0-1.0)
    pub pass_threshold: Option<f32>,
}

/// A single evaluation criterion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Criterion {
    /// Name of the criterion
    pub name: String,

    /// Weight (should sum to 100 across all criteria)
    pub weight: u32,

    /// Description of what good looks like
    pub description: String,
}

/// Sample answer for LLM calibration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleAnswer {
    /// The sample response text
    pub content: String,

    /// Expected score for this sample (for calibration)
    pub expected_score: Option<f32>,

    /// Reveal policy
    #[serde(skip)]
    pub reveal: SolutionReveal,
}

// Note: Evaluation output types (UseCaseEvaluation, CriterionScore, KeyPointCoverage,
// EvaluationFeedback) are defined in the MCP server crate, not here. The preprocessor
// only needs to parse and render exercises; evaluation happens server-side.