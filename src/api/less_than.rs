use crate::Dinero;

/// Check whether the value of a Dinero object is less than another.
///
/// **You can only compare objects that share the same currency.** The function also normalizes objects to the same scale (the highest) before comparing them.
///
/// The `Dinero` struct implements `Ord` trait. You can directly comparison operators:
///
/// ```rust
/// use dinero::{currencies::EUR, Dinero};
///
/// let result = Dinero::new(20, EUR, None) < Dinero::new(10, EUR, None);
/// ```
pub fn less_than(a: &Dinero, b: &Dinero) -> bool {
    a < b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::EUR;

    #[test]
    fn test_less_than() {
        assert!(less_than(
            &Dinero::new(500, EUR, None),
            &Dinero::new(800, EUR, None)
        ));
    }

    #[test]
    fn test_less_than_false() {
        assert!(!less_than(
            &Dinero::new(800, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn test_less_than_equal() {
        assert!(!less_than(
            &Dinero::new(500, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn test_less_than_scale() {
        assert!(less_than(
            &Dinero::new(5000, EUR, Some(3)),
            &Dinero::new(800, EUR, None),
        ));
    }
}
