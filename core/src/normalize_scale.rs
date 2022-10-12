use crate::{transform_scale::transform_scale, Dinero};

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
    use currencies::{EUR, USD};
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
