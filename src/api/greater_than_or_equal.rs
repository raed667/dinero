use crate::Dinero;

/// greater than or equals
///
/// Check whether the value of a Dinero object is greater than or equals another.
///
/// **You can only compare objects that share the same currency.** The function also normalizes objects to the same scale (the highest) before comparing them.
///
/// Example
/// ```rust
/// let result = greater_than_or_equal(&Dinero::new(20, EUR, None), &Dinero::new(10, EUR, None));
///
/// ```
/// The `Dinero` struct implements `Ord` trait. You can directly comparison operators:
///
/// ```rust
/// let result = Dinero::new(20, EUR, None) >= Dinero::new(10, EUR, None);
/// ```
///
pub fn greater_than_or_equal(a: &Dinero, b: &Dinero) -> bool {
    a >= b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::EUR;

    #[test]
    fn greater_than_or_equal_true() {
        assert!(greater_than_or_equal(
            &Dinero::new(800, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn greater_than_or_equal_false() {
        assert!(!greater_than_or_equal(
            &Dinero::new(500, EUR, None),
            &Dinero::new(800, EUR, None)
        ));
    }

    #[test]
    fn greater_than_or_equal_equal() {
        assert!(greater_than_or_equal(
            &Dinero::new(500, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn greater_than_or_equal_scale() {
        assert!(greater_than_or_equal(
            &Dinero::new(800, EUR, None),
            &Dinero::new(5000, EUR, Some(3))
        ));
    }
}
