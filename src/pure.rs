//! Pure.

use crate::higher::Higher;

/// Typeclass for lifting values into a context.
pub trait Pure: Higher {
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
    #[inline]
    fn unit() -> Self
    where
        Self: Higher<Param = ()> + Sized,
    {
        Self::pure(())
    }
}

impl<A> Pure for Option<A> {
    #[inline]
    fn pure(x: A) -> Option<A> {
        Some(x)
    }
}

impl<A, E> Pure for Result<A, E> {
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

    impl<A> Pure for Box<A> {
        #[inline]
        fn pure(x: A) -> Self {
            Box::new(x)
        }
    }

    impl<A> Pure for Vec<A> {
        #[inline]
        fn pure(x: A) -> Self {
            vec![x]
        }
    }

    impl<A> Pure for LinkedList<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = LinkedList::new();
            result.push_back(x);
            result
        }
    }

    impl<A> Pure for VecDeque<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = VecDeque::new();
            result.push_back(x);
            result
        }
    }

    impl<A: Ord> Pure for BinaryHeap<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = BinaryHeap::new();
            result.push(x);
            result
        }
    }

    impl<A: Ord> Pure for BTreeSet<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = BTreeSet::new();
            result.insert(x);
            result
        }
    }

    impl<A: Eq + Hash> Pure for HashSet<A> {
        #[inline]
        fn pure(x: A) -> Self {
            let mut result = HashSet::new();
            result.insert(x);
            result
        }
    }
}
