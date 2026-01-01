name: Legal Compliance Request — Jurisdictional or Regulatory Alignment
about: Request support for a new law, regulation, or jurisdiction in OntoCoder Public
title: "[LEGAL] Compliance with <Law/Jurisdiction>"
labels: legal, compliance, needs-ethics-review

---

### ⚖️ 1. Primary Jurisdiction or Legal Framework
- Country / Region: _______________ (e.g., India, Nigeria, EU)
- Specific Law / Regulation: _______________  
  (e.g., India’s Digital Personal Data Protection Act, 2023)
- Official Source (URL or citation): _______________

> ❗ Only laws with **clear, enforceable formulations** will be considered.  
> Vague or principle-based frameworks require additional specification.

---

### 2. Required OntoCoder Components
> Select all that apply. Each requires a technical implementation.

- [ ] **New CLA model** (`legal/LICENSE-MODELS/CLA-XX.md`)  
- [ ] **Runtime guard** (`src/core/cla/xx/`)  
- [ ] **Consent validation logic** (AENGA extension)  
- [ ] **Data localization policy** (storage routing)  
- [ ] **User rights enforcement** (e.g., deletion, explanation)  
- [ ] **Audit trail requirement** (immutable logging)  
- [ ] **Cross-border transfer rule**  

---

### 3. Alignment with the Three Laws of Ontogenesis
> Mandatory for acceptance.

- [ ] **Law I**: The law protects human dignity and prohibits instrumentalization.  
- [ ] **Law II**: Compliance does not weaken AENGA/SGRL; it enhances system integrity.  
- [ ] **Law III**: The law contributes to ontological density (e.g., by requiring contextual justification).

> ❌ If any box is unchecked, this request **contradicts the ontological foundation** and will be rejected.

---

### 4. Technical Specification Requested
Provide **exact requirements** (avoid “should” or “must consider”):

| Requirement | OntoCoder Mapping |
|------------|-------------------|
| “Explicit consent for biometric processing” | → `AENGA::autonomy_respect` + `consent_token.scope == "biometric"` |
| “Data of citizens must be stored domestically” | → `runtime.storage_location = jurisdiction.lower()` |
| “Right to explanation of automated decisions” | → `SGCL phase must include causal_network` |

> 📌 **Vague requests (e.g., “support GDPR”) will be closed.**  
> GDPR is already partially covered via CLA-BR/ZA patterns — specify **exact articles**.

---

### 5. Use Case Context
- [ ] Gaming platform (American, with users in this jurisdiction)  
- [ ] Medical diagnostic system  
- [ ] Legal reasoning assistant  
- [ ] Synthetic mind / avatar  
- [ ] Research prototype  
- [ ] Other: _______________

**Why is this urgent?**  
> (e.g., “Platform launches in Mumbai in Q3 2026 and must comply with DPDP Act”)

---

### 6. Proposed Implementation Plan
- Do you offer to contribute a **draft CLA-XX.md**?  
  - [ ] Yes → attach or link  
  - [ ] No  
- Do you have access to **legal expertise** in this jurisdiction?  
  - [ ] Yes → provide contact (optional)  
  - [ ] No

---

### 7. Risks of Non-Compliance
> Help us prioritize.

- [ ] Fines or penalties (specify max: _______________)  
- [ ] Service blocking in jurisdiction  
- [ ] Ethical violation (e.g., enables surveillance)  
- [ ] Scientific reproducibility impaired  

---

> **Note**: Legal compliance in OntoCoder Public is not about “checking boxes”.  
> It is about **encoding juridical truth into architectural invariants**.  
> Requests that reduce law to technicality will be declined.

Этот шаблон:

✅ Требует юридическую точность, а не общие формулировки,
✅ Привязывает каждую норму к онтологическому примитиву,
✅ Отклоняет запросы, нарушающие Три Закона,
✅ Приоритизирует через этический и архитектурный риск,
✅ Превращает право в исполняемый код, а не в “ограничение”.