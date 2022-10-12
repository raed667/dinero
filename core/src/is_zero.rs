use crate::Dinero;

pub fn is_zero(d: &Dinero) -> bool {
    d.amount == 0
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_zero() {
        assert_eq!(is_zero(&Dinero::new(0, USD, None)), true);
        assert_eq!(is_zero(&Dinero::new(-0, USD, None)), true);
        assert_eq!(is_zero(&Dinero::new(0, EUR, None)), true);

        assert_eq!(is_zero(&Dinero::new(1, EUR, None)), false);
        assert_eq!(is_zero(&Dinero::new(-1, EUR, None)), false);
    }
}
