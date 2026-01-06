# Exercise: Healthcare HIPAA Compliance

::: usecase
id: b1uc-001-healthcare-hipaa
domain: healthcare
difficulty: intermediate
time: 45 minutes
:::

Analyze how to implement a secure, HIPAA-compliant AI integration for a large hospital system.

::: scenario
organization: Memorial Regional Health System
industry: Healthcare
stakeholders:
  - Chief Information Officer (CIO)
  - Chief Medical Officer (CMO)
  - HIPAA Compliance Officer
constraints:
  - HIPAA compliance required for all PHI access
  - Epic EHR is the system of record
  - No patient data can leave the health system network

**Memorial Regional Health System** is a network of 12 hospitals and 50 outpatient clinics across three states. They employ over 15,000 healthcare workers including physicians, nurses, and administrative staff.

**Current Situation:**
- Physicians are copying patient notes into ChatGPT to generate summaries and referral letters.
- The compliance team discovered 47 incidents of PHI (Protected Health Information) being pasted into public AI services in the past quarter.
- IT has blocked access to consumer AI tools, causing physician satisfaction to plummet.
- Doctors are now using personal devices to access AI, making the problem invisible to IT.
- The CIO is under pressure from both the compliance officer (concerned about HIPAA violations) and the CMO (concerned about physician productivity and burnout).

**Requirements:**
- HIPAA compliance with full audit trails.
- Integration with existing Epic EHR system.
- Role-based access (physicians see different data than nurses or billing staff).
- No PHI should leave the hospital's controlled environment.
- Physicians want natural language access to patient data for clinical decision support.
:::

::: prompt
aspects:
  - How MCP addresses the immediate shadow AI risk
  - The security and compliance architecture
  - Integration approach with Epic EHR
  - How different roles would access the system
  - Business value proposition for the CIO
hints:
  - Consider how MCP keeps data within controlled boundaries
  - Think about OAuth integration with hospital identity systems
  - Consider the audit trail requirements for HIPAA
  - Address how to transition physicians from shadow AI to governed tools

As a pmcp.run partner, how would you propose addressing Memorial Regional Health System's challenges using MCP? Your response should cover each of the aspects listed above.
:::

::: evaluation
min_words: 150
max_words: 500
pass_threshold: 0.7

criteria:
  - name: Security Architecture
    weight: 25
    description: Explains how PHI stays within controlled boundaries, encryption, access controls
  - name: HIPAA Compliance
    weight: 25
    description: Addresses audit trails, data protection, breach prevention
  - name: Integration Strategy
    weight: 20
    description: Describes integration with Epic EHR using OpenAPI or database server types
  - name: Role-Based Access
    weight: 15
    description: Explains RBAC for different healthcare roles
  - name: Business Value
    weight: 15
    description: Articulates ROI, physician satisfaction, risk reduction

key_points:
  - MCP servers run within hospital infrastructure, PHI never leaves controlled environment
  - OAuth/SSO integration with hospital Active Directory for identity
  - Audit logging captures every access - who viewed what patient data, when, why
  - Role-based access: physicians see clinical data, billing sees financial, nurses see care plans
  - OpenAPI server type wraps Epic's FHIR APIs without building custom integration
  - Landing page provides self-service discovery for approved AI capabilities
  - Transition plan to move physicians from shadow AI to governed MCP servers
  - Metrics: reduced compliance incidents, maintained/improved physician productivity
:::

::: sample-answer reveal=never
expected_score: 0.85

For Memorial Regional Health System, I would propose a phased MCP implementation:

**Immediate Shadow AI Mitigation:**
Deploy MCP servers within the hospital's existing infrastructure. Unlike public AI services, MCP servers run inside the hospital's network, so PHI never leaves controlled boundaries. This addresses the compliance team's concerns while giving physicians the AI capabilities they need.

**Security and Compliance Architecture:**
- All MCP servers deploy within the hospital's private cloud/data center.
- OAuth integration with the hospital's Active Directory for single sign-on.
- Every request is logged with user identity, timestamp, data accessed, and purpose.
- These audit logs satisfy HIPAA's access logging requirements.
- Data encryption in transit (TLS 1.3) and at rest (AES-256).

**Epic EHR Integration:**
Use pmcp.run's OpenAPI server type to wrap Epic's FHIR APIs. This provides natural language access to patient data without building custom integrations. The server translates physician queries into FHIR API calls and returns structured responses.

**Role-Based Access:**
- Physicians: Full clinical data access, lab results, imaging, notes.
- Nurses: Care plans, medication administration, vital signs.
- Billing staff: Insurance, coding, financial data only.
- Admin staff: Scheduling, demographics, no clinical data.

This is enforced through RBAC policies that map hospital AD groups to MCP permissions.

**Business Value for CIO:**
- Eliminate HIPAA violation risk from shadow AI (potential fines of $50K-$1.5M per violation).
- Restore physician productivity without sacrificing compliance.
- Full visibility into AI usage across all facilities.
- Demonstrate to the board that AI adoption is intentional and governed.
:::

::: context
**Key Learning Points:**

Healthcare MCP deployments must address:

1. **Data Residency**: PHI must stay within controlled boundaries. MCP's architecture supports this because servers run in your infrastructure, not public cloud.

2. **Audit Requirements**: HIPAA requires logging who accessed what, when, and why. MCP's built-in observability provides these audit trails automatically.

3. **Integration Patterns**: Epic and other EHR systems expose FHIR APIs. The OpenAPI server type can wrap these without custom development.

4. **Gradual Transition**: Don't try to block shadow AI immediately. Provide better alternatives first, then gradually restrict unauthorized tools.

5. **Stakeholder Alignment**: The CIO must balance compliance (HIPAA), operations (physician productivity), and security. MCP addresses all three.
:::
