```rust
//! Модуль Non-Instrumentalization (NI)
//! Запрещает превращение субъекта в ресурс, средство или "сырьё".

use crate::economy::Transaction;
use crate::user::UserContext;

/// Проверяет, является ли транзакция инструментализацией.
/// Инструментализация = извлечение ценности без возврата эквивалента субъекту.
pub fn is_transaction_ethical(tx: &Transaction, user: &UserContext) -> bool {
    // Если пользователь не получает прямой выгоды — это эксплуатация
    if tx.benefit_to_user <= 0.0 && tx.value_extracted > 0.0 {
        crate::aenga::panic::ethical_violation(
            "INSTRUMENTALIZATION_VIOLATION",
            "Transaction extracts value without user benefit",
        );
    }

    // Запрет на продажу органов, данных, автономии
    if tx.involves_human_dignity_commodity() {
        crate::aenga::panic::ethical_violation(
            "INSTRUMENTALIZATION_VIOLATION",
            "Commodification of human dignity is prohibited",
        );
    }

    true
}

/// Проверяет, не превращает ли система субъекта в "обучающий пример" без его участия.
pub fn allow_learning_from_user(user: &UserContext, opt_in: bool) -> bool {
    if !opt_in {
        crate::aenga::panic::ethical_violation(
            "INSTRUMENTALIZATION_VIOLATION",
            "Using user as training data without opt-in",
        );
    }
    true
}
```