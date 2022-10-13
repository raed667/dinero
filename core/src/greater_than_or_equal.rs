use crate::Dinero;

pub fn greater_than_or_equal(a: &Dinero, b: &Dinero) -> bool {
    a >= b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::EUR;

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
