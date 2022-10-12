use crate::{messages::INVALID_INPUT, normalize_scale::normalize_scale, Dinero};

pub fn multiply(a: &Dinero, amount: isize, scale: Option<isize>) -> Dinero {
    let b = Dinero {
        currency: a.currency,
        amount: 1,
        scale: scale.unwrap_or(a.scale),
    };

    let normalized = normalize_scale(vec![a.to_owned(), b.to_owned()]);

    let an_opt = normalized.get(0);

    let an = match an_opt {
        Some(an) => an,
        None => panic!("{}", INVALID_INPUT),
    };

    Dinero {
        currency: an.currency,
        scale: an.scale,
        amount: an.amount * amount,
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use currencies::EUR;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_multiply() {
        assert_eq!(
            multiply(&Dinero::new(5, EUR, None), 4, None),
            Dinero::new(20, EUR, None)
        );

        assert_eq!(
            multiply(&Dinero::new(5, EUR, None), -4, None),
            Dinero::new(-20, EUR, None)
        );

        assert_eq!(
            multiply(&Dinero::new(5, EUR, None), 0, None),
            Dinero::new(0, EUR, None)
        );

        assert_eq!(
            multiply(&Dinero::new(5, EUR, None), 4, Some(5)),
            Dinero::new(20000, EUR, Some(5))
        );
    }
}
