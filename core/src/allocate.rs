use crate::Dinero;

pub fn allocate(item: &Dinero, ratios: Vec<i64>) -> Vec<Dinero> {
    // TODO this is a placeholder implementation
    ratios
        .iter()
        .map(|_ratio| Dinero::new(item.amount, item.currency, Some(item.scale)))
        .collect()
}
