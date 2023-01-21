//! Monad.
//!
//! A monad is a monoid in the category of endofunctors, what these means is that
//! there must be two operations, `pure` and `flat_map`, and these two operations
//! must satisfy three laws:
//!
//! 1. Left identity: `pure(x).flat_map(f) == f(x)`
//! 2. Right identity: `m.flat_map(pure) == m`
//! 3. Associativity: `m.flat_map(f).flat_map(g) == m.flat_map(|x| f(x).flat_map(g))`
//!
//! The first law says that if we take a value, put it in a default context with
//! `pure` and then `flat_map` it with a function, we get the same result as just
//! taking the value and applying the function to it.
//!
//! The second law says that if we have a monad, we can `flat_map` it with the
//! `pure` function and the result is our original monad.
//!
//! The third law says that `flat_map` is associative.  This may not be so
//! obvious, but what it's saying is that it doesn't matter how we nest our
//! `flat_map`s, the result is the same.  In other words, the following two
//! expressions are equivalent:
//!
//! ```ignore
//! m.flat_map(f).flat_map(g)
//! ```
//!
//! ```ignore
//! m.flat_map(|x| f(x).flat_map(g))
//! ```
//!
//! The first law is sometimes called the "unit law" and the second law is
//! sometimes called the "morphism law".  The third law is sometimes called the
//! "associativity law".
//!
//! # Examples
//!
//! ```
//! use rust2fun::prelude::*;
//! #
//! # struct CreditCard;
//! # struct User;
//! # struct Transaction;
//!
//! fn get_user(id: u32) -> Option<User> {
//!     todo!("Get a user from a storage by id if it exists")
//! }
//!
//!  fn get_credit_card(user: User) -> Option<CreditCard> {
//!     todo!("Get a credit card of the user if it has one")
//!  }
//!
//!  fn charge_credit_card(amount: u32, card: CreditCard) -> Option<Transaction> {
//!     todo!("Charge a credit card if it has enough money")
//!  }
//!
//!  fn charge_user_card(amount: u32, user_id: u32) -> Option<Transaction> {
//!     get_user(user_id)
//!         .flat_map(get_credit_card)
//!         .flat_map(|card| charge_credit_card(amount, card))
//!  }
//! ```

use crate::prelude::{Applicative, FlatMap};

/// A monad. Allows composition of dependent effectful functions.
/// See [the module level documentation](self) for more.
pub trait Monad<B>: FlatMap<B> + Applicative {}

impl<T, B> Monad<B> for T where T: FlatMap<B> + Applicative {}
