//! # Dinero
//!
//! Dinero lets you express monetary values in Rust. The library is a port of [dinerojs v2](https://v2.dinerojs.com) and uses similar paradigms.
//!
//! Money is complex, and the primitives of the language aren't enough to properly represent it. Dinero is a Rust library that lets you express monetary values, but also perform mutations, conversions, comparisons, formatting, and overall make money manipulation easier and safer in your application.
//!
//! You can perform mutations, conversions, comparisons, format them extensively, and overall make money manipulation in your application easier and safer.
//!
use std::cmp::Ordering;

use api::{have_same_amount, have_same_currency, normalize_scale_tuple};

use crate::currencies::Currency;

pub mod api;
pub mod currencies;
pub mod format;
pub mod messages;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Dinero {
    pub amount: isize, // Make more generic
    pub currency: Currency,
    pub scale: isize,
}

#[cfg(not(tarpaulin_include))]
impl Dinero {
    pub fn new(amount: isize, currency: Currency, scale: Option<isize>) -> Dinero {
        Dinero {
            scale: scale.unwrap_or_else(|| currency.exponent.to_owned()),
            amount,
            currency,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for Dinero {
    fn eq(&self, other: &Dinero) -> bool {
        let a = self.to_owned();
        let b = other.to_owned();

        have_same_amount(&[a, b]) && have_same_currency(&[a, b])
    }
}

#[cfg(not(tarpaulin_include))]
impl PartialOrd for Dinero {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(not(tarpaulin_include))]
impl Ord for Dinero {
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.to_owned();
        let b = other.to_owned();

        let (an, bn) = normalize_scale_tuple(a, b);

        an.amount.cmp(&bn.amount)
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::EUR;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_dinero_new() {
        assert_eq!(
            Dinero::new(42, EUR, Some(2)),
            Dinero {
                amount: 42,
                currency: EUR,
                scale: 2
            }
        );
    }
    #[test]
    fn assert_dinero_eq() {
        assert_eq!(
            Dinero::new(42, EUR, Some(2)), //
            Dinero::new(42, EUR, Some(2))
        );
    }

    #[test]
    fn assert_dinero_neq() {
        assert_ne!(
            Dinero::new(142, EUR, Some(2)), //
            Dinero::new(42, EUR, Some(2))
        );
    }

    #[test]
    fn assert_dinero_gt() {
        assert!(Dinero::new(142, EUR, Some(2)) > Dinero::new(42, EUR, Some(2)));
    }

    #[test]
    fn assert_dinero_gt_eq() {
        assert!(Dinero::new(142, EUR, Some(2)) >= Dinero::new(42, EUR, Some(2)));
    }

    #[test]
    fn assert_dinero_lt() {
        assert!(Dinero::new(1, EUR, Some(2)) < Dinero::new(42, EUR, Some(2)));
    }

    #[test]
    fn assert_dinero_lt_eq() {
        assert!(Dinero::new(1, EUR, Some(2)) <= Dinero::new(42, EUR, Some(2)));
    }
}
