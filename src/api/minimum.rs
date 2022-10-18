use crate::{error::DineroError, Dinero};

use super::normalize_scale::normalize_scale;

/// Get the lowest of the passed Dinero vector.
///
/// **You can only compare objects that share the same currency.** The function also normalizes objects to the same scale (the highest) before comparing them.
pub fn minimum(dinero_objects: Vec<Dinero>) -> Result<Dinero, DineroError> {
    if dinero_objects.is_empty() {
        Err(DineroError::EmptyDinerosError)?
    }
    let currency = dinero_objects.get(0).unwrap().currency;
    let have_same_currency = dinero_objects
        .iter()
        .all(|d| d.currency.code == currency.code);

    if !have_same_currency {
        Err(DineroError::UnequalCurrencyError { a: None, b: None })?
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
                format!(
                    "{:?}",
                    DineroError::UnequalCurrencyError { a: None, b: None }
                )
            ),
            _ => panic!("minimum should not return value for unequal currencies"),
        }
    }

    #[test]
    fn test_add_empty() {
        let result = minimum(vec![]);

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!("{:?}", DineroError::EmptyDinerosError)
            ),
            _ => panic!("minimum should not return value for empty vec"),
        }
    }
}
