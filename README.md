<img align="right" height="150" src="./rust2fun.svg">

# rust2fun (pronounced: rʌstafʌn)

[![Crates.io](https://img.shields.io/crates/v/rust2fun.svg)](https://crates.io/crates/rust2fun)
[![docs.rs](https://img.shields.io/docsrs/rust2fun)](https://docs.rs/rust2fun/0.1.0/rust2fun/)
![build](https://github.com/chapiteau-team/rust2fun/actions/workflows/rust.yml/badge.svg)

A library for functional programming in Rust.

## Build

By default, the library is built with the `std` feature enabled. To disable it, use the `--no-default-features` flag.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust2fun = "0.1.0"
```

and import the prelude:

```rust
use rust2fun::prelude::*;
```

## Supported features

### Combinators:

- function composition (with [compose](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.compose.html) macro)
- pipelines (with [pipe](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.pipe.html) macro)
- currying
    - [curry2](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.curry2.html) macro
    - [curry3](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.curry3.html) macro
    - etc
- argument flipping (with [flip](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.flip.html) macro)
- constant functions
    - [constant](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.constant.html) macro
    - [constant1](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.constant1.html) macro (K combinator)
    - [constant2](https://docs.rs/rust2fun/0.1.0/rust2fun/macro.constant2.html) macro
    - etc
- [id](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.id.html) (I combinator)
- [apply](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.apply.html) (A combinator)
- [apply_to](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.apply_to.html) (T combinator)
- [substitution](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.substitution.html) (S combinator)
- [converge](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.converge.html)
- [on](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.on.html)
- [if_else](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.if_else.html)
- [fix](https://docs.rs/rust2fun/0.1.0/rust2fun/combinator/fn.fix.html) (Y combinator)

### Type classes:

- [Semigroupal](https://docs.rs/rust2fun/0.1.0/rust2fun/semigroupal/trait.Semigroupal.html)
- [Invariant](https://docs.rs/rust2fun/0.1.0/rust2fun/invariant/trait.Invariant.html)
- [Functor](https://docs.rs/rust2fun/0.1.0/rust2fun/functor/trait.Functor.html)
- [Apply](https://docs.rs/rust2fun/0.1.0/rust2fun/apply/trait.Apply.html)
- [Applicative](https://docs.rs/rust2fun/0.1.0/rust2fun/applicative/trait.Applicative.html)
- [FlatMap](https://docs.rs/rust2fun/0.1.0/rust2fun/flatmap/trait.FlatMap.html)
- [Monad](https://docs.rs/rust2fun/0.1.0/rust2fun/monad/trait.Monad.html)

## Release notes

0.1.0 (2023-01-22)

- Initial release: combinators, Semigroupal, Invariant, Functor, Apply, Applicative, FlatMap, Monad
