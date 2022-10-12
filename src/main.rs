// The project probably won't have a binary, this is just for quick testing

#[cfg(not(tarpaulin_include))]
fn main() {
    use core::Dinero;

    use currencies::{CountryCode, Currency};

    let eth = Currency {
        code: CountryCode::Custom,
        base: 10,
        exponent: 18,
    };

    let d1 = Dinero::new(10000000000000, eth, None);

    print!("{:?}", d1);
}
