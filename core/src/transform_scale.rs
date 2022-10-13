use crate::Dinero;

pub fn transform_scale(item: &Dinero, new_scale: isize) -> Dinero {
    let scale = item.scale;
    let amount = item.amount;
    let currency = item.currency;

    let is_new_scale_larger = new_scale > scale;
    let new_amount: isize;
    let factor: isize;

    if is_new_scale_larger {
        let pow = (new_scale - scale) as u32;
        factor = currency.base.pow(pow);
        new_amount = amount * factor;
    } else {
        let pow = (scale - new_scale) as u32;
        factor = currency.base.pow(pow);
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
