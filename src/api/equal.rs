use crate::Dinero;

/// Equal
///
/// Check whether the value of a Dinero object is equal to another.
///
/// This function does same-value equality, determining whether two Dinero objects are functionally equivalent. It also normalizes objects to the same scale (the highest) before comparing them.
///
/// Example
/// ```rust
/// let result = equal(&Dinero::new(10, EUR, None), &Dinero::new(10, EUR, None));
///
/// ```
/// The `Dinero` struct implements `PartialEq` trait. You can directly use the equality comparison:
///
/// ```rust
/// let result = Dinero::new(10, EUR, None) == Dinero::new(10, EUR, None);
/// ```
///
pub fn equal(a: &Dinero, b: &Dinero) -> bool {
    a == b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::{EUR, USD};

    #[test]
    fn test_equal_same_value_currency() {
        assert!(equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(12, EUR, None)
        ));
    }
    #[test]
    fn test_equal_zero() {
        assert!(equal(
            &Dinero::new(0, EUR, None),
            &Dinero::new(0, EUR, None)
        ));
    }

    #[test]
    fn test_equal_negative() {
        assert!(equal(
            &Dinero::new(-10, EUR, None),
            &Dinero::new(-10, EUR, None)
        ));
    }
    #[test]
    fn test_equal_different_amount() {
        assert!(!equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(10, EUR, None)
        ));
    }
    #[test]
    fn test_equal_different_negatives() {
        assert!(!equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(-12, EUR, None)
        ));
    }
    #[test]
    fn test_equal_different_currency() {
        assert!(!equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(12, USD, None)
        ));
    }

    #[test]
    fn test_equal_different_scale() {
        assert!(equal(
            &Dinero::new(500, USD, Some(2)),
            &Dinero::new(5000, USD, Some(3))
        ));
    }
}
