use crate::{normalize_scale::normalize_scale, Dinero};

pub fn have_same_amount(dinero_objects: &[Dinero]) -> bool {
    if dinero_objects.len() == 0 {
        return true;
    }
    let normalized = normalize_scale(dinero_objects.to_owned());

    let first_dinero = normalized.get(0).unwrap().to_owned();

    return normalized
        .iter()
        .all(|item| item.amount == first_dinero.amount);
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::{EUR, TND, USD};

    #[test]
    fn test_have_same_amount_and_currency() {
        assert!(have_same_amount(&vec![
            Dinero::new(5, EUR, Some(2)),
            Dinero::new(5, USD, Some(2)),
            Dinero::new(5, TND, Some(2))
        ]));
    }

    #[test]
    fn test_have_same_amount_different_currency() {
        assert!(have_same_amount(&vec![
            Dinero::new(0, EUR, None),
            Dinero::new(0, USD, None),
            Dinero::new(0, TND, None)
        ]));
    }

    #[test]
    fn test_have_same_amount_negative() {
        assert!(have_same_amount(&vec![
            Dinero::new(-10, EUR, Some(2)),
            Dinero::new(-10, USD, Some(2)),
            Dinero::new(-10, TND, Some(2))
        ]));
    }
    #[test]
    fn test_have_same_amount_diff_end() {
        assert!(!have_same_amount(&vec![
            Dinero::new(10, EUR, Some(2)),
            Dinero::new(10, USD, Some(2)),
            Dinero::new(1, TND, Some(2))
        ]));
    }
    #[test]
    fn test_have_same_amount_diff_start() {
        assert!(!have_same_amount(&vec![
            Dinero::new(1, EUR, Some(2)),
            Dinero::new(10, USD, Some(2)),
            Dinero::new(10, TND, Some(2))
        ]));
    }

    #[test]
    fn test_have_same_amount_diff_mixed() {
        assert!(!have_same_amount(&vec![
            Dinero::new(1, EUR, Some(2)),
            Dinero::new(10, USD, Some(2)),
            Dinero::new(20, USD, Some(2)),
            Dinero::new(30, USD, Some(2)),
            Dinero::new(10, TND, Some(2))
        ]));
    }

    #[test]
    fn test_have_same_amounts_base() {
        assert!(have_same_amount(&vec![
            Dinero::new(10, USD, Some(2)),
            Dinero::new(100, USD, Some(3)),
        ]));
    }

    #[test]
    fn test_have_same_amount_empty() {
        assert!(have_same_amount(&vec![]));
    }
}
