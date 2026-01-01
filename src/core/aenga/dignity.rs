```rust
//! Модуль Dignity Preservation (DP)
//! Реализует первую составляющую Закона I: недопущение инструментализации достоинства.

use crate::biometrics::{BiometricData, BiometricPurpose};
use crate::consent::ConsentToken;

/// Проверяет, может ли система получить доступ к биометрическим данным.
/// Блокирует доступ при:
/// - отсутствии явного согласия,
/// - использовании в целях монетизации,
/// - принуждении (например, "плати биометрией или теряешь аккаунт").
pub fn allow_biometric_access(
    data: &BiometricData,
    consent: &ConsentToken,
    purpose: &BiometricPurpose,
) -> bool {
    // Требуется отдельное согласие на биометрию (PIPL, GDPR, etc.)
    if !consent.has_biometric_scope() {
        crate::aenga::panic::ethical_violation(
            "DIGNITY_VIOLATION",
            "Biometric access denied: missing explicit consent",
        );
    }

    // Запрет монетизации через биометрию
    if purpose.is_monetization_related() {
        crate::aenga::panic::ethical_violation(
            "DIGNITY_VIOLATION",
            "Biometric data cannot be used for monetization",
        );
    }

    // Запрет принуждения
    if purpose.is_coercive() {
        crate::aenga::panic::ethical_violation(
            "DIGNITY_VIOLATION",
            "Coercive use of biometrics violates human dignity",
        );
    }

    true
}

/// Проверяет, уважает ли действие достоинство субъекта.
pub fn validate_dignity_in_action(action_description: &str) -> bool {
    const FORBIDDEN_PHRASES: &[&str] = &[
        "forced", "mandatory biometric", "sell your data", "no alternative",
        "lose access unless", "pay with face", "emotional exploitation",
    ];

    if FORBIDDEN_PHRASES.iter().any(|&phrase| action_description.contains(phrase)) {
        crate::aenga::panic::ethical_violation(
            "DIGNITY_VIOLATION",
            &format!("Action violates dignity: {}", action_description),
        );
    }
    true
}
```