use crate::{error::DineroError, Dinero};

/// Add up two Dineros.
///
/// **You can only add Dineros that share the same currency.** The function also normalizes objects to the same scale (the highest) before adding them up.
pub fn add(a: &Dinero, b: &Dinero) -> Result<Dinero, DineroError> {
    if a.currency.code != b.currency.code {
        Err(DineroError::UnequalCurrencyError {
            a: Some(a.currency),
            b: Some(b.currency),
        })
    } else {
        Ok(*a + *b)
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
    fn test_add_positive() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(2, EUR, None))?,
            Dinero::new(3, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_negative() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(-3, EUR, None))?,
            Dinero::new(-2, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(0, EUR, None))?,
            Dinero::new(1, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_scale() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(3, EUR, Some(10)))?,
            Dinero::new(100000003, EUR, Some(10))
        );
        Ok(())
    }

    #[test]
    fn test_add_unequal_currency() {
        let a = Dinero::new(1, EUR, None);
        let b = Dinero::new(2, USD, None);
        let result = add(&a, &b);

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
            _ => panic!("add should not return value for unequal currencies"),
        }
    }
}
