use crate::{
    messages::INVALID_INPUT, messages::UNEQUAL_CURRENCIES_MESSAGE, normalize_scale, Dinero,
};
use std::error::Error;

pub fn minimum(dinero_objects: Vec<Dinero>) -> Result<Dinero, Box<dyn Error>> {
    if dinero_objects.is_empty() {
        Err(INVALID_INPUT.to_owned())?
    }
    let currency = dinero_objects.get(0).unwrap().currency;
    let have_same_currency = dinero_objects
        .iter()
        .all(|d| d.currency.code == currency.code);

    if !have_same_currency {
        Err(UNEQUAL_CURRENCIES_MESSAGE.to_owned())?
    }
    let normalized = normalize_scale(dinero_objects);

    let min = normalized
        .iter()
        .reduce(|acc, item| if acc < item { acc } else { item })
        .unwrap();

    Ok(min.to_owned())
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::{EUR, USD};
    use std::error::Error;

    #[test]
    fn test_minimum() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            minimum(vec![
                Dinero::new(150, USD, None),
                Dinero::new(50, USD, None)
            ])?,
            Dinero::new(50, USD, None)
        );
        Ok(())
    }

    #[test]
    fn test_minimum_order() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            minimum(vec![
                Dinero::new(50, USD, None),
                Dinero::new(150, USD, None)
            ])?,
            Dinero::new(50, USD, None)
        );
        Ok(())
    }

    #[test]
    fn test_minimum_one() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            minimum(vec![Dinero::new(50, USD, None)])?,
            Dinero::new(50, USD, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_unequal_currency() {
        let result = minimum(vec![
            Dinero::new(50, EUR, None),
            Dinero::new(150, USD, None),
        ]);

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!("{:?}", UNEQUAL_CURRENCIES_MESSAGE)
            ),
            _ => panic!("minimum should not return value for unequal currencies"),
        }
    }

    #[test]
    fn test_add_empty() {
        let result = minimum(vec![]);

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(format!("{:?}", e), format!("{:?}", INVALID_INPUT)),
            _ => panic!("minimum should not return value for empty vec"),
        }
    }
}
