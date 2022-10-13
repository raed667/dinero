use crate::Dinero;

pub enum RoundingMode {
    HalfDown,
    HalfTowardsZero,
    Up,
    Down,
    HalfEven,
    HalfUp,
    HalfAwayFromZero,
    HalfOdd,
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

pub fn to_unit(d: Dinero, digits: Option<isize>, round: Option<RoundingMode>) -> f64 {
    let to_unit_factor = d.currency.base.pow(d.scale.try_into().unwrap()) as f64;

    let digits_expo = match digits {
        Some(d) => d,
        None => d.scale,
    };

    let factor = d.currency.base.pow(digits_expo.try_into().unwrap()) as f64;

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
    fn test_to_unit() {
        assert_eq!(
            to_unit(Dinero::new(1050, USD, None), None, None), //
            10.5
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
    fn test_to_unit_round_half_towards_zero() {
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
                Dinero::new(1055, USD, None),
                Some(1),
                Some(RoundingMode::HalfAwayFromZero)
            ), //
            10.6
        );

        assert_eq!(
            to_unit(
                Dinero::new(1050, USD, None),
                Some(1),
                Some(RoundingMode::HalfAwayFromZero)
            ), //
            10.5
        );
    }
}
