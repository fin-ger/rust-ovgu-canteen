# OVGU Canteen Information for Rust
[![crates.io](https://img.shields.io/crates/v/ovgu-canteen.svg)](https://crates.io/crates/ovgu-canteen)
[![Latest Tag](https://img.shields.io/github/tag/fin-ger/rust-ovgu-canteen.svg)](https://github.com/fin-ger/rust-ovgu-canteen/releases)
[![Build Status](https://travis-ci.org/fin-ger/rust-ovgu-canteen.svg?branch=master)](https://travis-ci.org/fin-ger/rust-ovgu-canteen)
[![Documentation](https://img.shields.io/badge/docs.rs-reference-blue.svg)](https://docs.rs/ovgu-canteen/)
[![Homepage](https://img.shields.io/badge/github.io-homepage-red.svg)](https://fin-ger.github.io/rust-ovgu-canteen/)

This library parses the meals from the Otto-von-Guericke University canteen website.

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project.

### Prerequisites

First make sure you have the latest stable toolchain installed.

```
$ rustup update stable
```

Then check if stable is set as your default toolchain.

```
$ rustup toolchain list
```

If stable is not your default you can set it with

```
$ rustup default stable
```

To start development clone this repository on your local machine.

```
$ git clone git@github.com:fin-ger/rust-ovgu-canteen.git
```

### How to Build

```
$ cargo build
```

### How to Run Unit And Integration Tests

```
$ cargo test
```

### How to Run

To run this project issue the following command.

```
$ cargo run
```

### How to Run the Examples

In order to run an example from the `example` folder issue the following command.

```
$ cargo run --example <name>
```

To get the canteen information in a json format you can use this example:

```
$ cargo run --example json | jq -r '.[0].days[0].meals[0].price.student'
```

> jq is a json querying software available via your distributions package manager.


## Deployment

The project can be deployed to [crates.io](https://crates.io/).

### How to Publish a Version

The following command will publish your local version of this project to [crates.io](https://crates.io/).

```
$ cargo publish
```

## License

This project is licensed under the GPL-v3 license - see the [LICENSE.md](LICENSE.md) file for details.
