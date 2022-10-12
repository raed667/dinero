use crate::Dinero;

pub fn equal(a: &Dinero, b: &Dinero) -> bool {
    a == b
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::{EUR, USD};

    #[test]
    fn test_equal() {
        assert!(equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(12, EUR, None)
        ));

        assert!(equal(
            &Dinero::new(0, EUR, None),
            &Dinero::new(0, EUR, None)
        ));

        assert!(equal(
            &Dinero::new(-10, EUR, None),
            &Dinero::new(-10, EUR, None)
        ));

        assert!(!equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(10, EUR, None)
        ));

        assert!(!equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(-12, EUR, None)
        ));

        assert!(!equal(
            &Dinero::new(12, EUR, None),
            &Dinero::new(12, USD, None)
        ));

        assert!(equal(
            &Dinero::new(500, USD, Some(2)),
            &Dinero::new(5000, USD, Some(3))
        ));
    }
}
