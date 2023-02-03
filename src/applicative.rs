//! Applicative functor.
//! Allows application of a function in an Applicative context to a value in an Applicative context.
//!
//! # Examples
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! # struct CreditCardNumber;
//! # struct Date;
//! # struct Code;
//! # struct Error;
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
//!     Result::pure(curry3!(CreditCard::new))
//!         .ap(validate_number(number))
//!         .ap(validate_expiration(expiration))
//!         .ap(validate_cvv(cvv))
//! }
//!
//! // Can be also written as:
//! fn validate_credit_card_alt(
//!     number: CreditCardNumber,
//!     expiration: Date,
//!     cvv: Code,
//! ) -> Result<CreditCard, Error> {
//!     validate_number(number)
//!         .map(|number| |expiration, cvv| CreditCard::new(number, expiration, cvv))
//!         .ap2(validate_expiration(expiration), validate_cvv(cvv))
//! }
//! ```

use core::marker::PhantomData;

use crate::apply::Apply;
use crate::higher::Higher;

/// Applicative functor. This is a stronger version of Apply that has pure.
/// See [the module level documentation](self) for more.
pub trait Applicative: Apply<<Self as Higher>::Param>
where
    Self: Sized,
{
    /// Lift a value into a context.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let actual = Option::pure(1);
    /// assert_eq!(Some(1), actual);
    /// ```
    fn pure(x: Self::Param) -> Self;

    /// Lift Unit into a context.
    /// This is a convenience method for `pure(())`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let actual = Option::unit();
    /// assert_eq!(Some(()), actual);
    /// ```
    fn unit() -> Self
    where
        Self: Higher<Param = ()> + Sized,
    {
        Self::pure(())
    }
}

impl<A> Applicative for PhantomData<A> {
    #[inline]
    fn pure(_x: A) -> Self {
        PhantomData
    }
}

impl<A> Applicative for Option<A> {
    #[inline]
    fn pure(x: A) -> Option<A> {
        Some(x)
    }
}

impl<A, E> Applicative for Result<A, E> {
    #[inline]
    fn pure(x: A) -> Self {
        Ok(x)
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec;
    use std::vec::Vec;

    impl<A> Applicative for Box<A> {
        #[inline]
        fn pure(x: A) -> Self {
            Box::new(x)
        }
    }

    impl<A> Applicative for Vec<A> {
        #[inline]
        fn pure(x: A) -> Self {
            vec![x]
        }
    }

    impl<A> Applicative for LinkedList<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = LinkedList::new();
            result.push_back(x);
            result
        }
    }

    impl<A> Applicative for VecDeque<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = VecDeque::new();
            result.push_back(x);
            result
        }
    }

    impl<A: Ord> Applicative for BinaryHeap<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = BinaryHeap::new();
            result.push(x);
            result
        }
    }

    impl<A: Ord> Applicative for BTreeSet<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = BTreeSet::new();
            result.insert(x);
            result
        }
    }

    impl<A: Eq + Hash> Applicative for HashSet<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = HashSet::new();
            result.insert(x);
            result
        }
    }
}
