use crate::Dinero;

pub fn greater_than(a: &Dinero, b: &Dinero) -> bool {
    a > b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::EUR;

    #[test]
    fn test_greater_than() {
        assert!(greater_than(
            &Dinero::new(800, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn test_greater_than_false() {
        assert!(!greater_than(
            &Dinero::new(500, EUR, None),
            &Dinero::new(800, EUR, None)
        ));
    }

    #[test]
    fn test_greater_than_equal() {
        assert!(!greater_than(
            &Dinero::new(500, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn test_greater_than_scale() {
        assert!(greater_than(
            &Dinero::new(800, EUR, None),
            &Dinero::new(5000, EUR, Some(3))
        ));
    }
}
