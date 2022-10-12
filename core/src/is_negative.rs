use crate::Dinero;

pub fn is_negative(d: &Dinero) -> bool {
    d.amount < 0
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_negative_zero() {
        assert_eq!(is_negative(&Dinero::new(0, USD, None)), false);
    }
    #[test]
    fn test_is_negative_negative_zero() {
        assert_eq!(is_negative(&Dinero::new(-0, USD, None)), false);
    }

    #[test]
    fn test_is_negative_false() {
        assert_eq!(is_negative(&Dinero::new(1, EUR, None)), false);
    }
    #[test]
    fn test_is_negative() {
        assert_eq!(is_negative(&Dinero::new(-1, EUR, None)), true);
    }
    #[test]
    fn test_is_negative_value() {
        assert_eq!(is_negative(&Dinero::new(-10, EUR, None)), true);
    }
}
