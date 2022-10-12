use core::subtract::subtract;

// The project probably won't have a binary, this is just for quick testing

#[cfg(not(tarpaulin_include))]
fn main() {
    use core::Dinero;

    use currencies::USD;

    let d1 = Dinero::new(420, USD, Some(2));
    let d2 = Dinero::new(300, USD, Some(3));
    let d3 = subtract(&d1, &d2).unwrap();

    print!("{:?}", d3);
}
