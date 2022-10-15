use crate::Dinero;

/// Check whether the value of a Dinero is zero.
pub fn is_zero(d: &Dinero) -> bool {
    d.amount == 0
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_zero() {
        assert_eq!(is_zero(&Dinero::new(0, USD, None)), true);
        assert_eq!(is_zero(&Dinero::new(0, EUR, None)), true);
    }
    #[test]
    fn test_is_zero_negative() {
        assert_eq!(is_zero(&Dinero::new(-0, USD, None)), true);
    }
    #[test]
    fn test_is_zero_false_positive() {
        assert_eq!(is_zero(&Dinero::new(1, EUR, None)), false);
    }
    #[test]
    fn test_is_zero_false_negative() {
        assert_eq!(is_zero(&Dinero::new(-1, EUR, None)), false);
    }
}
