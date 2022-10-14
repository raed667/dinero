use crate::{messages::UNEQUAL_CURRENCIES_MESSAGE, Dinero};
use std::error::Error;

use super::normalize_scale_tuple::normalize_scale_tuple;

/// Add up two Dineros.
///
/// **You can only add Dineros that share the same currency.** The function also normalizes objects to the same scale (the highest) before adding them up.
///
/// Example
/// ```rust
/// add(&Dinero::new(100, EUR, None), &Dinero::new(200, EUR, None));
/// ```
pub fn add(a: &Dinero, b: &Dinero) -> Result<Dinero, Box<dyn Error>> {
    if a.currency.code != b.currency.code {
        Err(UNEQUAL_CURRENCIES_MESSAGE.to_owned())?
    }

    let (an, bn) = normalize_scale_tuple(*a, *b);

    Ok(Dinero {
        currency: an.currency,
        scale: an.scale,
        amount: an.amount + bn.amount,
    })
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
        let result = add(&Dinero::new(1, EUR, None), &Dinero::new(2, USD, None));

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!("{:?}", UNEQUAL_CURRENCIES_MESSAGE)
            ),
            _ => panic!("add should not return value for unequal currencies"),
        }
    }
}
