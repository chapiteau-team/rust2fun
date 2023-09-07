//! Monoid.

use core::marker::PhantomData;

use crate::semigroup::Semigroup;

/// A `Monoid` is a `Semigroup` with an identity element.
pub trait Monoid: Semigroup {
    /// Returns the identity element for this monoid.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(0, i32::empty());
    /// assert_eq!((0, "".to_owned()), <(i32, String)>::empty());
    /// ```
    fn empty() -> Self;

    /// Returns true if the value is the identity element.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert!(i32::empty().is_empty());
    /// assert!(!1.is_empty());
    /// ```
    #[inline]
    fn is_empty(&self) -> bool
    where
        Self: Eq + Sized,
    {
        self == &Self::empty()
    }

    /// Given an iterator of `Monoid`s, combine them all into one.
    /// If the sequence is empty, returns `Monoid::empty()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(0, Monoid::combine_all(Vec::<u8>::new()));
    /// assert_eq!(6, Monoid::combine_all(vec![1,2,3]));
    /// ```
    #[inline]
    fn combine_all<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        iter.into_iter().fold(Self::empty(), Self::combine)
    }
}

macro_rules! semigroup_numeric {
    ($($t:ty)*) => ($(
        impl Monoid for $t {
            #[inline]
            fn empty() -> Self { 0 as $t }
        }
    )*)
}

semigroup_numeric! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl Monoid for () {
    #[inline]
    fn empty() -> Self {}

    #[inline]
    fn is_empty(&self) -> bool {
        true // Is a trivial monoid.
    }
}

macro_rules! monoid_tuple {
    ($($idx:tt $t:tt),+) => {
        impl<$($t: Monoid,)*> Monoid for ($($t,)+)
        {
            #[inline]
            fn empty() -> Self {
                ($(
                    $t :: empty(),
                )+)
            }
        }
    };
}

monoid_tuple!(0 A);
monoid_tuple!(0 A, 1 B);
monoid_tuple!(0 A, 1 B, 2 C);
monoid_tuple!(0 A, 1 B, 2 C, 3 D);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
monoid_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

impl<T> Monoid for PhantomData<T> {
    #[inline]
    fn empty() -> Self {
        PhantomData
    }

    #[inline]
    fn is_empty(&self) -> bool {
        true // Is a trivial monoid.
    }
}

impl<T: Semigroup> Monoid for Option<T> {
    #[inline]
    fn empty() -> Self {
        None
    }
}

if_std! {
    use std::collections::*;
    use std::hash::Hash;
    use std::string::String;
    use std::vec::Vec;

    impl Monoid for String {
        #[inline]
        fn empty() -> Self {
            String::new()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            String::is_empty(self)
        }
    }

    macro_rules! monoid_new {
        ($name:ident) => {
            impl<T> Monoid for $name<T> {
                #[inline]
                fn empty() -> Self {
                    $name::new()
                }

                #[inline]
                fn is_empty(&self) -> bool {
                    $name::is_empty(self)
                }
            }
        };
        ($name:ident, $ct:tt $(+ $dt:tt )*) => {
            impl<T: $ct $(+ $dt )*> Monoid for $name<T> {
                #[inline]
                fn empty() -> Self {
                    $name::new()
                }

                #[inline]
                fn is_empty(&self) -> bool {
                    $name::is_empty(self)
                }
            }
        };
    }

    monoid_new!(Vec);
    monoid_new!(LinkedList);
    monoid_new!(VecDeque);
    monoid_new!(BinaryHeap, Ord);
    monoid_new!(BTreeSet, Ord);
    monoid_new!(HashSet, Eq + Hash);

    impl<K: Eq + Hash, V: Semigroup> Monoid for HashMap<K, V> {
        #[inline]
        fn empty() -> Self {
            HashMap::new()
        }

        #[inline]
        fn is_empty(&self) -> bool {
            HashMap::is_empty(self)
        }
    }
}
