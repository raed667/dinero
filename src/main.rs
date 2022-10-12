use core::{add, Dinero};

use currencies::USD;

// The project probably won't have a binary, this is just for quick testing

#[cfg(not(tarpaulin_include))]
fn main() {
    let d1 = Dinero::new(420, USD, Some(2));
    let d2 = Dinero::new(300, USD, Some(3));

    let d3 = add(&d1, &d2);

    match d3 {
        Ok(result) => println!("d3 {:?}\n\n", result),
        Err(e) => println!("Error {}", e),
    }
}
