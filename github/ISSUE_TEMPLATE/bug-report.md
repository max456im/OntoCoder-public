name: Bug Report — Technical or Ethical Violation
about: Report a bug that may affect AENGA, SGRL, SGCL, CLA compliance or system integrity
title: "[BUG] Short description"
labels: bug, needs-triage

---

### ⚠️ Critical: Does this affect ethical or legal compliance?
- [ ] Yes — this issue violates AENGA, SGRL, SGCL, or CLA  
- [ ] No — this is a technical or usability bug  

> If **Yes**, your report will be escalated to the **Ethics Core Team** and treated as high severity.

---

### 1. Affected Component
- [ ] `ontoc` (compiler)  
- [ ] `onto-runtime`  
- [ ] `ontoreg` (registry)  
- [ ] `src/core/aenga`  
- [ ] `src/core/sgRL`  
- [ ] `src/core/sgCL`  
- [ ] `src/core/cla`  
- [ ] Documentation (`docs/`, `laws/`, `legal/`)  
- [ ] Scripts (`verify-aenga.sh`, etc.)  
- [ ] Other: _______________

---

### 2. OntoCoder Public Version
- Version: `v2.0.0-aenga-public` (or commit hash: `abcdef1`)
- Installed via:  
  - [ ] Source build (`build.sh`)  
  - [ ] Release artifact  
  - [ ] Development workspace  

---

### 3. Environment
- OS: (e.g., Ubuntu 22.04, macOS Sonoma, Windows 11)  
- Architecture: (x64, ARM64)  
- Rust version: `rustc --version`  
- Go version (if applicable): `go version`  

---

### 4. Description
Provide a clear and concise description of the bug.

**Expected behavior**:  
> What *should* happen according to the Three Laws of Ontogenesis, AENGA spec, or technical docs.

**Actual behavior**:  
> What *actually* happens. Include error messages, exit codes (e.g., `13` = AENGA panic), or unexpected output.

**Steps to reproduce**:  
```bash
# Minimal reproducible example
ontoc compile broken-form.onto -o out.ontobin
# → exits with code 13 and message: "AENGA violation: ..."


---

Этот шаблон:

✅ **Разделяет технические и этические баги**,  
✅ **Обязывает указать связь с Тремя Законами**,  
✅ **Требует юрисдикционный контекст**,  
✅ **Предотвращает злоупотребление** через ответственность за ложные отчёты,  
✅ **Интегрируется с Ethics Core Team** для критических нарушений.
