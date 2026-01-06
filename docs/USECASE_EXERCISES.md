# UseCase Exercises: Design Document

This document specifies the design for **UseCase exercises** - a new exercise type for scenario-based analysis and written response exercises, evaluated by LLM-as-a-Judge.

## Overview

UseCase exercises present students with realistic enterprise scenarios and ask them to analyze the situation and propose solutions. Unlike code-based exercises (which have starter code, tests, and automated validation), UseCase exercises:

- Present a multi-paragraph scenario with business context
- Ask open-ended analysis questions
- Accept written prose responses (150-600 words typically)
- Are evaluated by an LLM using weighted criteria and key points
- Provide educational context after submission

### Use Cases for UseCase Exercises

1. **Business analysis**: How would you implement MCP for a healthcare organization with HIPAA requirements?
2. **Architecture design**: Design a multi-tenant deployment for an alliance of organizations.
3. **Change management**: Develop a 90-day transition plan from shadow AI to governed MCP.
4. **Troubleshooting scenarios**: A customer reports X issue - how would you investigate?

## Relationship to Existing Exercise Types

```
┌────────────────────────────────────────────────────────────────────┐
│                         Exercise Types                              │
├─────────────────────────────┬──────────────────────────────────────┤
│      Code Exercises         │       UseCase Exercises              │
│      (existing)             │       (new)                          │
├─────────────────────────────┼──────────────────────────────────────┤
│ • Starter code              │ • Scenario description               │
│ • Hints (progressive)       │ • Hints (strategic guidance)         │
│ • Solution (code)           │ • Sample answer (prose)              │
│ • Tests (automated)         │ • Evaluation criteria (LLM)          │
│ • Playground execution      │ • LLM-as-Judge evaluation            │
│ • Reflection questions      │ • Context/learning points            │
└─────────────────────────────┴──────────────────────────────────────┘
```

## Architecture

### Rendering Pipeline

```
┌─────────────────────────────────────────────────────────────────────┐
│                      usecase-exercise.md                             │
│                   (Markdown with directives)                         │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          Parser                                      │
│                  (extended for UseCase)                              │
│                                                                      │
│   Input: Markdown with UseCase directives                            │
│   Output: UseCaseExercise struct                                     │
└─────────────────────────────┬───────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              │                               │
              ▼                               ▼
┌─────────────────────────┐     ┌─────────────────────────────────────┐
│   HTML Renderer         │     │   MCP Server                        │
│   (feature: render)     │     │   (course server)                   │
│                         │     │                                     │
│   • Scenario display    │     │   • get_usecase_exercise            │
│   • Prompt display      │     │   • submit_usecase_response         │
│   • Hints (collapsible) │     │   • LLM-as-Judge evaluation         │
│   • Response textarea   │     │   • Progress tracking               │
│   • Submit button       │     │                                     │
└───────────┬─────────────┘     └─────────────────────────────────────┘
            │
            ▼
┌─────────────────────────┐
│   Browser               │
│                         │
│   • Read scenario       │
│   • View hints          │
│   • Write response      │
│   • Submit (via MCP)    │
│   • View feedback       │
└─────────────────────────┘
```

### Evaluation Pipeline (MCP Server)

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Student        │     │  MCP Server     │     │  LLM Evaluator  │
│  Response       │ ──▶ │  Prepares       │ ──▶ │  (Claude)       │
│  (text)         │     │  Evaluation     │     │                 │
│                 │     │  Prompt         │     │                 │
└─────────────────┘     └─────────────────┘     └────────┬────────┘
                                                         │
                                                         ▼
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│  Feedback to    │     │  Parse LLM      │     │  LLM Returns    │
│  Student        │ ◀── │  Response       │ ◀── │  Structured     │
│  (with scores)  │     │                 │     │  Evaluation     │
└─────────────────┘     └─────────────────┘     └─────────────────┘
```

## Core Types

### UseCaseExercise

```rust
/// A UseCase exercise - scenario-based analysis with LLM evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub hints: Vec<UseCaseHint>,

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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UseCaseDomain {
    /// Healthcare with HIPAA, PHI, clinical systems
    Healthcare,

    /// Defense with classification, security, compliance
    Defense,

    /// Financial services with regulations, real-time data
    Financial,

    /// General enterprise scenarios
    General,
}
```

### Scenario and Prompt

```rust
/// The scenario context for a UseCase exercise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scenario {
    /// Organization name (fictional)
    pub organization: Option<String>,

    /// Industry context
    pub industry: Option<String>,

    /// The full scenario text (markdown)
    pub content: String,

    /// Key stakeholders mentioned
    pub stakeholders: Vec<String>,

    /// Constraints or requirements
    pub constraints: Vec<String>,
}

/// The prompt asking for analysis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCasePrompt {
    /// The main question/prompt
    pub prompt: String,

    /// Specific aspects to address
    pub aspects: Vec<String>,

    /// Strategic hints (optional)
    pub hints: Vec<String>,
}
```

### Evaluation Criteria

```rust
/// Evaluation criteria for LLM-as-Judge.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}
```

### LLM Evaluation Output

```rust
/// Result from LLM-as-Judge evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCaseEvaluation {
    /// Overall score (0.0-1.0)
    pub overall_score: f32,

    /// Whether the student passed
    pub passed: bool,

    /// Scores per criterion
    pub criterion_scores: Vec<CriterionScore>,

    /// Key points covered
    pub key_points_covered: Vec<KeyPointCoverage>,

    /// Qualitative feedback
    pub feedback: EvaluationFeedback,

    /// Word count of submission
    pub word_count: u32,
}

/// Score for a single criterion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CriterionScore {
    /// Criterion name
    pub name: String,

    /// Score (0.0-1.0)
    pub score: f32,

    /// Weight used
    pub weight: u32,

    /// Specific feedback for this criterion
    pub feedback: String,
}

/// Coverage of a key point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPointCoverage {
    /// The key point
    pub key_point: String,

    /// Whether it was covered
    pub covered: bool,

    /// How well it was covered (if covered)
    pub quality: Option<String>,
}

/// Qualitative feedback from the evaluator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationFeedback {
    /// Summary of strengths
    pub strengths: Vec<String>,

    /// Areas for improvement
    pub improvements: Vec<String>,

    /// Overall narrative feedback
    pub narrative: String,
}
```

## Directive Syntax

### New Directives

#### `::: usecase`

Replaces `::: exercise` for UseCase exercises.

```markdown
::: usecase
id: b1uc-001-healthcare-shadow
domain: healthcare
difficulty: intermediate
time: 45 minutes
prerequisites: [ch01, ch02]
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `id` | Yes | string | Unique identifier with `uc-` in the name |
| `domain` | Yes | enum | `healthcare`, `defense`, `financial`, `general` |
| `difficulty` | Yes | enum | `beginner`, `intermediate`, `advanced` |
| `time` | No | string | Estimated completion time |
| `prerequisites` | No | array | Required prior exercises/chapters |

---

#### `::: scenario`

The business context (replaces description for UseCase).

```markdown
::: scenario
organization: Memorial Regional Health System
industry: Healthcare
stakeholders:
  - Chief Medical Information Officer
  - Director of Clinical Informatics
  - HIPAA Compliance Officer
constraints:
  - HIPAA compliance required for all PHI access
  - Epic EHR is the system of record
  - No patient data can leave the health system network

**Memorial Regional Health System** is a 500-bed academic medical center...

[Multi-paragraph scenario description...]
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `organization` | No | string | Fictional organization name |
| `industry` | No | string | Industry sector |
| `stakeholders` | No | array | Key people mentioned in scenario |
| `constraints` | No | array | Business/technical constraints |
| (content) | Yes | markdown | The scenario narrative |

---

#### `::: prompt`

The analysis question (what to address).

```markdown
::: prompt
aspects:
  - Authentication and authorization architecture
  - HIPAA compliance controls
  - Audit trail requirements
  - User experience for physicians
hints:
  - Consider Azure AD integration patterns
  - Think about MFA for PHI access
  - Address break-glass emergency access

Design an MCP authentication and authorization architecture for
Memorial Regional that enables physician productivity while
maintaining strict HIPAA compliance.

Address each of the aspects listed above in your response.
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `aspects` | No | array | Specific aspects to address |
| `hints` | No | array | Strategic hints for the analysis |
| (content) | Yes | markdown | The main prompt text |

---

#### `::: evaluation`

Evaluation criteria for LLM-as-Judge.

```markdown
::: evaluation
min_words: 200
max_words: 600
pass_threshold: 0.7

criteria:
  - name: Security Architecture
    weight: 30
    description: Explains authentication, authorization, and access control design
  - name: HIPAA Compliance
    weight: 25
    description: Addresses PHI protection, audit trails, and regulatory requirements
  - name: User Experience
    weight: 20
    description: Considers physician workflow and productivity
  - name: Technical Feasibility
    weight: 25
    description: Proposes realistic, implementable solutions

key_points:
  - OAuth 2.0 / OIDC integration with Azure AD
  - Role-based access control mapped to clinical roles
  - MFA for PHI access with step-up authentication
  - Audit logging for HIPAA compliance
  - Break-glass emergency access procedures
  - EHR integration considerations
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `min_words` | No | integer | Minimum word count |
| `max_words` | No | integer | Maximum word count |
| `pass_threshold` | No | float | Score needed to pass (0.0-1.0) |
| `criteria` | Yes | array | Weighted evaluation criteria |
| `key_points` | Yes | array | Expected topics to cover |

---

#### `::: sample-answer`

Sample answer for LLM calibration (not shown to students).

```markdown
::: sample-answer reveal=never
expected_score: 0.85

**Authentication Architecture:**

Memorial Regional should implement OAuth 2.0 / OIDC integration with
their existing Azure AD infrastructure...

[Full sample answer...]
:::
```

**Fields:**

| Field | Required | Type | Description |
|-------|----------|------|-------------|
| `reveal` | No | enum | `never` (default), `after-submission`, `on-demand` |
| `expected_score` | No | float | Expected score for calibration |
| (content) | Yes | markdown | The sample answer text |

---

#### `::: context`

Educational context shown after submission (replaces reflection).

```markdown
::: context
**Key Learning Points:**

Healthcare MCP implementations must address unique regulatory requirements:

1. **HIPAA Security Rule**: Requires access controls, audit trails,
   encryption, and workforce training...

2. **Clinical Workflow Integration**: Physicians have limited time...

[Educational explanation...]
:::
```

---

### Reused Directives

These directives from code exercises are reused with slight modifications:

- `::: objectives` - Learning objectives (same as code exercises)
- `::: hint` - Strategic hints (content is prose, not code)

## Complete Example

```markdown
# Exercise: Healthcare SSO and HIPAA Compliance

::: usecase
id: b4uc-001-healthcare-sso
domain: healthcare
difficulty: intermediate
time: 45 minutes
prerequisites: [ch13-oauth, ch14-providers]
:::

Design authentication for a healthcare organization with HIPAA requirements.

::: objectives
thinking:
  - Understand HIPAA Security Rule requirements for access control
  - Recognize the balance between security and clinical workflow
  - Identify integration patterns for healthcare identity systems

doing:
  - Design OAuth/SSO architecture for healthcare MCP
  - Map clinical roles to MCP permissions
  - Address audit logging for compliance
:::

::: scenario
organization: Valley Health Partners
industry: Healthcare
stakeholders:
  - Chief Information Security Officer
  - Clinical Informatics Director
  - HIPAA Privacy Officer
  - Emergency Department Medical Director
constraints:
  - Must integrate with existing Azure AD
  - HIPAA compliance is non-negotiable
  - Cannot disrupt clinical workflows
  - Must support 25,000 employees across 15 hospitals

**Valley Health Partners** is a healthcare network with 15 hospitals,
200 clinics, and 25,000 employees across physicians, nurses,
administrative staff, and IT.

**Current Identity Infrastructure:**
- Microsoft Azure AD as the primary identity provider
- Azure AD groups for role management (e.g., "Physicians", "Nurses")
- Existing HIPAA-compliant SSO for all internal applications
- MFA required for clinical system access

**MCP Server Portfolio:**
- `clinical-records`: Access to Epic EHR patient data
- `lab-results`: Laboratory information system queries
- `billing-data`: Patient billing and insurance information
- `scheduling`: Appointment and resource scheduling

**Key Challenges:**
- Different roles need different access levels
- PHI access requires enhanced authentication
- Audit trails must satisfy HIPAA requirements
- Emergency "break-glass" access is needed for critical situations
:::

::: prompt
aspects:
  - Azure AD OAuth integration configuration
  - RBAC mapping from Azure AD groups to MCP permissions
  - MFA requirements for PHI-accessing tools
  - Audit trail implementation for HIPAA Article 30
  - Break-glass emergency access procedures
hints:
  - Consider Azure AD Conditional Access policies
  - Think about tool-level permission granularity
  - Address session timeout for clinical workstations

Design the authentication and authorization architecture for
Valley Health Partners' MCP implementation. Address each aspect
listed above, explaining your reasoning and trade-offs.
:::

::: hint level=1 title="Start with Identity Integration"
Begin by considering how Azure AD connects to MCP:
- What OAuth flow is appropriate for healthcare?
- How do you map AD groups to MCP roles?
- What claims should be included in tokens?
:::

::: hint level=2 title="HIPAA Access Controls"
HIPAA Security Rule (45 CFR 164.312) requires:
- Unique user identification
- Emergency access procedures
- Automatic logoff
- Audit controls

How does each of these map to MCP features?
:::

::: evaluation
min_words: 200
max_words: 600
pass_threshold: 0.7

criteria:
  - name: SSO Integration
    weight: 25
    description: Correctly describes Azure AD OAuth integration
  - name: RBAC Design
    weight: 25
    description: Maps Azure AD groups to appropriate tool permissions
  - name: MFA Implementation
    weight: 20
    description: Addresses step-up MFA for sensitive operations
  - name: Audit Trail
    weight: 20
    description: Explains logging for HIPAA compliance
  - name: Emergency Access
    weight: 10
    description: Addresses break-glass procedures

key_points:
  - OAuth 2.0 / OIDC integration with Azure AD as identity provider
  - Azure AD groups mapped to MCP roles (Physicians → clinical_full)
  - Tool-level permissions (clinical-records requires Physicians role)
  - Step-up MFA for tools accessing PHI (require_mfa = true)
  - Session management with clinical shift-based timeout
  - Audit logs include user identity, timestamp, tool called, data accessed
  - Break-glass emergency access with enhanced logging
  - Compliance with HIPAA Security Rule access control requirements
:::

::: sample-answer reveal=never
expected_score: 0.85

**OAuth Integration with Azure AD**

Valley Health Partners should implement OAuth 2.0 / OIDC integration
with Azure AD as the identity provider:

```toml
[auth]
type = "oauth2"
provider = "azure_ad"

[auth.azure_ad]
tenant_id = "valley-health-tenant"
client_id_env = "AZURE_CLIENT_ID"
client_secret_env = "AZURE_CLIENT_SECRET"
```

**RBAC Design**

Map existing Azure AD groups to MCP roles with appropriate tool access:

| Azure AD Group | MCP Role | Tool Access |
|----------------|----------|-------------|
| Physicians | clinical_full | clinical-records, lab-results, scheduling |
| Nurses | clinical_limited | lab-results (read), scheduling |
| Billing-Staff | financial_only | billing-data |
| IT-Admin | admin | All (for support) |

**MFA for PHI**

Configure step-up MFA for tools that access Protected Health Information:

```toml
[[auth.tool_permissions]]
tool = "clinical-records.*"
require_mfa = true
roles = ["clinical_full", "clinical_limited"]
```

**Audit Trail**

Every access is logged with:
- User identity (from Azure AD)
- Timestamp (UTC)
- Tool called
- Parameters (excluding PHI values)
- Response status

Logs are retained for 6 years per HIPAA requirements.

**Break-Glass Access**

Emergency access procedure for critical situations:
1. Separate "Emergency Access" role in Azure AD
2. Requires additional authentication factor
3. Triggers enhanced audit logging
4. Automatic security team notification
5. 30-minute session maximum
:::

::: context
**Key Learning Points:**

Healthcare SSO implementations have specific requirements:

1. **Azure AD Integration**: Most healthcare organizations use Azure AD.
   The configuration must specify tenant, client credentials (from env vars),
   and appropriate scopes.

2. **Group-to-Role Mapping**: Leverage existing AD group structure rather
   than creating parallel permission systems. This reduces administrative
   burden and keeps identity management centralized.

3. **MFA for PHI**: HIPAA requires strong authentication for PHI access.
   Step-up MFA (additional auth for sensitive operations) balances security
   with clinical workflow.

4. **Audit for HIPAA**: Every access must be logged for the required
   6-year retention period. Logs should include who, what, when, and
   ideally why (context from the request).

5. **Break-Glass**: Clinical emergencies require immediate data access.
   The break-glass procedure provides access with enhanced auditing
   and automatic review.

The key insight: Security and usability are not opposites. Well-designed
authentication enables clinicians while maintaining compliance.
:::
```

## HTML Rendering

### Structure

```html
<article class="usecase-exercise"
         data-exercise-id="b4uc-001-healthcare-sso"
         data-domain="healthcare"
         data-difficulty="intermediate">

  <header class="exercise-header">
    <h2 class="exercise-title">Exercise: Healthcare SSO and HIPAA Compliance</h2>
    <div class="exercise-meta">
      <span class="domain healthcare">Healthcare</span>
      <span class="difficulty intermediate">Intermediate</span>
      <span class="time">45 minutes</span>
    </div>
  </header>

  <section class="exercise-description">
    <p>Design authentication for a healthcare organization with HIPAA requirements.</p>
  </section>

  <section class="exercise-objectives">
    <!-- Same as code exercises -->
  </section>

  <section class="exercise-scenario">
    <h3>Scenario</h3>
    <div class="scenario-header">
      <span class="organization">Valley Health Partners</span>
      <span class="industry">Healthcare</span>
    </div>
    <div class="scenario-content">
      <!-- Rendered markdown -->
    </div>
    <div class="scenario-constraints">
      <h4>Constraints</h4>
      <ul>
        <li>Must integrate with existing Azure AD</li>
        <!-- ... -->
      </ul>
    </div>
  </section>

  <section class="exercise-prompt">
    <h3>Your Task</h3>
    <div class="prompt-content">
      <!-- Prompt markdown -->
    </div>
    <div class="prompt-aspects">
      <h4>Address these aspects:</h4>
      <ul>
        <li>Azure AD OAuth integration configuration</li>
        <!-- ... -->
      </ul>
    </div>
  </section>

  <section class="exercise-hints">
    <!-- Same as code exercises but with strategic content -->
  </section>

  <section class="exercise-response">
    <h3>Your Response</h3>
    <div class="response-requirements">
      <span class="word-count">200-600 words</span>
    </div>
    <textarea class="response-editor"
              id="response-b4uc-001"
              placeholder="Enter your analysis here..."
              data-min-words="200"
              data-max-words="600"></textarea>
    <div class="response-meta">
      <span class="current-word-count">0 words</span>
    </div>
    <div class="response-actions">
      <button class="btn-submit" data-exercise-id="b4uc-001">
        Submit for Evaluation
      </button>
    </div>
  </section>

  <section class="exercise-evaluation" hidden>
    <!-- Populated after submission -->
    <h3>Evaluation Results</h3>
    <div class="overall-score"></div>
    <div class="criterion-scores"></div>
    <div class="feedback"></div>
  </section>

  <section class="exercise-context" hidden>
    <!-- Shown after submission -->
    <h3>Key Learning Points</h3>
    <div class="context-content">
      <!-- Rendered markdown -->
    </div>
  </section>

  <footer class="exercise-footer">
    <div class="progress-indicator">
      <input type="checkbox" id="complete-b4uc-001">
      <label for="complete-b4uc-001">Mark Complete</label>
    </div>
  </footer>
</article>
```

### CSS Classes

New CSS classes for UseCase exercises:

```css
/* Domain badges */
.domain.healthcare { background: #e8f5e9; color: #2e7d32; }
.domain.defense { background: #e3f2fd; color: #1565c0; }
.domain.financial { background: #fff3e0; color: #ef6c00; }
.domain.general { background: #f5f5f5; color: #616161; }

/* Scenario section */
.exercise-scenario { /* ... */ }
.scenario-header { /* ... */ }
.scenario-constraints { /* ... */ }

/* Response area */
.exercise-response { /* ... */ }
.response-editor {
    min-height: 300px;
    font-family: inherit; /* Not monospace */
}
.current-word-count { /* ... */ }

/* Evaluation results */
.exercise-evaluation { /* ... */ }
.overall-score { /* ... */ }
.criterion-scores { /* ... */ }
.criterion-score.passed { /* ... */ }
.criterion-score.needs-work { /* ... */ }
```

## MCP Server Integration

### Tools

```rust
/// List available UseCase exercises
pub fn list_usecase_exercises(
    part_id: Option<String>,
    domain: Option<UseCaseDomain>,
) -> Vec<UseCaseExerciseSummary>;

/// Get a specific UseCase exercise with full details
pub fn get_usecase_exercise(
    exercise_id: String,
) -> UseCaseExercise;

/// Submit a response for evaluation
pub fn submit_usecase_response(
    exercise_id: String,
    response: String,
) -> UseCaseEvaluation;

/// Get hints for an exercise (respects hint policies)
pub fn get_usecase_hint(
    exercise_id: String,
    level: u8,
) -> Result<UseCaseHint, HintError>;
```

### AI Instructions File

UseCase exercises use a similar `.ai.toml` companion file:

```toml
# b4uc-001-healthcare-sso.ai.toml

[metadata]
exercise_file = "b4uc-001-healthcare-sso.md"
exercise_type = "usecase"
version = "1.0"

[instructions]
role = """
You are a senior solutions architect helping a partner consultant
understand healthcare MCP implementations. Guide them through the
analysis, asking clarifying questions and ensuring they consider
all aspects of the problem.
"""

approach = """
1. SCENARIO UNDERSTANDING (3-5 minutes)
   - Ensure they understand the healthcare context
   - Clarify HIPAA requirements if needed
   - Discuss stakeholder concerns

2. ANALYSIS PHASE (15-20 minutes)
   - Guide them through each aspect
   - Ask about trade-offs they're considering
   - Encourage thinking about edge cases

3. REVIEW PHASE (5-10 minutes)
   - Review their response before submission
   - Suggest areas to strengthen
   - Check word count requirements

4. POST-SUBMISSION (5 minutes)
   - Discuss the evaluation results
   - Walk through the context/learning points
   - Connect to broader certification topics
"""

[evaluation]
# LLM-as-Judge system prompt for evaluation
evaluator_prompt = """
You are evaluating a partner certification response for a healthcare
MCP implementation scenario. Score the response against each criterion
using the rubric provided. Be fair but rigorous - this is professional
certification.

Consider:
- Technical accuracy of the proposed solution
- Completeness of coverage for each aspect
- Understanding of healthcare/HIPAA requirements
- Practical implementability of the design
"""

# Score calibration
calibration_samples = [
    { file = "samples/b4uc-001-high.md", expected_score = 0.90 },
    { file = "samples/b4uc-001-passing.md", expected_score = 0.75 },
    { file = "samples/b4uc-001-failing.md", expected_score = 0.55 },
]

[feedback]
on_pass = """
Excellent analysis! You've demonstrated solid understanding of healthcare
MCP authentication requirements. Your response covered the key aspects
and showed awareness of the trade-offs involved.
"""

on_near_pass = """
Good effort! Your response shows understanding of the core concepts,
but some areas need more depth. Review the feedback on specific
criteria and consider how to strengthen those areas.
"""

on_fail = """
This response needs more work before you're ready for certification.
Review the context section to understand the key learning points,
then try the exercise again with those insights in mind.
"""
```

## Implementation Phases

### Phase 1: Core Types and Parser

1. Add new types to `types.rs`:
   - `UseCaseExercise`, `UseCaseMetadata`, `UseCaseDomain`
   - `Scenario`, `UseCasePrompt`, `EvaluationCriteria`
   - `UseCaseEvaluation`, `CriterionScore`

2. Extend parser in `parser.rs`:
   - Parse `::: usecase` directive
   - Parse `::: scenario` directive
   - Parse `::: prompt` directive
   - Parse `::: evaluation` directive
   - Parse `::: sample-answer` directive
   - Parse `::: context` directive

3. Add detection logic:
   - `is_usecase_exercise()` based on `::: usecase` presence
   - `parse_usecase_exercise()` function

### Phase 2: HTML Renderer

1. Add rendering for UseCase exercises:
   - Scenario section with organization, constraints
   - Prompt section with aspects
   - Response textarea (not code editor)
   - Word count display
   - Submit button

2. Add UseCase-specific CSS:
   - Domain badges
   - Scenario styling
   - Response area (prose, not code)
   - Evaluation results display

3. Add JavaScript:
   - Word count tracking
   - Submit handler (calls MCP server)
   - Evaluation display
   - Context reveal after submission

### Phase 3: MCP Server Integration

1. Built-in mdbook-course server support:
   - Load UseCase exercises from `.md` files
   - Load evaluation criteria
   - Implement `submit_usecase_response` tool
   - Implement LLM-as-Judge evaluation

2. AI instructions support:
   - Load `.ai.toml` companion files
   - Evaluator prompt construction
   - Calibration sample loading

### Phase 4: LLM-as-Judge Implementation

1. Evaluation prompt construction:
   - Include criteria with weights
   - Include key points
   - Include sample answers for calibration
   - Structured output format

2. Response parsing:
   - Extract scores per criterion
   - Extract key point coverage
   - Extract qualitative feedback
   - Calculate overall score

3. Feedback generation:
   - Based on score thresholds
   - Criterion-specific feedback
   - Improvement suggestions

## Migration from Quiz Format

Existing UseCase questions in quiz TOML files should be converted to the
exercise format:

**Before (quiz TOML):**
```toml
[[questions]]
type = "UseCase"
id = "b4uc-001-healthcare-sso"
domain = "healthcare"
scenario = "..."
prompt.prompt = "..."
prompt.hints = [...]
evaluation.criteria = [...]
evaluation.key_points = [...]
```

**After (exercise Markdown):**
```markdown
# Exercise: Healthcare SSO

::: usecase
id: b4uc-001-healthcare-sso
domain: healthcare
...
:::

::: scenario
...
:::

::: prompt
...
:::

::: evaluation
...
:::
```

A migration script can automate this conversion.

## Appendix: Domain-Specific Considerations

### Healthcare

- HIPAA compliance requirements
- PHI protection and access controls
- Clinical workflow considerations
- EHR integration patterns (Epic, Cerner)
- Break-glass emergency access

### Defense

- Classification levels (UNCLASSIFIED through TS/SCI)
- Air-gapped environments
- Multi-level security (MLS)
- CMMC/NIST 800-171 compliance
- Clearance-based access control

### Financial

- Real-time data requirements
- SEC/FINRA compliance
- Shadow AI governance
- Audit trail requirements
- Multi-jurisdictional considerations

### General Enterprise

- SSO integration patterns
- Multi-tenant deployments
- Data privacy (GDPR, CCPA)
- Change management
- Organizational adoption
