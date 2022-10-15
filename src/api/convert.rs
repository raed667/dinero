use crate::currencies::Currency;
use crate::Dinero;

use super::transform_scale;

// The conversion rate from a Dinero to another currency
pub struct Rate {
    pub currency: Currency,
    pub amount: i128,
    pub scale: u32,
}

impl Rate {
    pub fn new(currency: Currency, amount: i128, scale: Option<u32>) -> Rate {
        Rate {
            scale: scale.unwrap_or_else(|| 0),
            amount,
            currency,
        }
    }
}

/// Convert a Dinero object from a currency to another.
///
/// When using scaled amounts, the function converts the returned object to the safest scale.
pub fn convert(item: &Dinero, rate: Rate) -> Dinero {
    if item.currency.eq(&rate.currency) {
        item.to_owned()
    } else if item.amount == 0 || rate.amount == 0 {
        Dinero::new(0, rate.currency, None)
    } else {
        let scale = item.scale;
        let amount = item.amount;
        let new_scale = scale + rate.scale;
        let conversion_scale = u32::max(new_scale, rate.currency.exponent);

        transform_scale(
            &Dinero::new(
                amount * rate.amount,
                rate.currency.to_owned(),
                Some(new_scale),
            ),
            conversion_scale,
        )
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::{EUR, IQD, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_scale() {
        assert_eq!(
            convert(&Dinero::new(500, USD, None), Rate::new(EUR, 89, Some(2))),
            Dinero::new(44500, EUR, Some(4))
        );
    }

    #[test]
    fn test_convert_no_scale() {
        assert_eq!(
            convert(&Dinero::new(500, USD, None), Rate::new(IQD, 1199, None)),
            Dinero::new(5995000, IQD, Some(3))
        );
    }

    #[test]
    fn test_convert_same_currency() {
        assert_eq!(
            convert(&Dinero::new(100, EUR, None), Rate::new(EUR, 300, None)),
            Dinero::new(100, EUR, None)
        );
    }

    #[test]
    fn test_convert_value_zero() {
        assert_eq!(
            convert(&Dinero::new(0, EUR, None), Rate::new(USD, 300, None)),
            Dinero::new(0, USD, None)
        );
    }

    #[test]
    fn test_convert_target_zero() {
        assert_eq!(
            convert(&Dinero::new(110, EUR, None), Rate::new(USD, 0, None)),
            Dinero::new(0, USD, None)
        );
    }
}
