use std::error::Error;

use crate::{messages::INVALID_INPUT, Dinero};

fn compare(remainder: isize, is_positive: bool) -> bool {
    if is_positive {
        remainder > 0
    } else {
        remainder < 0
    }
}

fn distribute(value: isize, ratios: Vec<isize>) -> Vec<isize> {
    let total: isize = ratios.iter().sum();
    if total == 0 {
        ratios
    } else {
        let mut remainder = value;

        let mut shares: Vec<isize> = ratios
            .iter()
            .map(|ratio| -> isize {
                let share = (value * ratio) / total;
                remainder -= share;
                share
            })
            .collect();

        let is_positive = value > 0;
        let amount = if is_positive { 1 } else { -1 };

        let mut i = 0;
        while compare(remainder, is_positive) {
            if ratios[i] != 0 {
                shares[i] += amount;
                remainder -= amount;
            }
            i += 1;
        }
        shares
    }
}

/// Distribute the amount of a Dinero object across a list of ratios.
///
/// Monetary values have indivisible units, meaning you can't always exactly split them. With allocate, you can split a monetary amount then distribute the remainder as evenly as possible.
pub fn allocate(item: &Dinero, ratios: Vec<isize>) -> Result<Vec<Dinero>, Box<dyn Error>> {
    if ratios.is_empty() {
        Err(INVALID_INPUT.to_owned())?
    } else {
        let has_only_positive_ratios = ratios.iter().all(|ratio| *ratio >= 0);

        if !has_only_positive_ratios {
            Err(INVALID_INPUT.to_owned())?
        } else {
            let shares = distribute(item.amount, ratios);
            let result: Vec<Dinero> = shares
                .iter()
                .map(|share| Dinero::new(*share, item.currency, Some(item.scale)))
                .collect();

            Ok(result)
        }
    }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
    use super::*;
    use crate::currencies::USD;
    use pretty_assertions::assert_eq;
    use std::{error::Error, vec};

    #[test]
    fn test_ratios_empty() {
        let result = allocate(&Dinero::new(42, USD, None), vec![]);
        assert!(result.is_err());
        match result {
            Err(e) => assert_eq!(format!("{:?}", e), format!("{:?}", INVALID_INPUT)),
            _ => panic!("allocate should not return value for empty vector"),
        }
    }

    #[test]
    fn test_ratios_negative() {
        let result = allocate(&Dinero::new(42, USD, None), vec![1, -2, 3]);
        assert!(result.is_err());
        match result {
            Err(e) => assert_eq!(format!("{:?}", e), format!("{:?}", INVALID_INPUT)),
            _ => panic!("allocate should not return value for negative vector values"),
        }
    }

    #[test]
    fn test_ratios_half() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(42, USD, None), vec![50, 50])?,
            vec![
                Dinero::new(21, USD, None), //
                Dinero::new(21, USD, None)
            ]
        );

        assert_eq!(
            allocate(&Dinero::new(1003, USD, None), vec![50, 50])?,
            vec![
                Dinero::new(502, USD, None), //
                Dinero::new(501, USD, None)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_ignore_zero() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(1003, USD, None), vec![0, 50, 50])?,
            vec![
                Dinero::new(0, USD, None), //
                Dinero::new(502, USD, None),
                Dinero::new(501, USD, None)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_ignore_all_zeros() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(1003, USD, None), vec![0, 0, 0])?,
            vec![
                Dinero::new(0, USD, None), //
                Dinero::new(0, USD, None),
                Dinero::new(0, USD, None)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_negative_amount() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(-1003, USD, None), vec![50, 50])?,
            vec![
                Dinero::new(-502, USD, None), //
                Dinero::new(-501, USD, None),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_1_3() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(100, USD, None), vec![1, 3])?,
            vec![
                Dinero::new(25, USD, None), //
                Dinero::new(75, USD, None)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_20_40() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(42, USD, None), vec![20, 40])?,
            vec![
                Dinero::new(14, USD, None), //
                Dinero::new(28, USD, None)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_even() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(43, USD, None), vec![20, 40])?,
            vec![
                Dinero::new(15, USD, None), //
                Dinero::new(28, USD, None)
            ]
        );
        Ok(())
    }

    #[test]
    fn test_ratios_long() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            allocate(&Dinero::new(142, USD, None), vec![20, 10, 22, 1, 40])?,
            vec![
                Dinero::new(31, USD, None), //
                Dinero::new(16, USD, None),
                Dinero::new(33, USD, None),
                Dinero::new(1, USD, None),
                Dinero::new(61, USD, None),
            ]
        );
        Ok(())
    }
}
