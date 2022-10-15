use crate::Dinero;

use super::normalize_scale_tuple::normalize_scale_tuple;

/// Multiply a Dinero
///
/// If you need to multiply by a fractional multiplier, you shouldn't use floats, but scaled amounts instead. For example, instead of passing `2.1`, you should pass `{ amount: 21, scale: 1 }`. When using scaled amounts, the function converts the returned objects to the safest scale.
pub fn multiply(a: &Dinero, amount: isize, scale: Option<isize>) -> Dinero {
    let b = Dinero {
        currency: a.currency,
        amount: 1,
        scale: scale.unwrap_or(a.scale),
    };

    let (an, _) = normalize_scale_tuple(*a, b);

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
    use crate::currencies::EUR;
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
