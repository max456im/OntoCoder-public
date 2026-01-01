```rust
//! PIPL Guard — специфическая реализация для китайской юрисдикции.

use crate::cla::jurisdiction::DataProcessingContext;

pub struct PIPLGuard;

impl PIPLGuard {
    /// Проверка в соответствии с PIPL (2021)
    pub fn validate(ctx: &DataProcessingContext) -> Result<(), String> {
        // Статья 13: необходимость согласия
        if ctx.data_type == "personal" && !ctx.has_biometric_consent {
            return Err("PIPL Art.13: personal data processing requires consent".to_string());
        }

        // Статья 29: биометрия — отдельное согласие
        if ctx.data_type == "biometric" && !ctx.has_biometric_consent {
            return Err("PIPL Art.29: biometric data requires separate written consent".to_string());
        }

        // Статья 37: трансграничный перенос
        if ctx.storage_location != "cn" {
            return Err("PIPL Art.37: cross-border transfer of CN citizen data prohibited without CAC approval".to_string());
        }

        Ok(())
    }
}
```