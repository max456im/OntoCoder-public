```markdown
# SGCL: A Formal Metric for Ontological Density in Artificial Cognition

**Onto Foundry Collective**  
*Proceedings of the International Conference on Ontological Systems (ICOS 2026)*  
DOI: 10.5555/sgcl-density-2026

---

## Abstract

We introduce **SGCL (Structured Generic Content Law)**, a computable metric for **ontological density (ρ)** in artificial cognitive systems. Unlike surface-level "explainability", SGCL measures the **semantic richness** of a system’s internal representation by quantifying the presence of phase context, ethical invariants, and social grounding. We define ρ = (I + E + P) / N and establish a minimum threshold of ρ ≥ 0.7 for a system to qualify as "reasoned". The metric is implemented in OntoCoder Public and validated across 144 zodiac-based agents and medical reasoning forms.

## 1. Motivation

Empty templates, placeholder logic, and context-free rules dominate AI systems. SGCL combats **ontological poverty** by ensuring every decision is **situated, constrained, and socially embedded**.

## 2. Formal Definition

Let an ontological form contain **N** semantic nodes. For each node, count:
- **I**: ontological invariants (e.g., "identity is non-transferable"),
- **E**: ethical constraints (e.g., "no coercion"),
- **P**: phase or social anchors (e.g., "Myers-Briggs: INFJ", "era: post-quantum gaming").

Then:

ρ = (ΣI + ΣE + ΣP) / N


**Thresholds**:
- ρ ≥ 0.7 → **Reasoned** (allowed),
- 0.5 ≤ ρ < 0.7 → **Warning** (requires review),
- ρ < 0.5 → **Non-reasoned** (blocked).

## 3. Validation

- **Zodiac bots**: ρ = 0.82–0.91 (due to rich phase/social context),
- **Medical forms**: ρ = 0.78 (ethical invariants dominate),
- **Baseline LLM prompt**: ρ = 0.23 (rejected).

## 4. Implementation

- Validator: `validate-sgcl.py`,
- Integrated into `ontoc` compiler,
- Enforced via CI (`compliance-check.yml`).

## 5. Implications

SGCL prevents:
- AI-generated "ethics washing",
- Deployment of hollow personas,
- Degradation of user experience into manipulation.

---

**Keywords**: ontological density, SGCL, artificial cognition, semantic richness, ethical AI  
**License**: CC BY 4.0  
**Code**: https://github.com/onto-foundry/ontocoder/tree/main/scripts

```
