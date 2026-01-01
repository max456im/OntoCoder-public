```rust
use crate::parser::OntoForm;

const SUPPORTED_JURISDICTIONS: &[&str] = &["CN", "BR", "ZA", "DEFAULT"];

pub fn validate_cla(form: &OntoForm) -> Result<(), String> {
    let jur = &form.meta.cla_jurisdiction;

    if !SUPPORTED_JURISDICTIONS.contains(&jur.as_str()) {
        return Err(format!(
            "CLA violation: unsupported jurisdiction '{}'. Supported: {:?}",
            jur, SUPPORTED_JURISDICTIONS
        ));
    }

    // Проверка: sgRL_protection == "enabled"
    if form.runtime.sgRL_protection != "enabled" {
        return Err("CLA/SGRL violation: sgRL_protection must be 'enabled'".to_string());
    }

    println!("[ontoc/CLA] ✓ Jurisdiction '{}' accepted", jur);
    Ok(())
}
```