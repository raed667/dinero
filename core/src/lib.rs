use currencies::Currency;

use crate::have_same_amount::have_same_amount;
use crate::have_same_currency::have_same_currency;

pub mod add;
pub mod allocate;
pub mod equal;
pub mod have_same_amount;
pub mod have_same_currency;
pub mod is_negative;
pub mod is_positive;
pub mod is_zero;
pub mod messages;
pub mod multiply;
pub mod normalize_scale;
pub mod subtract;
pub mod transform_scale;
pub mod trim_scale;
#[derive(Debug, Clone, Copy, Eq)]
pub struct Dinero {
    pub amount: isize, // Make more generic
    pub currency: Currency,
    pub scale: isize,
}

impl Dinero {
    pub fn new(amount: isize, currency: Currency, scale: Option<isize>) -> Dinero {
        Dinero {
            scale: scale.unwrap_or_else(|| currency.exponent.to_owned()),
            amount,
            currency,
        }
    }
}

impl PartialEq for Dinero {
    fn eq(&self, other: &Dinero) -> bool {
        let a = self.to_owned();
        let b = other.to_owned();

        have_same_amount(&[a, b]) && have_same_currency(&[a, b])
    }
}
