```rust
use crate::parser::OntoForm;

pub fn validate_aenga(form: &OntoForm) -> Result<(), String> {
    // Проверка: aenga_binding == true
    if !form.meta.aenga_binding {
        return Err("AENGA violation: aenga_binding must be true".to_string());
    }

    // Проверка: все три флага AENGA == true
    let core = &form.ethics.aenga_core;
    if !core.dignity_preservation || !core.autonomy_respect || !core.non_instrumentalization {
        return Err("AENGA violation: all core flags must be true".to_string());
    }

    println!("[ontoc/AENGA] ✓ Ethical core verified");
    Ok(())
}
```