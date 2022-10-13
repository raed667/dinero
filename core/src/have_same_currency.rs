use crate::Dinero;

pub fn have_same_currency(dinero_objects: &[Dinero]) -> bool {
    if dinero_objects.is_empty() {
        return true;
    }

    let first_dinero = dinero_objects.get(0).unwrap().to_owned();

    return dinero_objects
        .iter()
        .all(|item| item.currency.code == first_dinero.currency.code);
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_have_same_currency() {
        assert_eq!(
            have_same_currency(&vec![
                Dinero::new(0, EUR, None),
                Dinero::new(-5, EUR, None),
                Dinero::new(5, EUR, None)
            ]),
            true
        );

        assert_eq!(
            have_same_currency(&vec![
                Dinero::new(1, EUR, None),
                Dinero::new(1, EUR, None),
                Dinero::new(1, EUR, None)
            ]),
            true
        );

        assert_eq!(have_same_currency(&vec![Dinero::new(0, EUR, None),]), true);

        assert_eq!(
            have_same_currency(&vec![
                Dinero::new(1, EUR, None),
                Dinero::new(1, USD, None),
                Dinero::new(1, EUR, None)
            ]),
            false
        );
    }

    #[test]
    fn test_have_same_currency_empty() {
        assert!(have_same_currency(&vec![]));
    }
}
