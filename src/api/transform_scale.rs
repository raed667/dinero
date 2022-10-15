use crate::Dinero;

/// Transform a Dinero to a new scale.
///
/// Transforming to a higher scale means that the internal amount value increases by orders of magnitude. Be careful not to exceed the minimum and maximum safe integers.
pub fn transform_scale(item: &Dinero, new_scale: u32) -> Dinero {
    let scale = item.scale;
    let amount = item.amount;
    let currency = item.currency;

    let is_new_scale_larger = new_scale > scale;
    let new_amount: i128;
    let factor: i128;

    if is_new_scale_larger {
        factor = i128::from(currency.base.pow(new_scale - scale));
        new_amount = amount * factor;
    } else {
        factor = i128::from(currency.base.pow(scale - new_scale));
        new_amount = amount / factor;
    }

    Dinero {
        amount: new_amount,
        currency,
        scale: new_scale,
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::EUR;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_transform_scale() {
        assert_eq!(
            transform_scale(&Dinero::new(500, EUR, Some(2)), 4),
            Dinero::new(50000, EUR, Some(4))
        );

        assert_eq!(
            transform_scale(&Dinero::new(3000, EUR, Some(2)), 0),
            Dinero::new(30, EUR, Some(0))
        );
    }
}
