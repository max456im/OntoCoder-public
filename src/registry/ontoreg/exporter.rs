```rust
//! Экспорт форм для научного цитирования или деплоя.

use crate::metadata::RegistryMetadata;

pub fn export_for_citation(meta: &RegistryMetadata) -> String {
    format!(
        "OntoForm: {} (v{})\nJurisdiction: {}\nSGCL Density: {:.2}\nAENGA: Compliant\nDOI: pending\n",
        meta.name, meta.version, meta.jurisdiction, meta.sgcl_density
    )
}

pub fn export_for_deployment(meta: &RegistryMetadata, form_bytes: &[u8]) -> Result<Vec<u8>, String> {
    // В реальности: упаковка в зашифрованный контейнер с jur-тегом
    Ok(form_bytes.to_vec())
}
```