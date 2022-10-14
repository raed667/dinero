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

`Dinero` objects are minimal. Every function in `dinero.js` is side-effect free, allowing you only to bundle exactly what you use.

```rust
use dinero::{api::add, currencies::USD, Dinero};

fn main() {
    let d1 = Dinero::new(500, USD, None);
    let d2 = Dinero::new(800, USD, None);

    let result = add(&d1, &d2);
}
```

## 🦀 Disclaimer

I'm using this project to learn about Rust. And I'm working my way through the language and the ecosystem.

Consider the current version of Dinero **unstable**. There will _definitely_ be breaking changes.

## 📜 License

[MIT](LICENSE)
