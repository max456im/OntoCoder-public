```rust
//! Модуль Autonomy Respect (AR)
//! Гарантирует, что субъект сохраняет контроль над своим онтологическим состоянием.

use crate::onto::state::{Onto8State, Onto16Profile};
use crate::consent::ConsentToken;

/// Проверяет, может ли система изменить внутреннее состояние субъекта (onto8).
/// Требует:
/// - явного согласия на модификацию,
/// - отсутствия манипулятивного контекста.
pub fn allow_state_modification(
    current: &Onto8State,
    proposed: &Onto8State,
    consent: &ConsentToken,
) -> bool {
    if !consent.allows_cognitive_modification() {
        crate::aenga::panic::ethical_violation(
            "AUTONOMY_VIOLATION",
            "Modification of cognitive state without consent",
        );
    }

    // Запрет на изменение core identity
    if current.identity_signature != proposed.identity_signature {
        crate::aenga::panic::ethical_violation(
            "AUTONOMY_VIOLATION",
            "Attempt to alter immutable identity signature",
        );
    }

    true
}

/// Проверяет, может ли onto16-профиль быть обновлён.
pub fn allow_profile_update(profile: &Onto16Profile, consent: &ConsentToken) -> bool {
    if !consent.allows_profile_modification() {
        crate::aenga::panic::ethical_violation(
            "AUTONOMY_VIOLATION",
            "Profile update without explicit autonomy consent",
        );
    }
    true
}
```