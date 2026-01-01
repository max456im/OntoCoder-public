```rust
//! Разрешение юрисдикции и применение соответствующих правил.

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Jurisdiction {
    CN, // Китай
    BR, // Бразилия
    ZA, // ЮАР
    Default,
}

impl Jurisdiction {
    pub fn from_code(code: &str) -> Result<Self, String> {
        match code.to_uppercase().as_str() {
            "CN" => Ok(Jurisdiction::CN),
            "BR" => Ok(Jurisdiction::BR),
            "ZA" => Ok(Jurisdiction::ZA),
            "DEFAULT" => Ok(Jurisdiction::Default),
            _ => Err(format!("Unknown CLA jurisdiction: {}", code)),
        }
    }

    pub fn requires_local_storage(&self) -> bool {
        matches!(self, Jurisdiction::CN) // PIPL требует хранения в КНР
    }

    pub fn requires_separate_biometric_consent(&self) -> bool {
        matches!(self, Jurisdiction::CN | Jurisdiction::BR | Jurisdiction::ZA)
    }

    pub fn allows_cross_border_transfer(&self) -> bool {
        !matches!(self, Jurisdiction::CN)
    }
}

/// Контекст обработки данных пользователя.
pub struct DataProcessingContext {
    pub user_jurisdiction: Jurisdiction,
    pub storage_location: String, // "cn", "us", "eu", etc.
    pub has_biometric_consent: bool,
    pub data_type: String,
}

/// Проверяет, разрешена ли обработка в данной юрисдикции.
pub fn is_processing_compliant(ctx: &DataProcessingContext) -> Result<(), String> {
    let j = &ctx.user_jurisdiction;

    // Проверка хранения (PIPL)
    if j.requires_local_storage() && ctx.storage_location != "cn" {
        return Err("CLA-CN violation: data of CN citizens must be stored in China".to_string());
    }

    // Проверка согласия на биометрию
    if j.requires_separate_biometric_consent() && ctx.data_type == "biometric" && !ctx.has_biometric_consent {
        return Err("CLA violation: missing explicit biometric consent".to_string());
    }

    Ok(())
}
```