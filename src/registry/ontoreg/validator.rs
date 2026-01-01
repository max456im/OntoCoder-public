```rust
//! Валидация формы перед регистрацией в реестре.

use crate::metadata::RegistryMetadata;
use crate::super::super::compiler::ontoc::parser::OntoForm;

pub fn validate_form_for_registry(form: &OntoForm, jurisdiction: &str) -> Result<RegistryMetadata, String> {
    // 1. Проверка AENGA (повторная, на случай подмены)
    if !form.meta.aenga_binding ||
       !form.ethics.aenga_core.dignity_preservation ||
       !form.ethics.aenga_core.autonomy_respect ||
       !form.ethics.aenga_core.non_instrumentalization {
        return Err("Registration denied: AENGA core violation".to_string());
    }

    // 2. Проверка SGCL
    for phase in &form.ontology.phases {
        if phase.density_score < 0.7 {
            return Err("Registration denied: SGCL density too low".to_string());
        }
    }

    // 3. Проверка юрисдикции
    if !["CN", "BR", "ZA", "DEFAULT"].contains(&jurisdiction) {
        return Err("Registration denied: unsupported CLA jurisdiction".to_string());
    }

    // 4. Проверка SGRL
    if form.runtime.sgRL_protection != "enabled" {
        return Err("Registration denied: SGRL protection must be enabled".to_string());
    }

    let meta = RegistryMetadata::new_from_form(form, jurisdiction);
    Ok(meta)
}
```