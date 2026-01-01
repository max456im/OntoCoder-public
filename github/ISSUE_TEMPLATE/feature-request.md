name: Feature Request — Technical or Ethical Enhancement
about: Propose a new capability that aligns with the Three Laws of Ontogenesis and scientific-technical rigor
title: "[FEAT] Short description"
labels: enhancement, needs-ethics-review

---

### ⚖️ 1. Alignment with the Three Laws of Ontogenesis
> **This is mandatory.** Features that contradict the Laws cannot be accepted.

- [ ] **Law I (Dignity)**: This feature upholds human dignity and prohibits instrumentalization.  
- [ ] **Law II (Self-Protection)**: This feature does not weaken AENGA or SGRL; ideally, it strengthens them.  
- [ ] **Law III (Ontological Density)**: This feature adds semantic richness, not empty abstraction.

> ❌ If any box is unchecked, **do not submit** — the feature violates the ontological contract.

---

### 2. Proposed Capability
**What problem does this solve?**  
> Describe the gap in current functionality (e.g., "No support for Indian data protection law").

**What is the proposed solution?**  
> Be specific: new module, CLI flag, runtime behavior, etc.

**Why is this necessary for reasoned systems?**  
> Connect to ethics, science, or real-world compliance (e.g., "PIPL requires...").

---

### 3. Technical Scope
- [ ] New component in `src/core/`  
- [ ] Extension to `ontoc` (compiler)  
- [ ] Enhancement to `onto-runtime`  
- [ ] New CLA jurisdiction (`legal/LICENSE-MODELS/`)  
- [ ] Documentation (`laws/`, `docs/`)  
- [ ] Example (`examples/`)  
- [ ] Other: _______________

**Backward compatibility**:  
- [ ] Fully compatible  
- [ ] Requires minor migration  
- [ ] Breaking change (explain below)

---

### 4. Jurisdictional & Legal Context
- Does this feature target a **specific legal framework**?  
  - [ ] China (PIPL)  
  - [ ] Brazil (LGPD)  
  - [ ] South Africa (POPIA)  
  - [ ] India (PDPB)  
  - [ ] Russia  
  - [ ] Other: _______________  
- Will it require a **new CLA model**?  
  - [ ] Yes → attach draft or reference legislation  
  - [ ] No

---

### 5. Ontological Density Impact
> All features must increase, not dilute, semantic integrity.

- This feature introduces:  
  - [ ] New ethical invariants  
  - [ ] Phase context (e.g., era, narrative)  
  - [ ] Social or cognitive grounding (e.g., MBTI, proximity)  
  - [ ] Causal traceability  
- Estimated SGCL density contribution: ≥ ____ (e.g., +0.1 per form)

---

### 6. Use Case Example
Provide a minimal `.onto` snippet or architectural diagram:

```yaml
# Example: CLA-IN support
meta:
  cla_jurisdiction: "IN"
  aenga_binding: true
ethics:
  aenga_core:
    dignity_preservation: true
    autonomy_respect: true
    non_instrumentalization: true
runtime:
  sgRL_protection: enabled
  data_localization: "in"
  
  
---

Этот шаблон:

✅ **Обязывает обосновывать соответствие Трём Законам**,  
✅ **Требует юрисдикционную и онтологическую конкретику**,  
✅ **Отсеивает «пустые» улучшения**, снижающие SGCL-плотность,  
✅ **Связывает инновацию с этической необходимостью**,  
✅ **Сохраняет OntoCoder как стандарт, а не набор фич**.
