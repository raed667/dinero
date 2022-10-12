use crate::Dinero;

pub fn is_positive(d: &Dinero) -> bool {
    d.amount >= 0
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_positive() {
        assert_eq!(is_positive(&Dinero::new(0, USD, None)), true);
        assert_eq!(is_positive(&Dinero::new(-0, USD, None)), true);
        assert_eq!(is_positive(&Dinero::new(1, EUR, None)), true);
        assert_eq!(is_positive(&Dinero::new(10, EUR, None)), true);

        assert_eq!(is_positive(&Dinero::new(-1, EUR, None)), false);
        assert_eq!(is_positive(&Dinero::new(-10, EUR, None)), false);
    }
}
