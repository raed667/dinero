use crate::Dinero;

/// Check whether a Dinero has minor currency units.
pub fn has_sub_units(d: &Dinero) -> bool {
    d.amount % d.currency.base.pow(d.scale.try_into().unwrap()) != 0
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::USD;

    #[test]
    fn test_has_sub_units() {
        assert!(has_sub_units(&Dinero::new(1150, USD, None)));
        assert!(!has_sub_units(&Dinero::new(1100, USD, None)));
    }

    #[test]
    fn test_has_sub_units_scale() {
        assert!(has_sub_units(&Dinero::new(1100, USD, Some(3))));
    }
}
