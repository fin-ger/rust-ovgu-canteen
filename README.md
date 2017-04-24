# OVGU Canteen Information for Rust [![crates.io](https://img.shields.io/crates/v/ovgu-canteen.svg)](https://crates.io/crates/ovgu-canteen)

**This repository contains a program that extracts the canteen meals of the Otto-von-Guericke University from the website.**

---


## [Documentation](https://fin-ger.github.io/rust-ovgu-canteen/target/doc/ovgu_canteen/)

## [Overview](https://fin-ger.github.io/rust-ovgu-canteen/)

## Current example usage

First setup development environment:

```
cargo update
```

Then test if the library works:

```
cargo run --example json | jq -r '.[0].days[0].meals[0].price.student'
```

> jq is a json querying software available via your distributions package manager.
