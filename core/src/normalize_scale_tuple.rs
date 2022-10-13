use crate::{transform_scale::transform_scale, Dinero};

pub fn normalize_scale_tuple(a: Dinero, b: Dinero) -> (Dinero, Dinero) {
    if a.scale == b.scale {
        return (a.to_owned(), b.to_owned());
    }

    if a.scale > b.scale {
        return (a.to_owned(), transform_scale(&b, a.scale));
    }

    (transform_scale(&a, b.scale), b.to_owned())
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_normalize_scale_tuple() {
        assert_eq!(
            normalize_scale_tuple(
                Dinero::new(10, EUR, Some(2)), //
                Dinero::new(20, EUR, Some(2))
            ),
            (
                Dinero::new(10, EUR, Some(2)), //
                Dinero::new(20, EUR, Some(2))
            )
        );

        assert_eq!(
            normalize_scale_tuple(
                Dinero::new(20, USD, Some(2)), //
                Dinero::new(30, EUR, Some(3))
            ),
            (
                Dinero::new(200, USD, Some(3)), //
                Dinero::new(30, EUR, Some(3))
            )
        );

        assert_eq!(
            normalize_scale_tuple(
                Dinero::new(30, EUR, Some(3)), //
                Dinero::new(20, USD, Some(2)),
            ),
            (
                Dinero::new(30, EUR, Some(3)), //
                Dinero::new(200, USD, Some(3)),
            )
        );
    }
}
