use crate::Dinero;

pub fn less_than_or_equal(a: &Dinero, b: &Dinero) -> bool {
    a <= b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::EUR;

    #[test]
    fn less_than_or_equal_true() {
        assert!(less_than_or_equal(
            &Dinero::new(500, EUR, None),
            &Dinero::new(800, EUR, None),
        ));
    }

    #[test]
    fn less_than_or_equal_false() {
        assert!(!less_than_or_equal(
            &Dinero::new(800, EUR, None),
            &Dinero::new(500, EUR, None),
        ));
    }

    #[test]
    fn less_than_or_equal_equal() {
        assert!(less_than_or_equal(
            &Dinero::new(500, EUR, None),
            &Dinero::new(500, EUR, None)
        ));
    }

    #[test]
    fn less_than_or_equal_scale() {
        assert!(less_than_or_equal(
            &Dinero::new(5000, EUR, Some(3)),
            &Dinero::new(800, EUR, None),
        ));
    }
}
