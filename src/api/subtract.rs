use crate::{messages::UNEQUAL_CURRENCIES_MESSAGE, Dinero};
use std::error::Error;

use super::normalize_scale_tuple::normalize_scale_tuple;

/// Subtract two Dineros
///
/// **You can only subtract objects that share the same currency.** The function also normalizes objects to the same scale (the highest) before subtracting them.
///
/// Example
/// ```rust
/// subtract(&Dinero::new(100, EUR, None), &Dinero::new(200, EUR, None));
/// ```
pub fn subtract(a: &Dinero, b: &Dinero) -> Result<Dinero, Box<dyn Error>> {
    if a.currency.code != b.currency.code {
        Err(UNEQUAL_CURRENCIES_MESSAGE.to_owned())?
    }

    let (an, bn) = normalize_scale_tuple(*a, *b);

    Ok(Dinero {
        currency: an.currency,
        scale: an.scale,
        amount: an.amount - bn.amount,
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
        let result = subtract(&Dinero::new(1, EUR, None), &Dinero::new(2, USD, None));

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!("{:?}", UNEQUAL_CURRENCIES_MESSAGE)
            ),
            _ => panic!("subtract should not return value for unequal currencies"),
        }
    }
}
