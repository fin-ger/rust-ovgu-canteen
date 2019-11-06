# OVGU Canteen Information for Rust
[![crates.io](https://img.shields.io/crates/v/ovgu-canteen.svg)](https://crates.io/crates/ovgu-canteen)
[![Latest Tag](https://img.shields.io/github/tag/fin-ger/rust-ovgu-canteen.svg)](https://github.com/fin-ger/rust-ovgu-canteen/releases)
[![Build Status](https://travis-ci.org/fin-ger/rust-ovgu-canteen.svg?branch=master)](https://travis-ci.org/fin-ger/rust-ovgu-canteen)
[![Documentation](https://docs.rs/ovgu-canteen/badge.svg)](https://docs.rs/ovgu-canteen/)
[![Homepage](https://img.shields.io/badge/github.io-homepage-blue.svg)](https://fin-ger.github.io/rust-ovgu-canteen/)
[![Built with Spacemacs](https://cdn.rawgit.com/syl20bnr/spacemacs/442d025779da2f62fc86c2082703697714db6514/assets/spacemacs-badge.svg)](http://spacemacs.org)

This library parses the meals from the Otto-von-Guericke University canteen website.

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
ovgu_canteen = "^0"
```

Then put this in your crate root:

```rust
extern crate ovgu_canteen;
```

## How to Run the Examples

In order to run an example from the `examples` folder issue the following command.

```
$ cargo run --example <name>
```

To get the canteen information in a json format you can use this example:

```
$ cargo run --example json | jq -r '.[0].days[0].meals[0].price.student'
```

> jq is a json querying software available via your distributions package manager.

## License

This project is licensed under the GPL-v3 license - see the [LICENSE](LICENSE) file for details.

