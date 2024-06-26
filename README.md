<img align="right" height="75" src="./rust2fun.svg">

# rust2fun (pronounced: rʌstafʌn)

[![Crates.io](https://img.shields.io/crates/v/rust2fun.svg)](https://crates.io/crates/rust2fun)
[![docs.rs](https://img.shields.io/docsrs/rust2fun)](https://docs.rs/rust2fun/0.2.1/rust2fun/)
![build](https://github.com/chapiteau-team/rust2fun/actions/workflows/rust.yml/badge.svg)

A library for functional programming in Rust.

## Build

By default, the library is built with the `std` feature enabled. To disable it, use the `--no-default-features` flag.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust2fun = "0.2.1"
```

and import the prelude:

```rust
use rust2fun::prelude::*;
```

## Supported features

### Combinators:

- function composition (with [compose](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.compose.html) macro)
- pipelines (with [pipe](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.pipe.html) macro)
- currying
    - [curry2](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.curry2.html) macro
    - [curry3](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.curry3.html) macro
    - etc
- argument flipping (with [flip](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.flip.html) macro)
- constant functions
    - [constant](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.constant.html) macro
    - [constant1](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.constant1.html) macro (K combinator)
    - [constant2](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.constant2.html) macro
    - etc
- [id](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.id.html) (I combinator)
- [apply](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.apply.html) (A combinator)
- [apply_to](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.apply_to.html) (T combinator)
- [substitution](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.substitution.html) (S combinator)
- [converge](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.converge.html)
- [on](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.on.html) (Psi combinator)
- [if_else](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.if_else.html)
- [fix](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.fix.html) (Y combinator)
- no operation
    - [noop](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.noop.html)
    - [noop1](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.noop1.html)
    - [noop2](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.noop2.html)
    - etc
- tuple constructors
    - [tuple2](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.tuple2.html)
    - [tuple3](https://docs.rs/rust2fun/0.2.1/rust2fun/combinator/fn.tuple3.html)
    - etc

### Type classes:

- [Semigroup](https://docs.rs/rust2fun/0.2.1/rust2fun/semigroup/trait.Semigroup.html)
- [Monoid](https://docs.rs/rust2fun/0.2.1/rust2fun/monoid/trait.Monoid.html)
- [Semigroupal](https://docs.rs/rust2fun/0.2.1/rust2fun/semigroupal/trait.Semigroupal.html)
- [Invariant](https://docs.rs/rust2fun/0.2.1/rust2fun/invariant/trait.Invariant.html)
- [Functor](https://docs.rs/rust2fun/0.2.1/rust2fun/functor/trait.Functor.html)
- [Bifunctor](https://docs.rs/rust2fun/0.2.1/rust2fun/bifunctor/trait.Bifunctor.html)
- [Pure](https://docs.rs/rust2fun/0.2.1/rust2fun/pure/trait.Pure.html)
- [AndThen](https://docs.rs/rust2fun/0.2.1/rust2fun/and_then/trait.AndThen.html)
- [Apply](https://docs.rs/rust2fun/0.2.1/rust2fun/apply/trait.Apply.html)
- [Applicative](https://docs.rs/rust2fun/0.2.1/rust2fun/applicative/trait.Applicative.html)
- [FlatMap](https://docs.rs/rust2fun/0.2.1/rust2fun/flatmap/trait.FlatMap.html)
- [Monad](https://docs.rs/rust2fun/0.2.1/rust2fun/monad/trait.Monad.html) + ( [bind!](https://docs.rs/rust2fun/0.2.1/rust2fun/macro.bind.html) notation )
- FnK (functor transformation)

### Data types:

- [NEVec](https://docs.rs/rust2fun/0.2.1/rust2fun/data/ne_vec/struct.NEVec.html) (non-empty vector)
- [Validated](https://docs.rs/rust2fun/0.2.1/rust2fun/data/validated/enum.Validated.html)
- [ValidatedNev](https://docs.rs/rust2fun/0.2.1/rust2fun/data/validated/type.ValidatedNev.html)

## Examples

1. Function `print_user_credit_card` accepts user(s) wrapped in any effect (Option, Result, Vec, etc.) and prints
   corresponding credit card(s).

```rust
fn get_credit_card(user: User) -> CreditCard {
    // Get credit card for user
}

fn print_credit_card(card: CreditCard) {
    // Print credit card details
}

fn print_credit_card_of<F>(user: F)
    where
        F: Functor<CreditCard, Param=User>,
        F::Target<CreditCard>: Functor<(), Param=CreditCard>,
{
    user.map(get_credit_card).map(print_credit_card);
}
```

...usage:

```rust
fn user(id: u32) -> Option<User> {
    // Get user from database
}

fn all_users() -> Vec<User> {
    // Get all users from database
}

print_credit_card_of(user(1));
print_credit_card_of(all_users());
```

2. Validation accumulating all errors.

Assuming we have the following validation rules that need to be applied to create a new credit card:

```rust
fn validate_number(number: CreditCardNumber) -> ValidatedNev<CreditCardNumber, Error> {
    // Validating credit card number
}

fn validate_expiration(date: Date) -> ValidatedNev<Date, Error> {
    // Validating expiration date
}

fn validate_cvv(cvv: Code) -> ValidatedNev<Code, Error> {
    // Validating CVV code
}
```

...we can create a new credit card by applying all validation rules and collecting all errors in a vector `Vec`,
non-empty vector `NEVec` (like in the example) or other semigroup (e.g. `String`, `u32`, etc.):

```rust
fn validate_credit_card(
    number: CreditCardNumber,
    expiration: Date,
    cvv: Code,
) -> ValidatedNev<CreditCard, Error> {
    ValidatedNev::pure(CreditCard::new)
        .ap3(validate_number(number),
             validate_expiration(expiration),
             validate_cvv(cvv))
}
```

...alternatively, this can be done using the `map3` method:

```rust
fn validate_credit_card(
    number: CreditCardNumber,
    expiration: Date,
    cvv: Code,
) -> ValidatedNev<CreditCard, Error> { 
    MapN::map3(validate_number(number),
               validate_expiration(expiration),
               validate_cvv(cvv),
               CreditCard::new)
}
```

3. `bind!` notation for monads (like `do` notation in Haskell or `for` comprehension in Scala):

Assuming we have the following functions defined:

```rust
fn get_opening_prices() -> Vec<(AssetId, i32)> {
  // Get opening prices from an external service
}

fn get_closing_prices() -> Vec<(AssetId, i32)> {
  // Get closing prices from an external service
}

fn get_asset_name(id: AssetId) -> Option<String> {
  // Recover asset name for the given id
}
```

...we can use `bind!` notation to calculate daily profit for each asset:

```rust
let profits: Vec<(String, i32)> = bind! {
    for (id_open, opening_price) in get_opening_prices();
    for (id_close, closing_price) in get_closing_prices();
    let diff = closing_price - opening_price;
    for name in OptionToVec.apply(get_asset_name(id_open)),
        if id_open == id_close && diff > 0;
    (name, diff)
};
```

## Release notes

0.1.0 (2023-01-22)

- Initial release: combinators, Semigroupal, Invariant, Functor, Apply, Applicative, FlatMap, Monad

0.2.0 (2023-09-10)

- The project got its logo (thanks [olasinitsyna](https://www.behance.net/olasinitsyna))
- Moved macros imports to the prelude
- Added `noopX` and `tupleX` sets of functions
- Added type classes: Semigroup, Monoid, Bifunctor + Higher2 (thanks [lrind](https://github.com/lrind))
- Added data types: NEVec, Validated
- Added `bind!` notation
- Multiple fixes and improvements

0.2.1 (2023-09-21)

- Fixed Semigroupal and Apply behavior (thanks [GoldsteinE](https://github.com/GoldsteinE) for the report)
- Added type classes: Pure, AndThen
- Refactored `mapX` and `apX` functions
