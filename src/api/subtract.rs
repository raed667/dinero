use crate::{error::DineroError, Dinero};

/// Subtract two Dineros
///
/// **You can only subtract objects that share the same currency.** The function also normalizes objects to the same scale (the highest) before subtracting them.
pub fn subtract(a: &Dinero, b: &Dinero) -> Result<Dinero, DineroError> {
    if a.currency.code != b.currency.code {
        Err(DineroError::UnequalCurrencyError {
            a: Some(a.currency),
            b: Some(b.currency),
        })
    } else {
        Ok(*a - *b)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use std::error::Error;

    use super::*;
    use crate::currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_subtract() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(5, EUR, None), &Dinero::new(2, EUR, None))?,
            Dinero::new(3, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_subtract_negative() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(1, EUR, None), &Dinero::new(-3, EUR, None))?,
            Dinero::new(4, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_subtract_zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(1, EUR, None), &Dinero::new(0, EUR, None))?,
            Dinero::new(1, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_subtract_scale() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(1, EUR, None), &Dinero::new(3, EUR, Some(10)))?,
            Dinero::new(99999997, EUR, Some(10))
        );
        Ok(())
    }

    #[test]
    fn test_subtract_currency_check() {
        let a = Dinero::new(1, EUR, None);
        let b = Dinero::new(2, USD, None);
        let result = subtract(&a, &b);

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!(
                    "{:?}",
                    DineroError::UnequalCurrencyError {
                        a: Some(a.currency),
                        b: Some(b.currency)
                    }
                )
            ),
            _ => panic!("subtract should not return value for unequal currencies"),
        }
    }
}
