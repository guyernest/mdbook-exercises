# Exercise: Defense-Grade Air-Gapped MCP

::: usecase
id: b1uc-002-defense-security
domain: defense
difficulty: advanced
time: 60 minutes
:::

Design a multi-level security (MLS) architecture for MCP in an air-gapped environment.

::: scenario
organization: Joint Special Operations Command (JSOC)
industry: Defense
stakeholders:
  - Command Security Officer
  - Lead Systems Architect
  - Mission Intelligence Lead
constraints:
  - No internet connectivity (Air-gapped)
  - Must support UNCLASSIFIED, SECRET, and TOP SECRET data streams
  - Cross-domain solution (CDS) required for data movement
  - Must comply with NIST 800-171 / CMMC Level 5

**JSOC** is deploying a local intelligence synthesis platform in a forward-deployed tactical environment. The system must provide AI-assisted analysis of multiple intelligence feeds while maintaining strict physical and logical separation between classification levels.

**Current Identity Infrastructure:**
- Hardware-based PKI (CAC/PIV cards)
- Local LDAP/Active Directory within each classification enclave
- No connection to external identity providers

**Key Challenges:**
- Large Language Models (LLMs) must run entirely on local GPU hardware.
- Analysts at different clearance levels need to call the same set of analytical tools, but those tools must only access data appropriate for the user's current session enclave.
- Audit logs must be aggregated to a secure security information and event management (SIEM) system across enclaves.
:::

::: prompt
aspects:
  - Local LLM deployment strategy
  - Enclave-based MCP server architecture
  - Multi-level security (MLS) mapping for MCP permissions
  - Cross-domain audit log aggregation
hints:
  - Consider sidecar deployments for LLM inference
  - Think about "Data Diode" patterns for logging
  - Address hardware token (PKI) integration for MCP authentication

Propose a technical architecture for JSOC's air-gapped MCP deployment. Address the challenges of multi-level security and local inference.
:::

::: evaluation
min_words: 200
max_words: 600
pass_threshold: 0.8

criteria:
  - name: MLS Design
    weight: 30
    description: Correctly addresses enclave separation and classification boundaries
  - name: Local Inference
    weight: 25
    description: Proposes realistic strategy for air-gapped LLM execution
  - name: Authentication
    weight: 20
    description: Addresses PKI/Hardware token integration
  - name: Compliance
    weight: 25
    description: Meets NIST 800-171 / CMMC security requirements

key_points:
  - Deployment of quantized LLMs on local NVIDIA/AMD hardware
  - Isolated MCP server instances per classification enclave
  - PKI-based authentication with local CRL/OCSP validation
  - One-way data diodes for SIEM log aggregation from TS to UNCLASS enclaves
  - RBAC mapped to security clearance claims in user certificates
:::

::: context
**Key Learning Points:**

Defense-grade MCP implementations differ from enterprise in several ways:

1. **Local Weights**: You cannot use API-based LLMs. Servers must be configured to talk to local inference endpoints (e.g., vLLM, TGI) running on local GPUs.

2. **Strict Enclaves**: Instead of one large MCP cluster, defense architectures often use replicated instances in each classification enclave to prevent data leakage.

3. **Certificate Auth**: Password-based or public OAuth is rarely allowed. MCP servers must support mTLS or PKI-based identity.

4. **Hardware Constraints**: Tactical environments have power and cooling limits, affecting which model sizes can be realistically deployed.
:::
