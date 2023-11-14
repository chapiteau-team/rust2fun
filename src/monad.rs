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
//! fn get_credit_card(user: User) -> Option<CreditCard> {
//!     todo!("Get a credit card of the user if it has one")
//! }
//!
//! fn charge_credit_card(amount: u32, card: CreditCard) -> Option<Transaction> {
//!     todo!("Charge a credit card if it has enough money")
//! }
//!
//! fn charge_user_card(amount: u32, user_id: u32) -> Option<Transaction> {
//!     get_user(user_id)
//!         .flat_map(get_credit_card)
//!         .flat_map(|card| charge_credit_card(amount, card))
//! }
//!
//! // Or using the `bind!` macro:
//! fn charge_user_card_alt(amount: u32, user_id: u32) -> Option<Transaction> {
//!     bind! {
//!         for user in get_user(user_id);
//!         for card in get_credit_card(user);
//!         return charge_credit_card(amount, card);
//!     }
//! }
//! ```

use crate::prelude::FlatMap;
use crate::pure::Pure;

/// A monad. Allows composition of dependent effectful functions.
/// See [the module level documentation](self) for more.
pub trait Monad<B>: FlatMap<B> + Pure {}

impl<T, B> Monad<B> for T where T: FlatMap<B> + Pure {}

/// Bind macro. Allows for a more natural syntax for monadic composition.
/// It is similar to the `do` notation in Haskell or the `for` notation in Scala.
///
/// # Usage
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = bind! {
///     for x in Some(1);
///     for y in Some(2);
///     x + y
/// };
/// assert_eq!(Some(3), actual);
///
/// let actual = bind! {
///     for x in Some(1);
///     for y in None::<i32>;
///     x + y
/// };
/// assert_eq!(None, actual);
/// ```
///
/// The syntax supports pattern matching, can bind variables and contain statements.
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = bind! {
///     for (_, a) in Some((1, 2));
///     let b = 3;
///     std::println!("a = {}, b = {}", a, b);
///     a + b
/// };
///
/// assert_eq!(Some(5), actual);
/// ```
///
/// Guards can be implemented using an if statement within a bind discarding the result.
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = bind! {
///     for x in Some(1);
///     for _guard in if x > 0 { Some(()) } else { None };
///     x
/// };
///
/// assert_eq!(Some(1), actual);
/// ```
///
/// ... or using the special `if` syntax for monoids.
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = bind! {
///     for x in Some(1);
///     for _guard in Some(()), if x > 0;
///     x
/// };
///
/// assert_eq!(Some(1), actual);
/// ```
///
/// The last example can be rewritten with the return syntax as follows:
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = bind! {
///     for x in Some(1);
///     return Some(x), if x > 0;
/// };
///
/// assert_eq!(Some(1), actual);
/// ```
///
/// The return keyword is used to return the result of the last bind.
///
/// ```
/// use rust2fun::prelude::*;
///
/// let actual = bind! {
///     for x in Some(1);
///     for y in Some(2);
///     return Some(x + y);
/// };
///
/// assert_eq!(Some(3), actual);
/// ```
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// # type AssetId = u32;
///
/// fn get_opening_prices() -> Vec<(AssetId, i32)> {
///     vec![(1, 225), (2, 310), (3, 128), (4, 99), (5, 200), (6, 0)]
/// }
///
/// fn get_closing_prices() -> Vec<(AssetId, i32)> {
///    vec![(5, 210), (3, 130), (2, 308), (4, 100), (1, 220)]
/// }
///
/// fn get_asset_name(id: AssetId) -> Option<String> {
///     match id {
///         1 => Some("AAPL".to_string()),
///         2 => Some("MSFT".to_string()),
///         3 => Some("GOOG".to_string()),
///         4 => Some("AMZN".to_string()),
///         _ => None,
///     }
/// }
///
/// let profits = bind! {
///     for (id_open, opening_price) in get_opening_prices();
///     for (id_close, closing_price) in get_closing_prices();
///     let diff = closing_price - opening_price;
///     for name in OptionToVec.apply(get_asset_name(id_open)),
///         if id_open == id_close && diff > 0;
///     (name, diff)
/// };
///
/// assert_eq!(vec![("GOOG".to_string(), 2), ("AMZN".to_string(), 1)], profits);
/// ```
#[macro_export]
macro_rules! bind {
    (return $e:expr, if $cond:expr;) => (
        if $cond { $e } else { $crate::monoid::Monoid::empty() }
    );
    (return $e:expr;) => (
        $e;
    );
    (let $x:ident : $t:ty  = $e:expr; $($rest:tt)+) => ({
        let $x : $t = $e;
        bind!($($rest)+)
    });
    (let $p:pat = $e:expr; $($rest:tt)+) => ({
        let $p = $e;
        bind!($($rest)+)
    });
    (for $p:pat in $e:expr , if $cond:expr ; $($rest:tt)+) => (
        $crate::flatmap::FlatMap::flat_map(
            if $cond { $e } else { $crate::monoid::Monoid::empty() },
            move |$p| bind!($($rest)+),
        )
    );
    (for $p:pat in $e:expr; $($rest:tt)+) => (
        $crate::flatmap::FlatMap::flat_map($e, move |$p| bind!($($rest)+))
    );
    ($s:stmt;  $($rest:tt)+) => ({
        $s
        bind!($($rest)+)
    });
    ($e:expr) => (
        $crate::pure::Pure::pure($e)
    );
}
