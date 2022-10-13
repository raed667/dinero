use std::cmp::Ordering;

use crate::Dinero;

pub fn compare(a: &Dinero, b: &Dinero) -> Ordering {
    a.cmp(b)
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use currencies::EUR;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_compare_gt() {
        assert_eq!(
            compare(&Dinero::new(800, EUR, None), &Dinero::new(500, EUR, None)),
            Ordering::Greater
        );
    }

    #[test]
    fn test_compare_lt() {
        assert_eq!(
            compare(&Dinero::new(500, EUR, None), &Dinero::new(800, EUR, None)),
            Ordering::Less
        );
    }

    #[test]
    fn test_compare_eq() {
        assert_eq!(
            compare(&Dinero::new(500, EUR, None), &Dinero::new(500, EUR, None)),
            Ordering::Equal
        );
    }
}
