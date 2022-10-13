use crate::Dinero;

pub fn less_than(a: &Dinero, b: &Dinero) -> bool {
    a < b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::EUR;

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
