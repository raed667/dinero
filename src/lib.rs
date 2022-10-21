//! # Dinero
//!
//! Dinero lets you express monetary values in Rust. The library is a port of [dinerojs v2](https://v2.dinerojs.com) and uses similar paradigms.
//!
//! Money is complex, and the primitives of the language aren't enough to properly represent it. Dinero is a Rust library that lets you express monetary values, but also perform mutations, conversions, comparisons, formatting, and overall make money manipulation easier and safer in your application.
//!
//! You can perform mutations, conversions, comparisons, format them extensively, and overall make money manipulation in your application easier and safer.
//!
//! ## Getting started
//! You can create Dinero objects with the associated function `Dinero::new()`
//!
//! ```rust
//!use dinero::{api::add, currencies::USD, format::to_unit, Dinero};
//!
//!// Create a Dinero object of value 8.5 USD (the default scale for USD is 2)
//! let d1 = Dinero::new(850, USD, None);
//!// Create a Dinero object of value 5 USD with a custom scale 3
//!let d2 = Dinero::new(5000, USD, Some(3));
//!
//!// Add the 2 Dineros, the value is stored in the result Dinero without modifying d1 and d2
//!let result = add(&d1, &d2);
//!
//! // Or you can use the standard operators
//! // let result = d1 + d2;
//!
//!match result {
//!   Ok(value) => println!("{} USD", to_unit(value, None, None)), // 13.5 USD
//!   Err(_) => println!("Error adding d1+d2"),
//!}
//!```
use std::{cmp::Ordering, ops::Add, ops::Sub};

use api::{have_same_amount, have_same_currency, normalize_scale_tuple};

use crate::currencies::Currency;

pub mod api;
pub mod currencies;
pub mod error;
pub mod format;

#[derive(Debug, Clone, Copy, Eq)]
pub struct Dinero {
    pub amount: i128, // Make more generic
    pub currency: Currency,
    pub scale: u32,
}

#[cfg(not(tarpaulin_include))]
impl Dinero {
    pub fn new(amount: i128, currency: Currency, scale: Option<u32>) -> Dinero {
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

#[cfg(not(tarpaulin_include))]
impl Add for Dinero {
    type Output = Dinero;

    fn add(self, other: Self) -> Self::Output {
        let (an, bn) = normalize_scale_tuple(self, other);

        Dinero {
            currency: an.currency,
            scale: an.scale,
            amount: an.amount + bn.amount,
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl Sub for Dinero {
    type Output = Dinero;

    fn sub(self, other: Self) -> Self::Output {
        let (an, bn) = normalize_scale_tuple(self, other);

        Dinero {
            currency: an.currency,
            scale: an.scale,
            amount: an.amount - bn.amount,
        }
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
