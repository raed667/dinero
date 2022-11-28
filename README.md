<p align="center">
  <a href="https://crates.io/crates/dinero">
    <img alt="Dinero-rust" src="https://user-images.githubusercontent.com/1442690/195835572-3a110f53-23ff-43f7-a188-74565eb9931c.png">
  </a>
</p>

<p align="center">
  <!-- Stability -->
  <a href="https://crates.io/crates/dinero"><img alt="Stability: alpha" src="https://img.shields.io/badge/stability-alpha-f4d03f.svg" /></a>
  <!-- Version -->
  <a href="https://crates.io/crates/dinero"><img alt="Crates.io" src="https://img.shields.io/crates/v/dinero"></a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dinero"><img alt="Crates.io" src="https://img.shields.io/crates/d/dinero"></a>
  <!-- Tests -->
  <a href="https://github.com/raed667/dinero/actions/workflows/ci.yml"><img src="https://github.com/raed667/dinero/actions/workflows/ci.yml/badge.svg" /></>
  <!-- codecov -->
  <a href="https://codecov.io/gh/raed667/dinero"><img src="https://codecov.io/gh/raed667/dinero/branch/main/graph/badge.svg?token=6IH3LQRXNH"/></a>
  <!-- Docs -->
  <a href="https://docs.rs/dinero"><img src="https://docs.rs/dinero/badge.svg"/></a>
  <!-- license -->
  <a href="https://crates.io/crates/dinero"><img alt="Crates.io" src="https://img.shields.io/crates/l/dinero"></a>
</p>

<p align="center">
  Dinero is a Rust port of <a href="https://v2.dinerojs.com/docs">Dinero.js</a>
  <br>
  Dinero lets you create, calculate, and format money in Rust.<br>
  <a href="https://docs.rs/dinero/latest/dinero/"><strong>docs.rs/dinero</strong></a>
</p>

---

## 📦 Install

```sh
$ cargo add dinero
```

## ⚡️ Quick start

`Dinero` objects are minimal. The API is heavily inspired by `dinero.js` unless there is a more suitable Rust way to implement things.

```rust
use dinero::{api::add, currencies::USD, format::to_unit, Dinero};

// Create a Dinero object of value 8.5 USD (the default scale for USD is 2)
let d1 = Dinero::new(850, USD, None);
// Create a Dinero object of value 5 USD with a custom scale 3
let d2 = Dinero::new(5000, USD, Some(3));

// Add the 2 Dineros, the value is stored in the result Dinero without modifying d1 and d2
let result = add(&d1, &d2); // Similar API as Dinero.js

let result = d1 + d2; // Or you can use the standard operators

match result {
  Ok(value) => println!("{} USD", to_unit(value, None, None)), // 13.5 USD
  Err(_) => println!("Error adding d1+d2"),
}
```

## 🦀 Disclaimer

I'm using this project to learn about Rust. And I'm working my way through the language and the ecosystem.

Consider the current version of Dinero **unstable**. There will _definitely_ be breaking changes.

## 📜 License

[MIT](LICENSE)
