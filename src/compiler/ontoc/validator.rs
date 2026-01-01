```rust
use std::path::Path;
use crate::parser::OntoForm;
use crate::aenga_check;
use crate::sgcl_check;
use crate::cla_check;
use crate::codegen;

/// Основной этап компиляции: валидация → генерация кода.
pub fn validate_and_compile(form: &OntoForm, output_path: &Path) -> Result<(), String> {
    // 1. AENGA: этическое ядро ОБЯЗАТЕЛЬНО и НЕОТКЛЮЧАЕМО
    aenga_check::validate_aenga(form)?;

    // 2. SGCL: онтологическая плотность ≥ 0.7
    sgcl_check::validate_sgcl(form)?;

    // 3. CLA: юрисдикция указана и поддерживается
    cla_check::validate_cla(form)?;

    // 4. Генерация бинарника
    codegen::generate_ontobin(form, output_path)?;

    Ok(())
}
```
