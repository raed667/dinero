use crate::{
    messages::{INVALID_INPUT, UNEQUAL_CURRENCIES_MESSAGE},
    normalize_scale::normalize_scale,
    Dinero,
};
use std::error::Error;

pub fn add(a: &Dinero, b: &Dinero) -> Result<Dinero, Box<dyn Error>> {
    if a.currency.code != b.currency.code {
        Err(UNEQUAL_CURRENCIES_MESSAGE.to_owned())?
    }

    let normalized = normalize_scale(vec![a.to_owned(), b.to_owned()]);

    let an_opt = normalized.get(0);
    let bn_opt = normalized.get(1);

    let an = match an_opt {
        Some(an) => an,
        None => panic!("{}", INVALID_INPUT),
    };

    let bn = match bn_opt {
        Some(bn) => bn,
        None => panic!("{}", INVALID_INPUT),
    };

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
    use currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_add() -> Result<(), Box<dyn Error>> {
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
    fn test_add_currency_check() {
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
