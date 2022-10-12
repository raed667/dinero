use std::error::Error;

use currencies::Currency;
use messages::{INVALID_INPUT, UNEQUAL_CURRENCIES_MESSAGE};

mod messages;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dinero {
    pub amount: i64, // Make more generic
    pub currency: Currency,
    pub scale: i64,
}

impl Dinero {
    pub fn new(amount: i64, currency: Currency, scale: Option<i64>) -> Dinero {
        Dinero {
            scale: scale.unwrap_or(currency.exponent.to_owned()),
            amount,
            currency,
        }
    }
}

fn count_trailing_zeros(input: i64, base: i64) -> i64 {
    if input == 0 {
        return 0;
    }

    let mut i = 0;
    let mut temp = input;

    while temp % base == 0 {
        temp = temp / base;
        i += 1;
    }

    return i;
}

pub fn allocate(item: &Dinero, ratios: Vec<i64>) -> Vec<Dinero> {
    // TODO this is a placeholder implementation
    ratios
        .iter()
        .map(|_ratio| Dinero::new(item.amount, item.currency, Some(item.scale)))
        .collect()
}

pub fn trim_scale(item: &Dinero) -> Dinero {
    let base = item.currency.base;
    let exponent = item.currency.exponent;

    let trailing_zeros_length = count_trailing_zeros(item.amount, base);
    let difference = item.scale - trailing_zeros_length;

    let new_scale = difference.max(exponent);

    if new_scale == item.scale {
        return item.to_owned();
    }

    return transform_scale(item, new_scale);
}

pub fn transform_scale(item: &Dinero, new_scale: i64) -> Dinero {
    let scale = item.scale;
    let amount = item.amount;
    let currency = item.currency;

    let is_new_scale_larger = new_scale > scale;
    let new_amount: i64;
    let factor: i64;

    if is_new_scale_larger {
        let pow = (new_scale - scale) as u32;
        factor = currency.base.pow(pow);
        new_amount = amount * factor;
    } else {
        let pow = (scale - new_scale) as u32;
        factor = currency.base.pow(pow);
        new_amount = amount / factor;
    }

    return Dinero {
        amount: new_amount,
        currency,
        scale: new_scale,
    };
}

pub fn normalize_scale(dinero_objects: Vec<Dinero>) -> Vec<Dinero> {
    let highest_scale = dinero_objects
        .iter()
        .map(|item| item.scale)
        .reduce(|acc, item| if acc >= item { acc } else { item })
        .unwrap_or(0);

    return dinero_objects
        .iter()
        .map(|d| -> Dinero {
            let scale = d.scale;
            if scale == highest_scale {
                return *d;
            }
            return transform_scale(d, highest_scale);
        })
        .collect();
}

pub fn add(a: &Dinero, b: &Dinero) -> Result<Dinero, Box<dyn Error>> {
    if a.currency.code != b.currency.code {
        Err(UNEQUAL_CURRENCIES_MESSAGE.to_owned())?
    }

    let normalized = normalize_scale(vec![a.to_owned(), b.to_owned()]);

    let an_opt = normalized.get(0);
    let bn_opt = normalized.get(1);

    let an = match an_opt {
        Some(an) => an,
        None => panic!("{}", INVALID_INPUT),
    };

    let bn = match bn_opt {
        Some(bn) => bn,
        None => panic!("{}", INVALID_INPUT),
    };

    Ok(Dinero {
        currency: an.currency,
        scale: an.scale,
        amount: an.amount + bn.amount,
    })
}

pub fn subtract(a: &Dinero, b: &Dinero) -> Result<Dinero, Box<dyn Error>> {
    if a.currency.code != b.currency.code {
        Err(UNEQUAL_CURRENCIES_MESSAGE.to_owned())?
    }

    let normalized = normalize_scale(vec![a.to_owned(), b.to_owned()]);

    let an_opt = normalized.get(0);
    let bn_opt = normalized.get(1);

    let an = match an_opt {
        Some(an) => an,
        None => panic!("{}", INVALID_INPUT),
    };

    let bn = match bn_opt {
        Some(bn) => bn,
        None => panic!("{}", INVALID_INPUT),
    };

    Ok(Dinero {
        currency: an.currency,
        scale: an.scale,
        amount: an.amount - bn.amount,
    })
}

pub fn multiply(a: &Dinero, amount: i64, scale: Option<i64>) -> Dinero {
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

pub fn is_zero(d: &Dinero) -> bool {
    d.amount == 0
}

pub fn is_negative(d: &Dinero) -> bool {
    d.amount < 0
}

pub fn is_positive(d: &Dinero) -> bool {
    d.amount >= 0
}

pub fn have_same_amount(dinero_objects: &Vec<Dinero>) -> bool {
    let first_dinero = dinero_objects.get(0).expect(INVALID_INPUT);

    return dinero_objects
        .iter()
        .all(|item| item.amount == first_dinero.amount);
}

pub fn have_same_currency(dinero_objects: &Vec<Dinero>) -> bool {
    let first_dinero = dinero_objects.get(0).expect(INVALID_INPUT);

    return dinero_objects
        .iter()
        .all(|item| item.currency.code == first_dinero.currency.code);
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use currencies::{EUR, TND, USD};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_allocate() {
        assert_eq!(
            allocate(&Dinero::new(500, USD, None), vec![50, 50]),
            vec![Dinero::new(250, USD, None), Dinero::new(250, USD, None)]
        )
    }

    #[test]
    fn test_trim_scale() {
        assert_eq!(
            trim_scale(&Dinero::new(99950, USD, Some(4))),
            Dinero::new(9995, USD, Some(3))
        );
    }

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

    #[test]
    fn test_normalize_scale() {
        assert_eq!(normalize_scale(vec![]), vec![]);

        assert_eq!(
            normalize_scale(vec![Dinero::new(10, EUR, Some(2))]),
            vec![Dinero::new(10, EUR, Some(2))]
        );

        assert_eq!(
            normalize_scale(vec![
                Dinero::new(10, EUR, Some(2)),
                Dinero::new(20, EUR, Some(2))
            ]),
            vec![Dinero::new(10, EUR, Some(2)), Dinero::new(20, EUR, Some(2))]
        );

        assert_eq!(
            normalize_scale(vec![
                Dinero::new(10, EUR, Some(2)),
                Dinero::new(20, USD, Some(2)),
                Dinero::new(30, EUR, Some(3)),
            ]),
            vec![
                Dinero::new(100, EUR, Some(3)),
                Dinero::new(200, USD, Some(3)),
                Dinero::new(30, EUR, Some(3))
            ]
        );
    }

    #[test]
    fn test_is_zero() {
        assert_eq!(is_zero(&Dinero::new(0, USD, None)), true);
        assert_eq!(is_zero(&Dinero::new(-0, USD, None)), true);
        assert_eq!(is_zero(&Dinero::new(0, EUR, None)), true);

        assert_eq!(is_zero(&Dinero::new(1, EUR, None)), false);
        assert_eq!(is_zero(&Dinero::new(-1, EUR, None)), false);
    }

    #[test]
    fn test_is_negative() {
        assert_eq!(is_negative(&Dinero::new(0, USD, None)), false);
        assert_eq!(is_negative(&Dinero::new(-0, USD, None)), false);
        assert_eq!(is_negative(&Dinero::new(1, EUR, None)), false);

        assert_eq!(is_negative(&Dinero::new(-1, EUR, None)), true);
        assert_eq!(is_negative(&Dinero::new(-10, EUR, None)), true);
    }

    #[test]
    fn test_is_positive() {
        assert_eq!(is_positive(&Dinero::new(0, USD, None)), true);
        assert_eq!(is_positive(&Dinero::new(-0, USD, None)), true);
        assert_eq!(is_positive(&Dinero::new(1, EUR, None)), true);
        assert_eq!(is_positive(&Dinero::new(10, EUR, None)), true);

        assert_eq!(is_positive(&Dinero::new(-1, EUR, None)), false);
        assert_eq!(is_positive(&Dinero::new(-10, EUR, None)), false);
    }

    #[test]
    fn test_have_same_amount() {
        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(5, EUR, None),
                Dinero::new(5, USD, None),
                Dinero::new(5, TND, None)
            ]),
            true
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(0, EUR, None),
                Dinero::new(0, USD, None),
                Dinero::new(0, TND, None)
            ]),
            true
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(-10, EUR, None),
                Dinero::new(-10, USD, None),
                Dinero::new(-10, TND, None)
            ]),
            true
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(10, EUR, None),
                Dinero::new(10, USD, None),
                Dinero::new(1, TND, None)
            ]),
            false
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(1, EUR, None),
                Dinero::new(10, USD, None),
                Dinero::new(10, TND, None)
            ]),
            false
        );

        assert_eq!(
            have_same_amount(&vec![
                Dinero::new(1, EUR, None),
                Dinero::new(10, USD, None),
                Dinero::new(20, USD, None),
                Dinero::new(30, USD, None),
                Dinero::new(10, TND, None)
            ]),
            false
        );
    }

    #[test]
    #[should_panic]
    fn test_have_same_amount_empty() {
        have_same_amount(&vec![]);
    }

    #[test]
    fn test_have_same_currency() {
        assert_eq!(
            have_same_currency(&vec![
                Dinero::new(0, EUR, None),
                Dinero::new(-5, EUR, None),
                Dinero::new(5, EUR, None)
            ]),
            true
        );

        assert_eq!(
            have_same_currency(&vec![
                Dinero::new(1, EUR, None),
                Dinero::new(1, EUR, None),
                Dinero::new(1, EUR, None)
            ]),
            true
        );

        assert_eq!(have_same_currency(&vec![Dinero::new(0, EUR, None),]), true);
    }

    #[test]
    #[should_panic]
    fn test_have_same_currency_empty() {
        have_same_currency(&vec![]);
    }

    #[test]
    fn test_add() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(2, EUR, None))?,
            Dinero::new(3, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_negative() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(-3, EUR, None))?,
            Dinero::new(-2, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(0, EUR, None))?,
            Dinero::new(1, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_add_scale() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            add(&Dinero::new(1, EUR, None), &Dinero::new(3, EUR, Some(10)))?,
            Dinero::new(100000003, EUR, Some(10))
        );
        Ok(())
    }

    #[test]
    fn test_add_currency_check() {
        let result = add(&Dinero::new(1, EUR, None), &Dinero::new(2, USD, None));

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!("{:?}", UNEQUAL_CURRENCIES_MESSAGE)
            ),
            _ => panic!("add should not return value for unequal currencies"),
        }
    }

    #[test]
    fn test_subtract() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(5, EUR, None), &Dinero::new(2, EUR, None))?,
            Dinero::new(3, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_subtract_negative() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(1, EUR, None), &Dinero::new(-3, EUR, None))?,
            Dinero::new(4, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_subtract_zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(1, EUR, None), &Dinero::new(0, EUR, None))?,
            Dinero::new(1, EUR, None)
        );
        Ok(())
    }

    #[test]
    fn test_subtract_scale() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            subtract(&Dinero::new(1, EUR, None), &Dinero::new(3, EUR, Some(10)))?,
            Dinero::new(99999997, EUR, Some(10))
        );
        Ok(())
    }

    #[test]
    fn test_subtract_currency_check() {
        let result = subtract(&Dinero::new(1, EUR, None), &Dinero::new(2, USD, None));

        assert!(result.is_err());

        match result {
            Err(e) => assert_eq!(
                format!("{:?}", e),
                format!("{:?}", UNEQUAL_CURRENCIES_MESSAGE)
            ),
            _ => panic!("subtract should not return value for unequal currencies"),
        }
    }

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
