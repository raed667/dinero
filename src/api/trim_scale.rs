use crate::Dinero;

use super::transform_scale::transform_scale;

fn count_trailing_zeros(input: i128, base: u32) -> u32 {
    if input == 0 {
        0
    } else {
        let mut count = 0;
        let mut temp = input;
        let base = i128::from(base);

        while temp % base == 0 {
            temp /= base;
            count += 1;
        }

        count
    }
}

/// Trim a Dinero's scale as much as possible, down to the currency exponent.
pub fn trim_scale(item: &Dinero) -> Dinero {
    let base = item.currency.base;
    let exponent = item.currency.exponent;

    let trailing_zeros_length = count_trailing_zeros(item.amount, base);
    let difference = item.scale - trailing_zeros_length;

    let new_scale = difference.max(exponent);

    if new_scale == item.scale {
        return item.to_owned();
    }

    transform_scale(item, new_scale)
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::USD;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_trim_scale() {
        assert_eq!(
            trim_scale(&Dinero::new(0, USD, Some(2))),
            Dinero::new(0, USD, Some(2))
        );

        assert_eq!(
            trim_scale(&Dinero::new(99950, USD, Some(4))),
            Dinero::new(9995, USD, Some(3))
        );

        assert_eq!(
            trim_scale(&Dinero::new(9995, USD, Some(3))),
            Dinero::new(9995, USD, Some(3))
        );
    }
}
