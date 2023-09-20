//! Error handling with the `Validated` type.
//!
//! [`Validated<T, E>`][Validated] is the type used for returning and propagating
//! errors. It is an enum with the variants, [`Valid`], representing success and
//! containing a valid value, and [`Invalid`], representing error and
//! containing an error value.
//!
//! Validated is very similar to [`Result`] in that [`Valid`] corresponds to
//! [`Ok`] and [`Invalid`] corresponds to [`Err`]. The difference is that [`Validated`] is
//! designed for use cases in which it may be more convenient to collect all of the errors
//! that have occurred, rather than stopping at the first error encountered.
//!
//! Note that unlike [`Result`] [`Validated`] is not a monad, but it is an applicative functor,
//! so it is possible to use it with functions that operate on [`Validated`] values in an
//! applicative style.
//!
//! # Examples
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! # type CreditCardNumber = String;
//! # type Date = String;
//! # type Code = u16;
//! # type Error = u8;
//! #
//! # struct CreditCard {
//! #     number: CreditCardNumber,
//! #     expiration: Date,
//! #     cvv: Code,
//! # }
//! #
//! # impl CreditCard {
//! #     fn new(number: CreditCardNumber, expiration: Date, cvv: Code) -> Self {
//! #         CreditCard {
//! #             number,
//! #             expiration,
//! #             cvv,
//! #         }
//! #     }
//! # }
//! #
//! fn validate_number(number: CreditCardNumber) -> ValidatedNev<CreditCardNumber, Error> {
//!     unimplemented!("Validate credit card number")
//! }
//!
//! fn validate_expiration(date: Date) -> ValidatedNev<Date, Error> {
//!     unimplemented!("Validate credit card expiration date")
//! }
//!
//! fn validate_cvv(cvv: Code) -> ValidatedNev<Code, Error> {
//!     unimplemented!("Validate credit card cvv")
//! }
//!
//! fn validate_credit_card(
//!     number: CreditCardNumber,
//!     expiration: Date,
//!     cvv: Code,
//! ) -> ValidatedNev<CreditCard, Error> {
//!     ValidatedNev::pure(CreditCard::new)
//!         .ap3(validate_number(number),
//!              validate_expiration(expiration),
//!              validate_cvv(cvv))
//! }
//!
//! // Alternative implementation using `map3`:
//! fn validate_credit_card_alt(
//!     number: CreditCardNumber,
//!     expiration: Date,
//!     cvv: Code,
//! ) -> ValidatedNev<CreditCard, Error> {
//!     MapN::map3(validate_number(number),
//!                validate_expiration(expiration),
//!                validate_cvv(cvv),
//!                CreditCard::new)
//! }
//! ```
pub use Validated::{Invalid, Valid};

use crate::and_then::AndThen;
use crate::apply::Apply;
use crate::bifunctor::Bifunctor;
use crate::functor::Functor;
use crate::higher::{Higher, Higher2};
use crate::invariant_functor;
use crate::pure::Pure;
use crate::semigroup::Semigroup;
use crate::semigroupal::Semigroupal;

mod from;

/// Type alias for a [`Validated`] value accumulating errors in a non-empty vector.
#[cfg(feature = "std")]
pub type ValidatedNev<T, E> = Validated<T, super::NEVec<E>>;

/// `Validated` is a type that represents either a [`Valid`] value or an error([`Invalid`]).
///
/// See the [module-level documentation](self) for more details.
#[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum Validated<T, E> {
    /// Contains a valid value.
    Valid(T),
    /// Contains the error.
    Invalid(E),
}

impl<T, E> Validated<T, E> {
    /// Returns `true` if the `Validated` is [`Valid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(1);
    /// assert_eq!(x.is_valid(), true);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.is_valid(), false);
    /// ```
    #[inline]
    pub const fn is_valid(&self) -> bool {
        matches!(*self, Valid(_))
    }

    /// Returns `true` if the `Validated` is [`Invalid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(1);
    /// assert_eq!(x.is_invalid(), false);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.is_invalid(), true);
    /// ```
    #[inline]
    pub const fn is_invalid(&self) -> bool {
        !self.is_valid()
    }

    /// Converts from `Validated<T, E>` to [`Option<T>`].
    ///
    /// Converts `self` into an [`Option<T>`], consuming `self`, and discarding the error, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(1);
    /// assert_eq!(x.valid(), Some(1));
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.valid(), None);
    /// ```
    #[inline]
    pub fn valid(self) -> Option<T> {
        match self {
            Valid(x) => Some(x),
            Invalid(_) => None,
        }
    }

    /// Converts from `Validated<T, E>` to [`Option<E>`].
    ///
    /// Converts `self` into an [`Option<E>`], consuming `self`, and discarding the value, if any.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(1);
    /// assert_eq!(x.invalid(), None);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.invalid(), Some("error"));
    /// ```
    #[inline]
    pub fn invalid(self) -> Option<E> {
        match self {
            Valid(_) => None,
            Invalid(x) => Some(x),
        }
    }

    /// Converts from `Validated<T, E>` to [`Result<T, E>`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(1);
    /// assert_eq!(x.into_result(), Ok(1));
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.into_result(), Err("error"));
    /// ```
    #[inline]
    pub fn into_result(self) -> Result<T, E> {
        match self {
            Valid(x) => Ok(x),
            Invalid(x) => Err(x),
        }
    }

    /// Converts from `&Validated<T, E>` to `Validated<&T, &E>`.
    ///
    /// Produces a new `Validated`, containing a reference
    /// into the original, leaving the original in place.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(1);
    /// assert_eq!(x.as_ref(), Valid(&1));
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.as_ref(), Invalid(&"error"));
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> Validated<&T, &E> {
        match *self {
            Valid(ref x) => Valid(x),
            Invalid(ref x) => Invalid(x),
        }
    }

    /// Converts from `&mut Validated<T, E>` to `Validated<&mut T, &mut E>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// fn mutate(x: &mut Validated<i32, i32>) {
    ///    match x.as_mut() {
    ///       Valid(x) => *x = 1,
    ///       Invalid(x) => *x = 2,
    ///   }
    /// }
    ///
    /// let mut x = Valid(-1);
    /// mutate(&mut x);
    /// assert_eq!(x, Valid(1));
    ///
    /// let mut x = Invalid(-2);
    /// mutate(&mut x);
    /// assert_eq!(x, Invalid(2));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Validated<&mut T, &mut E> {
        match *self {
            Valid(ref mut x) => Valid(x),
            Invalid(ref mut x) => Invalid(x),
        }
    }

    /// Maps a `Validated<T, E>` to `Validated<U, E>` by applying a function to a
    /// contained [`Valid`] value, leaving an [`Invalid`] value untouched.
    ///
    /// This function can be used to compose the results of two functions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// fn square(x: i32) -> i32 { x * x }
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.map(square), Valid(4));
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.map(square), Invalid("error"));
    /// ```
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U, E> {
        match self {
            Valid(x) => Valid(f(x)),
            Invalid(x) => Invalid(x),
        }
    }

    /// Returns the provided default (if [`Invalid`]), or applies a function to the
    /// contained value (if [`Valid`]).
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`map_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`map_or_else`]: Validated::map_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.map_or(0, |v| v * 2), 4);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.map_or(0, |v| v * 2), 0);
    /// ```
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Valid(x) => f(x),
            Invalid(_) => default,
        }
    }

    /// Maps a `Validated<T, E>` to `U` by applying fallback function `default` to
    /// a contained [`Invalid`] value, or a function `f` to a contained [`Valid`] value.
    ///
    /// This function can be used to unpack a successful result while handling an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.map_or_else(|e| e.len() as i32, |v| v * 2), 4);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.map_or_else(|e| e.len() as i32, |v| v * 2), 5);
    /// ```
    #[inline]
    pub fn map_or_else<U, D: FnOnce(E) -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Valid(x) => f(x),
            Invalid(x) => default(x),
        }
    }

    /// Maps a `Validated<T, E>` to `Validated<T, U>` by applying a function to a
    /// contained [`Invalid`] value, leaving a [`Valid`] value untouched.
    ///
    /// This function can be used to pass through a successful result while handling an error.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// fn stringify(x: i32) -> String { format!("error code: {x}") }
    ///
    /// let x: Validated<i32, i32> = Valid(2);
    /// assert_eq!(x.map_err(stringify), Valid(2));
    ///
    /// let x: Validated<i32, i32> = Invalid(13);
    /// assert_eq!(x.map_err(stringify), Invalid("error code: 13".to_string()));
    /// ```
    #[inline]
    pub fn map_err<U, F: FnOnce(E) -> U>(self, f: F) -> Validated<T, U> {
        match self {
            Valid(x) => Valid(x),
            Invalid(x) => Invalid(f(x)),
        }
    }

    /// Converts from `Validated<T, E>` (or `&Validated<T, E>`) to
    /// `Validated<&<T as Deref>::Target, &E>`.
    ///
    /// Coerces the [`Valid`] variant of a [`Validated`] via [`Deref`](core::ops::Deref)
    /// and returns the new [`Validated`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<String, u32> = Valid("hello".to_string());
    /// let y: Validated<&str, &u32> = Valid("hello");
    /// assert_eq!(x.as_deref(), y);
    ///
    /// let x: Validated<String, u32> = Invalid(42);
    /// let y: Validated<&str, &u32> = Invalid(&42);
    /// assert_eq!(x.as_deref(), y);
    /// ```
    #[inline]
    pub fn as_deref(&self) -> Validated<&T::Target, &E>
    where
        T: core::ops::Deref,
    {
        self.as_ref().map(|x| x.deref())
    }

    /// Converts from `Validated<T, E>` (or `&mut Validated<T, E>`) to
    /// `Validated<&mut <T as Deref>::Target, &mut E>`.
    ///
    /// Coerces the [`Valid`] variant of a [`Validated`] via [`DerefMut`](core::ops::DerefMut)
    /// and returns the new [`Validated`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut s = "HELLO".to_string();
    /// let mut x: Validated<String, u32> = Valid("hello".to_string());
    /// let y: Validated<&mut str, &mut u32> = Valid(&mut s);
    /// assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
    ///
    /// let mut i = 42;
    /// let mut x: Validated<String, u32> = Invalid(42);
    /// let y: Validated<&mut str, &mut u32> = Invalid(&mut i);
    /// assert_eq!(x.as_deref_mut().map(|x| { x.make_ascii_uppercase(); x }), y);
    /// ```
    #[inline]
    pub fn as_deref_mut(&mut self) -> Validated<&mut T::Target, &mut E>
    where
        T: core::ops::DerefMut,
    {
        self.as_mut().map(|x| x.deref_mut())
    }

    /// Returns the contained [`Valid`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Invalid`]
    /// case explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Validated::unwrap_or
    /// [`unwrap_or_else`]: Validated::unwrap_or_else
    /// [`unwrap_or_default`]: Validated::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Invalid`] with a panic message provided by
    /// the passed message, and the content of the [`Invalid`].
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Invalid("emergency failure");
    /// x.expect("Testing expect"); // panics with `Testing expect: emergency failure`
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T
    where
        E: core::fmt::Debug,
    {
        match self {
            Valid(x) => x,
            Invalid(e) => unwrap_failed(msg, &e),
        }
    }

    /// Returns the contained [`Valid`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Invalid`]
    /// case explicitly, or call [`unwrap_or`], [`unwrap_or_else`], or
    /// [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Validated::unwrap_or
    /// [`unwrap_or_else`]: Validated::unwrap_or_else
    /// [`unwrap_or_default`]: Validated::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Invalid`] with a panic message provided by
    /// the content of the [`Invalid`].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.unwrap(), 2);
    /// ```
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Invalid("emergency failure");
    /// x.unwrap(); // panics with `emergency failure`
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T
    where
        E: core::fmt::Debug,
    {
        match self {
            Valid(x) => x,
            Invalid(e) => unwrap_failed("called `Validated::unwrap()` on an `Invalid` value", &e),
        }
    }

    /// Returns the contained [`Valid`] value, consuming the `self` value.
    ///
    /// Consumes the `self` argument then, if [`Valid`], returns the contained
    /// value, otherwise if [`Invalid`], returns `default` value for `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.unwrap_or_default(), 2);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.unwrap_or_default(), 0);
    /// ```
    #[inline]
    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            Valid(x) => x,
            Invalid(_) => Default::default(),
        }
    }

    /// Returns the contained [`Invalid`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Valid`] with a panic message provided by
    /// the passed message, and the content of the [`Valid`].
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// x.expect_err("Testing expect_err"); // panics with `Testing expect_err: 2`
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect_err(self, msg: &str) -> E
    where
        T: core::fmt::Debug,
    {
        match self {
            Valid(x) => unwrap_failed(msg, &x),
            Invalid(x) => x,
        }
    }

    /// Returns the contained [`Invalid`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is an [`Valid`] with a panic message provided by
    /// the content of the [`Valid`].
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// x.unwrap_err(); // panics with `2`
    /// ```
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.unwrap_err(), "error");
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap_err(self) -> E
    where
        T: core::fmt::Debug,
    {
        match self {
            Valid(x) => unwrap_failed("called `Validated::unwrap_err()` on a `Valid` value", &x),
            Invalid(x) => x,
        }
    }

    /// Returns `other` if the result is [`Valid`], otherwise returns the [`Invalid`]
    /// value of `self`.
    ///
    /// Arguments passed to `and` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`and_then`], which is
    /// lazily evaluated.
    ///
    /// [`and_then`]: Validated::and_then
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// let y: Validated<&str, &str> = Invalid("late error");
    /// assert_eq!(x.and(y), Invalid("late error"));
    ///
    /// let x: Validated<i32, &str> = Invalid("early error");
    /// let y: Validated<&str, &str> = Valid("foo");
    /// assert_eq!(x.and(y), Invalid("early error"));
    ///
    /// let x: Validated<i32, &str> = Invalid("not a 2");
    /// let y: Validated<&str, &str> = Invalid("late error");
    /// assert_eq!(x.and(y), Invalid("not a 2"));
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// let y: Validated<&str, &str> = Valid("different result type");
    /// assert_eq!(x.and(y), Valid("different result type"));
    /// ```
    #[inline]
    pub fn and<U>(self, other: Validated<U, E>) -> Validated<U, E> {
        match self {
            Valid(_) => other,
            Invalid(x) => Invalid(x),
        }
    }

    /// Calls `f` if the result is [`Valid`], otherwise returns the [`Invalid`]
    /// value of `self`.
    ///
    /// This allows "chained" validation: the output of one validation can be fed
    /// into another validation function.
    ///
    /// This function has a "fail-fast" behaviour, meaning that it will stop
    /// execution on the first [`Invalid`] value. This makes it inconsistent with
    /// [`Apply::ap`] or other similar `Apply`-based functions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// fn sq_then_to_string(x: u32) -> Validated<String, &'static str> {
    ///     x.checked_mul(x).map(|sq| sq.to_string()).ok_or("overflowed").into()
    /// }
    ///
    /// assert_eq!(Valid(2).and_then(sq_then_to_string), Valid(4.to_string()));
    /// assert_eq!(Valid(1_000_000).and_then(sq_then_to_string), Invalid("overflowed"));
    /// assert_eq!(Invalid("invalid").and_then(sq_then_to_string), Invalid("invalid"));
    /// ```
    #[inline]
    pub fn and_then<U, F: FnOnce(T) -> Validated<U, E>>(self, f: F) -> Validated<U, E> {
        match self {
            Valid(x) => f(x),
            Invalid(x) => Invalid(x),
        }
    }

    /// Returns `other` if the result is [`Invalid`], otherwise returns the [`Valid`]
    /// value of `self`.
    ///
    /// Arguments passed to `or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`or_else`], which is
    /// lazily evaluated.
    ///
    /// [`or_else`]: Validated::or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// let y: Validated<i32, &str> = Invalid("late error");
    /// assert_eq!(x.or(y), Valid(2));
    ///
    /// let x: Validated<i32, &str> = Invalid("early error");
    /// let y: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.or(y), Valid(2));
    ///
    /// let x: Validated<i32, &str> = Invalid("not a 2");
    /// let y: Validated<i32, &str> = Invalid("late error");
    /// assert_eq!(x.or(y), Invalid("late error"));
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// let y: Validated<i32, &str> = Valid(100);
    /// assert_eq!(x.or(y), Valid(2));
    /// ```
    #[inline]
    pub fn or<U>(self, other: Validated<T, U>) -> Validated<T, U> {
        match self {
            Valid(x) => Valid(x),
            Invalid(_) => other,
        }
    }

    /// Calls `f` if the result is [`Invalid`], otherwise returns the [`Valid`]
    /// value of `self`.
    ///
    /// This function can be used for control flow based on Validated values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// fn sq(x: u32) -> Validated<u32, u32> { Valid(x * x) }
    /// fn err(x: u32) -> Validated<u32, u32> { Invalid(x) }
    ///
    /// assert_eq!(Valid(2).or_else(sq).or_else(sq), Valid(2));
    /// assert_eq!(Valid(2).or_else(err).or_else(sq), Valid(2));
    /// assert_eq!(Invalid(3).or_else(sq).or_else(err), Valid(9));
    /// assert_eq!(Invalid(3).or_else(err).or_else(err), Invalid(3));
    /// ```
    #[inline]
    pub fn or_else<U, F: FnOnce(E) -> Validated<T, U>>(self, f: F) -> Validated<T, U> {
        match self {
            Valid(x) => Valid(x),
            Invalid(x) => f(x),
        }
    }

    /// Returns the contained [`Valid`] value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`unwrap_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`unwrap_or_else`]: Validated::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x: Validated<i32, &str> = Valid(2);
    /// assert_eq!(x.unwrap_or(0), 2);
    ///
    /// let x: Validated<i32, &str> = Invalid("error");
    /// assert_eq!(x.unwrap_or(0), 0);
    /// ```
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Valid(x) => x,
            Invalid(_) => default,
        }
    }

    /// Returns the contained [`Valid`] value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// fn count(x: &str) -> usize { x.len() }
    /// assert_eq!(Valid(2).unwrap_or_else(count), 2);
    /// assert_eq!(Invalid("foo").unwrap_or_else(count), 3);
    /// ```
    #[inline]
    pub fn unwrap_or_else<F: FnOnce(E) -> T>(self, f: F) -> T {
        match self {
            Valid(x) => x,
            Invalid(x) => f(x),
        }
    }
}

// This is a separate function to reduce the code size of the methods
#[inline(never)]
#[cold]
#[track_caller]
fn unwrap_failed(msg: &str, error: &dyn core::fmt::Debug) -> ! {
    panic!("{msg}: {error:?}")
}

impl<T, E> Clone for Validated<T, E>
where
    T: Clone,
    E: Clone,
{
    fn clone(&self) -> Self {
        match self {
            Valid(x) => Valid(x.clone()),
            Invalid(x) => Invalid(x.clone()),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Valid(to), Valid(from)) => to.clone_from(from),
            (Invalid(to), Invalid(from)) => to.clone_from(from),
            (x, y) => *x = y.clone(),
        }
    }
}

impl<P, E> Higher for Validated<P, E> {
    type Param = P;
    type Target<T> = Validated<T, E>;
}

impl<T, E> Higher2 for Validated<T, E> {
    type Param1 = T;
    type Param2 = E;
    type Target<TV, TE> = Validated<TV, TE>;
}

invariant_functor!(Validated<T, E>);

impl<A, B, E> Functor<B> for Validated<A, E> {
    #[inline]
    fn map(self, f: impl FnMut(A) -> B) -> Validated<B, E> {
        self.map(f)
    }
}

impl<A, B, E: Semigroup> Semigroupal<B> for Validated<A, E> {
    #[inline]
    fn product(self, fb: Validated<B, E>) -> Validated<(A, B), E> {
        match (self, fb) {
            (Valid(a), Valid(b)) => Valid((a, b)),
            (Invalid(lhs), Invalid(rhs)) => Invalid(lhs.combine(rhs)),
            (Invalid(e), _) | (_, Invalid(e)) => Invalid(e),
        }
    }
}

impl<F, A, B, E: Semigroup> Apply<A, B> for Validated<F, E> {
    #[inline]
    fn ap(self, fa: Validated<A, E>) -> Validated<B, E>
    where
        F: FnOnce(A) -> B,
    {
        match (self, fa) {
            (Valid(f), Valid(a)) => Valid(f(a)),
            (Invalid(lhs), Invalid(rhs)) => Invalid(lhs.combine(rhs)),
            (Invalid(e), _) | (_, Invalid(e)) => Invalid(e),
        }
    }
}

impl<A, E: Semigroup> Pure for Validated<A, E> {
    #[inline]
    fn pure(x: A) -> Self {
        Valid(x)
    }
}

impl<T: Semigroup, E: Semigroup> Semigroup for Validated<T, E> {
    #[inline]
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Valid(lhs), Valid(rhs)) => Valid(lhs.combine(rhs)),
            (Invalid(lhs), Invalid(rhs)) => Invalid(lhs.combine(rhs)),
            (e @ Invalid(_), _) | (_, e @ Invalid(_)) => e,
        }
    }
}

impl<A, B, C, D> Bifunctor<C, D> for Validated<A, B> {
    #[inline]
    fn bimap(self, mut f: impl FnMut(A) -> C, mut g: impl FnMut(B) -> D) -> Validated<C, D> {
        match self {
            Valid(x) => Valid(f(x)),
            Invalid(e) => Invalid(g(e)),
        }
    }
}

impl<A, B, E> AndThen<B> for Validated<A, E> {
    #[inline]
    fn and_then<F>(self, f: F) -> Validated<B, E>
    where
        F: FnMut(A) -> Validated<B, E>,
    {
        self.and_then(f)
    }
}
