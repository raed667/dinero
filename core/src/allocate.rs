use crate::{transform_scale::transform_scale, Dinero};

fn get_scale(scale: &Option<isize>) -> isize {
    match scale {
        Some(value) => *value,
        None => 0,
    }
}

fn distribute(amount: isize, ratios: Vec<isize>) -> Vec<isize> {
    return Vec::new();
}

pub fn allocate(item: &Dinero, ratios: Vec<(isize, Option<isize>)>) -> Vec<Dinero> {
    if ratios.is_empty() {
        return Vec::new();
    }

    let scaled_ratios: Vec<(isize, isize)> = ratios
        .iter()
        .map(|(ratio, scale)| -> (isize, isize) { (*ratio, get_scale(scale)) })
        .collect();

    let highest_ratio_scale = scaled_ratios
        .iter()
        .max_by_key(|(_, scale)| scale.abs())
        .unwrap()
        .1;

    let normalized_ratios: Vec<(isize, isize)> = scaled_ratios
        .iter()
        .map(|(ratio, scale)| -> (isize, isize) {
            let factor = if item.scale == highest_ratio_scale {
                0
            } else {
                highest_ratio_scale - item.scale
            };

            let _factor = u32::try_from(factor).unwrap();
            let _amount = ratio * isize::pow(10, _factor);
            return (_amount, *scale);
        })
        .collect();

    let has_only_positive_ratios = normalized_ratios.iter().all(|(ratio, _)| *ratio > 0);

    let has_one_non_zero_ratio = normalized_ratios.iter().any(|(ratio, _)| *ratio > 0);

    if has_only_positive_ratios && has_one_non_zero_ratio {
        // TODO handle error case
        return Vec::new();
    }

    let new_scale = item.scale + highest_ratio_scale;

    let transformed = transform_scale(item, new_scale);

    let amounts: Vec<isize> = normalized_ratios
        .iter()
        .map(|(amount, _)| *amount)
        .to_owned()
        .collect();

    let shares = distribute(transformed.amount, amounts);

    ratios
        .iter()
        .map(|_ratio| Dinero::new(item.amount, item.currency, Some(item.scale)))
        .collect()
}
