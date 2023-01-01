//! Applicative is a stronger version of Apply that has pure.

use core::marker::PhantomData;

use crate::apply::Apply;
use crate::higher::Higher;

/// Applicative functor. This is a stronger version of Apply that has pure.
/// Allows application of a function in an Applicative context to a value in an Applicative context.
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
