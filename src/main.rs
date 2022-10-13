// The project probably won't have a binary, this is just for quick testing

#[cfg(not(tarpaulin_include))]
fn main() {
    use core::{currencies::custom, Dinero};

    let eth = custom(10, 18);

    let d1 = Dinero::new(10000000000000, eth, None);

    print!("{:?}", d1);
}
