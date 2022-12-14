use crate::Dinero;

pub enum RoundingMode {
    /// Round a number with half values down.
    HalfDown,
    /// Round a number with half values to nearest integer closest to zero.
    HalfTowardsZero,
    /// Round a number up.
    Up,
    /// Round a number down.
    Down,
    /// Round a number with half values to nearest even integer.
    HalfEven,
    /// Round a number with half values up.
    HalfUp,
    /// Round a number with half values to nearest integer farthest from zero.
    HalfAwayFromZero,
    /// Round a number with half values to nearest odd integer.
    HalfOdd,
    /// Does not round value.
    Identity,
}

fn is_half(value: f64) -> bool {
    value.abs() % 1.0 == 0.5
}

fn is_even(value: f64) -> bool {
    value % 2.0 == 0.0
}

fn sign(value: f64) -> f64 {
    if value == 0.0 {
        0.0
    } else if value.is_sign_positive() {
        1.0
    } else {
        -1.0
    }
}

/// Get the amount of a Dinero object in major currency unit.
///
/// By default, the number of represented fraction digits depends on the amount and scale of the Dinero object. You can specify how many fraction digits you want to represent and pass a rounding function.
///
/// For convenience, Dinero provides the following rounding functions: up, down, halfUp, halfDown, halfOdd, halfEven (bankers rounding), halfTowardsZero, and halfAwayFromZero.
///
pub fn to_unit(d: Dinero, digits: Option<u32>, round: Option<RoundingMode>) -> f64 {
    let to_unit_factor = d.currency.base.pow(d.scale) as f64;

    let digits_expo = match digits {
        Some(d) => d,
        None => d.scale,
    };

    let factor = d.currency.base.pow(digits_expo) as f64;

    let amount = d.amount as f64;

    let value = ((amount / to_unit_factor) as f64) * factor;

    let rounded = match round {
        Some(RoundingMode::Identity) => value,
        Some(RoundingMode::Up) => value.ceil(),
        Some(RoundingMode::Down) => value.floor(),
        Some(RoundingMode::HalfUp) => value.round(),
        Some(RoundingMode::HalfDown) => {
            if is_half(value) {
                value.floor()
            } else {
                value.round()
            }
        }
        Some(RoundingMode::HalfEven) => {
            let rounded = value.round();
            if !is_half(value) {
                return rounded;
            }
            if is_even(rounded) {
                rounded
            } else {
                rounded - 1.0
            }
        }
        Some(RoundingMode::HalfOdd) => {
            let rounded = value.round();
            if !is_half(value) {
                rounded
            } else if is_even(rounded) {
                rounded - 1.0
            } else {
                rounded
            }
        }
        Some(RoundingMode::HalfTowardsZero) => {
            if is_half(value) {
                sign(value) * value.abs().floor()
            } else {
                value.round()
            }
        }
        Some(RoundingMode::HalfAwayFromZero) => {
            if is_half(value) {
                sign(value) * value.abs().ceil()
            } else {
                value.round()
            }
        }
        None => value,
    };

    rounded / factor
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use crate::currencies::USD;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_sign() {
        assert_eq!(sign(0.0), 0.0);
        assert_eq!(sign(-0.0), 0.0);
        assert_eq!(sign(1.0), 1.0);
        assert_eq!(sign(-1.0), -1.0);

        assert_eq!(sign(42.0), 1.0);
        assert_eq!(sign(-42.0), -1.0);
    }

    #[test]
    fn test_to_unit() {
        assert_eq!(
            to_unit(Dinero::new(1050, USD, None), None, None), //
            10.5
        );

        assert_eq!(
            to_unit(
                Dinero::new(0, USD, Some(1)),
                None,
                Some(RoundingMode::HalfAwayFromZero)
            ), //
            0.0
        );
    }

    #[test]
    fn test_to_unit_zero() {
        assert_eq!(
            to_unit(Dinero::new(0, USD, None), None, None), //
            0.0
        );
    }
    #[test]
    fn test_to_unit_scale() {
        assert_eq!(
            to_unit(Dinero::new(10545, USD, Some(3)), None, None), //
            10.545
        );
    }

    #[test]
    fn test_to_unit_round_identity() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::Identity)
            ), //
            10.55
        );
    }

    #[test]
    fn test_to_unit_round_down() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::Down)
            ), //
            10.5
        );
    }

    #[test]
    fn test_to_unit_round_up() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::Up)
            ), //
            10.6
        );
    }

    #[test]
    fn test_to_unit_round_half_down() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfDown)
            ), //
            10.5
        );
    }

    #[test]
    fn test_to_unit_round_half_down_round() {
        assert_eq!(
            to_unit(
                Dinero::new(1056, USD, None),
                Some(1),
                Some(RoundingMode::HalfDown)
            ), //
            10.6
        );
    }

    #[test]
    fn test_to_unit_round_half_up() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfUp)
            ), //
            10.6
        );
    }

    #[test]
    fn test_to_unit_round_half_even() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfEven)
            ), //
            10.6
        );
    }

    #[test]
    fn test_to_unit_round_half_even_2() {
        assert_eq!(
            to_unit(
                Dinero::new(1056, USD, None),
                Some(1),
                Some(RoundingMode::HalfEven)
            ), //
            106.0
        );
    }

    #[test]
    fn test_to_unit_round_half_even_zero() {
        assert_eq!(
            to_unit(
                Dinero::new(0, USD, None),
                Some(1),
                Some(RoundingMode::HalfEven)
            ), //
            0.0
        );
    }

    #[test]
    fn test_to_unit_round_half_odd() {
        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfOdd)
            ), //
            10.5
        );
    }

    #[test]
    fn test_to_unit_round_half_odd_2() {
        assert_eq!(
            to_unit(
                Dinero::new(1056, USD, None),
                Some(1),
                Some(RoundingMode::HalfOdd)
            ), //
            10.6
        );
    }

    #[test]
    fn test_to_unit_round_half_odd_3() {
        assert_eq!(
            to_unit(
                Dinero::new(1000, USD, None),
                Some(1),
                Some(RoundingMode::HalfOdd)
            ), //
            10.0
        );
    }

    #[test]
    fn test_to_unit_round_half_odd_4() {
        assert_eq!(
            to_unit(
                Dinero::new(5, USD, Some(4)),
                Some(3),
                Some(RoundingMode::HalfOdd)
            ), //
            0.001
        );
    }

    #[test]
    fn test_to_unit_round_half_even_4() {
        assert_eq!(
            to_unit(
                Dinero::new(5, USD, Some(4)),
                Some(3),
                Some(RoundingMode::HalfEven)
            ), //
            0.0
        );
    }

    #[test]
    fn test_to_unit_round_half_towards_zero() {
        assert_eq!(
            to_unit(
                Dinero::new(0, USD, None),
                Some(1),
                Some(RoundingMode::HalfTowardsZero)
            ), //
            0.0
        );

        assert_eq!(
            to_unit(
                Dinero::new(-1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfTowardsZero)
            ), //
            -10.5
        );

        assert_eq!(
            to_unit(
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfTowardsZero)
            ), //
            10.5
        );

        assert_eq!(
            to_unit(
                Dinero::new(1050, USD, None),
                Some(1),
                Some(RoundingMode::HalfTowardsZero)
            ), //
            10.5
        );
    }

    #[test]
    fn test_to_unit_round_half_away_zero() {
        assert_eq!(
            to_unit(
                Dinero::new(15, USD, Some(1)),
                None,
                Some(RoundingMode::HalfAwayFromZero)
            ), //
            1.5
        );

        assert_eq!(
            to_unit(
                Dinero::new(1050, USD, None),
                Some(1),
                Some(RoundingMode::HalfAwayFromZero)
            ), //
            10.5
        );

        assert_eq!(
            to_unit(
                Dinero::new(5, USD, Some(4)),
                Some(3),
                Some(RoundingMode::HalfAwayFromZero)
            ), //
            0.001
        );
    }
}
