use crate::{messages::INVALID_INPUT, normalize_scale::normalize_scale, Dinero};

pub fn have_same_amount(dinero_objects: &[Dinero]) -> bool {
    let normalized = normalize_scale(dinero_objects.to_owned());

    let first_dinero = normalized.get(0).expect(INVALID_INPUT);

    return normalized
        .iter()
        .all(|item| item.amount == first_dinero.amount);
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::{EUR, TND, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_have_same_amount() {
        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(5, EUR, Some(2)),
                Dinero::new(5, USD, Some(2)),
                Dinero::new(5, TND, Some(2))
            ]),
            true
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(0, EUR, None),
                Dinero::new(0, USD, None),
                Dinero::new(0, TND, None)
            ]),
            true
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(-10, EUR, Some(2)),
                Dinero::new(-10, USD, Some(2)),
                Dinero::new(-10, TND, Some(2))
            ]),
            true
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(10, EUR, Some(2)),
                Dinero::new(10, USD, Some(2)),
                Dinero::new(1, TND, Some(2))
            ]),
            false
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(1, EUR, Some(2)),
                Dinero::new(10, USD, Some(2)),
                Dinero::new(10, TND, Some(2))
            ]),
            false
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(1, EUR, Some(2)),
                Dinero::new(10, USD, Some(2)),
                Dinero::new(20, USD, Some(2)),
                Dinero::new(30, USD, Some(2)),
                Dinero::new(10, TND, Some(2))
            ]),
            false
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(10, USD, Some(2)),
                Dinero::new(100, USD, Some(3)),
            ]),
            true
        );
    }

    #[test]
    #[should_panic]
    fn test_have_same_amount_empty() {
        have_same_amount(&vec![]);
    }
}
