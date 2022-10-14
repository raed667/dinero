use crate::Dinero;

use super::transform_scale::transform_scale;

/// normalize scale
///
/// Normalize a vector of Dineros to the highest scale.
///
/// Normalizing to a higher scale means that the internal amount value increases by orders of magnitude. **Be careful not to exceed the minimum and maximum safe integers.**
///
/// Example
/// ```rust
/// normalize_scale(vec![
///     Dinero::new(10, EUR, Some(2)),
///     Dinero::new(20000, USD, Some(5)),
///     Dinero::new(30, EUR, Some(3)),
/// ]);
/// ```
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
            transform_scale(d, highest_scale)
        })
        .collect();
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

    use super::*;
    use crate::currencies::{EUR, USD};
    use pretty_assertions::assert_eq;

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
}
