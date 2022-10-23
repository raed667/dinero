use std::fmt;

use crate::currencies::Currency;

#[derive(Debug)]
pub enum DineroError {
    UnequalCurrencyError {
        a: Option<Currency>,
        b: Option<Currency>,
    },
    EmptyDinerosError,
    EmptyRatiosError,
    NegativeRatiosError,
    InvalidCurrency,
}

impl std::error::Error for DineroError {}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for DineroError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DineroError::UnequalCurrencyError { a, b } => match (a, b) {
                (Some(x), Some(y)) => write!(
                    f,
                    "Dineros must of be of the same currency. Instead found a={:?}, b={:?}",
                    x, y
                ),
                _ => write!(f, "Dineros must of be of the same currency."),
            },
            DineroError::EmptyDinerosError => write!(f, "Dinero vector can't be empty"),
            DineroError::EmptyRatiosError => write!(f, "Ratio vector can't be empty"),
            DineroError::NegativeRatiosError => {
                write!(f, "Ratio vector can't have a negative value")
            }
            DineroError::InvalidCurrency => write!(f, "Invalid currency"),
        }
    }
}
