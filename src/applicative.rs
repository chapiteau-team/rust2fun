//! Applicative functor.
//! Allows application of a function in an Applicative context to a value in an Applicative context.
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
//! fn validate_number(number: CreditCardNumber) -> Result<CreditCardNumber, Error> {
//!     unimplemented!("Validate credit card number")
//! }
//!
//! fn validate_expiration(date: Date) -> Result<Date, Error> {
//!     unimplemented!("Validate credit card expiration date")
//! }
//!
//! fn validate_cvv(cvv: Code) -> Result<Code, Error> {
//!     unimplemented!("Validate credit card cvv")
//! }
//!
//! fn validate_credit_card(
//!     number: CreditCardNumber,
//!     expiration: Date,
//!     cvv: Code,
//! ) -> Result<CreditCard, Error> {
//!     Result::pure(CreditCard::new)
//!         .ap3(validate_number(number),
//!              validate_expiration(expiration),
//!              validate_cvv(cvv))
//! }
//! ```

use crate::apply::Apply;
use crate::pure::Pure;

/// Applicative functor. This is a stronger version of Apply that has pure.
/// See [the module level documentation](self) for more.
pub trait Applicative<A, B>: Apply<A, B> + Pure {}

impl<A, B, T> Applicative<A, B> for T where T: Apply<A, B> + Pure {}
