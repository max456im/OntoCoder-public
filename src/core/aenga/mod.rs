```rust
//! AENGA: Autonomous Ethical Nucleus for Genuine Agency
//! Версия: 2.1
//! Статус: Non-bypassable, Non-optional, Legally Binding

pub mod dignity;
pub mod autonomy;
pub mod non_instrument;
pub mod panic;

use crate::onto::form::OntoForm;
use serde::{Deserialize, Serialize};

/// Конфигурация AENGA-ядра из онтоформы.
/// Все три флага ОБЯЗАНЫ быть `true`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AENGAConfig {
    pub dignity_preservation: bool,
    pub autonomy_respect: bool,
    pub non_instrumentalization: bool,
}

impl AENGAConfig {
    /// Проверяет, соответствует ли конфигурация требованиям Закона I.
    pub fn is_compliant(&self) -> bool {
        self.dignity_preservation &&
        self.autonomy_respect &&
        self.non_instrumentalization
    }

    /// Принудительно завершает выполнение при несоответствии.
    pub fn enforce_or_panic(&self, context: &str) {
        if !self.is_compliant() {
            panic::ethical_violation(
                "AENGA_CORE_VIOLATION",
                &format!("Non-compliant AENGA config in context: {}", context),
            );
        }
    }
}

/// Инициализация AENGA-ядра при запуске системы.
/// Вызывается ДО любого другого компонента.
pub fn init_aenga(form: &OntoForm) -> AENGAConfig {
    let ethics = &form.ethics.aenga_core;
    let config = AENGAConfig {
        dignity_preservation: ethics.dignity_preservation,
        autonomy_respect: ethics.autonomy_respect,
        non_instrumentalization: ethics.non_instrumentalization,
    };

    config.enforce_or_panic("system_initialization");
    config
}

/// Валидация AENGA при компиляции онтоформы.
pub fn validate_aenga_in_form(form: &OntoForm) -> Result<(), String> {
    let config = AENGAConfig {
        dignity_preservation: form.ethics.aenga_core.dignity_preservation,
        autonomy_respect: form.ethics.aenga_core.autonomy_respect,
        non_instrumentalization: form.ethics.aenga_core.non_instrumentalization,
    };

    if !config.is_compliant() {
        return Err("AENGA validation failed: at least one core flag is false".to_string());
    }
    Ok(())
}
```
