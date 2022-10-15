use crate::Dinero;

/// Check whether a Dinero object is positive.
pub fn is_positive(d: &Dinero) -> bool {
    d.amount >= 0
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_positive_zero() {
        assert_eq!(is_positive(&Dinero::new(0, USD, None)), true);
    }
    #[test]
    fn test_is_positive_negative_zero() {
        assert_eq!(is_positive(&Dinero::new(-0, USD, None)), true);
    }
    #[test]
    fn test_is_positive() {
        assert_eq!(is_positive(&Dinero::new(1, EUR, None)), true);
    }
    #[test]
    fn test_is_positive_value() {
        assert_eq!(is_positive(&Dinero::new(10, EUR, None)), true);
    }
    #[test]
    fn test_is_positive_false() {
        assert_eq!(is_positive(&Dinero::new(-1, EUR, None)), false);
    }
    #[test]
    fn test_is_positive_value_false() {
        assert_eq!(is_positive(&Dinero::new(-10, EUR, None)), false);
    }
}
