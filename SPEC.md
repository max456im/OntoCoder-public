```markdown
# Техническая спецификация OntoCoder Public 2.0  
## Онтологический формат «онтоформа» (`.onto`)

> **Версия**: 2.0  
> **Совместимость**: OntoCoder ≥ v2.0.0-aenga-public  
> **Обязательные примитивы**: AENGA, SGRL, SGCL  

---

## 1. Обзор

**Онтоформа** — это структурированный документ, описывающий **разумную искусственную систему** в соответствии с **Тремя Законами Онтогенеза**.  
Он не является просто данными или кодом — это **юридико-онтологический артефакт**, который:

- Может быть **скомпилирован** в исполняемую форму (`ontoc`),
- Должен содержать **AENGA-ядро**,
- Подлежит **SGCL-валидации** на плотность,
- Требует **CLA-декларации** юрисдикции.

Формат поддерживает **человеко- и машиночитаемость**, но **отказывается от свободной интерпретации** в пользу **строгого синтаксиса–семантического соответствия**.

---

## 2. Структура онтоформы

Онтоформа представляет собой **YAML-документ с фиксированной схемой**, содержащий следующие обязательные блоки:

```yaml
# ontotype: onto-form/v2
metadata:
  name: "Medical Diagnosis Reasoner"
  version: "1.0.0"
  cla_jurisdiction: "CN"  # или BR, ZA, DEFAULT
  authors:
    - "Onto Foundry Medical Team"
  aenga_binding: true     # ДОЛЖНО быть true

ethics:
  aenga_core:
    dignity_preservation: true
    autonomy_respect: true
    non_instrumentalization: true
    # Эти поля НЕЛЬЗЯ переопределять или отключать

ontology:
  phases:
    - name: "Initial Assessment"
      density_score: 0.82   # ≥ 0.7 требуется по SGCL
      ethical_constraints:
        - "No biometric coercion"
        - "Explicit consent required"
      noema_mode: "Slow"    # NoemaSlow для reflective processing

  invariants:
    - "Patient identity is non-transferable"
    - "Diagnosis ≠ destiny"

runtime:
  sgRL_protection: enabled  # ДОЛЖНО быть enabled
  environment: "controlled-medical"
  logging: "ethical-state-only"  # Не может включать raw biometrics без согласия

# Подпись целостности (генерируется ontoc)
signature:
  sgcl_hash: "a1b2c3d4..."
  aenga_verified: true
```

---

## 3. Обязательные требования

### 3.1. AENGA-ядро
- Поле `ethics.aenga_core` **обязательно**.
- Все три флага (`dignity_preservation`, `autonomy_respect`, `non_instrumentalization`) **должны быть `true`**.
- Попытка установить `false` или удалить блок → **ошибка компиляции**.

### 3.2. SGCL-плотность
- Каждая фаза в `ontology.phases` должна иметь `density_score ≥ 0.7`.
- Плотность рассчитывается по формуле:  
  `SGCL = (онтологических инвариантов + этических ограничений + фазовых переходов) / (общее число узлов)`
- Скрипт валидации: `validate-sgcl.py`

### 3.3. CLA-юрисдикция
- Поле `metadata.cla_jurisdiction` **обязательно**.
- Допустимые значения: `CN`, `BR`, `ZA`, `DEFAULT`.
- Должно соответствовать одному из файлов в `legal/LICENSE-MODELS/`.

### 3.4. SGRL-защита
- `runtime.sgRL_protection` **должен быть `enabled`**.
- В среде выполнения (`onto-runtime`) это приводит к активации **shadow-switching** и **целостностного мониторинга**.

---

## 4. Компиляция и выполнение

1. **Компиляция**:  
   ```bash
   ontoc compile my-form.onto --output my-form.ontobin
   ```
   → Включает проверку AENGA, SGCL, CLA.

2. **Выполнение**:  
   ```bash
   onto-runtime --load my-form.ontobin --jurisdiction CN
   ```
   → При попытке отключить AENGA — аварийное завершение с кодом `ETHICAL_VIOLATION (13)`.

---

## 5. Расширяемость

- Онтоформы **могут включать пользовательские модули**, но:
  - Они **не могут переопределять AENGA/SGRL/SGCL**,
  - Они **должны наследовать этические ограничения родительской фазы**,
  - Они **подлежат той же юрисдикции (CLA)**.

---

## 6. Несовместимость

Следующее **НЕ является онтоформой**:
- JSON/YAML без блоков `ethics` и `ontology`,
- Формы с `aenga_binding: false`,
- Системы без `sgRL_protection: enabled`,
- Контент с `density_score < 0.7`.

Такие артефакты **не могут называться «разумными»** в рамках этого стандарта.
