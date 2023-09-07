//! Semigroup.

use core::marker::PhantomData;

/// A Semigroup is an algebraic structure consisting of a set together with an associative binary
/// operation. A Semigroup is a Monoid without an identity element.
pub trait Semigroup {
    /// Associative operation which combines two values.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(3, 1.combine(2));
    /// assert_eq!(Some(1), Some(1).combine(None));
    /// ```
    fn combine(self, other: Self) -> Self;

    /// Combine with itself `n` times.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(1, 1.combine_n(0));
    /// assert_eq!(2, 1.combine_n(1));
    /// assert_eq!(3, 1.combine_n(2));
    /// assert_eq!(4, Semigroup::combine_n(1, 3));
    /// ```
    #[inline]
    fn combine_n(self, n: u32) -> Self
    where
        Self: Sized + Clone,
    {
        if n == 0 {
            return self;
        }

        let mut result = self.clone();
        for _ in 1..n {
            result = result.combine(self.clone());
        }

        result.combine(self)
    }

    /// Combine all values in the iterator and return the total.
    /// If the sequence is empty, returns None. Otherwise, returns Some(total).
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(None, Semigroup::combine_all_option(Vec::<u8>::new()));
    /// assert_eq!(Some(6), Semigroup::combine_all_option(vec![1,2,3]));
    /// # #[cfg(feature = "std")]
    /// # use std::iter::repeat;
    /// # #[cfg(feature = "std")]
    /// assert_eq!(
    ///     Some("heyheyhey".to_owned()),
    ///     Semigroup::combine_all_option(repeat("hey".to_owned()).take(3)));
    /// ```
    #[inline]
    fn combine_all_option<I>(iter: I) -> Option<Self>
    where
        I: IntoIterator<Item = Self>,
        Self: Sized,
    {
        let mut iter = iter.into_iter();
        iter.next()
            .map(|init| iter.fold(init, |acc, x| acc.combine(x)))
    }
}

/// Macro to implement [Semigroup] for numeric types.
macro_rules! semigroup_numeric {
    ($($t:ty)*) => ($(
        impl Semigroup for $t {
            #[inline]
            fn combine(self, other: Self) -> Self { self + other }
        }
    )*)
}

semigroup_numeric! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

impl Semigroup for () {
    #[inline]
    fn combine(self, _other: Self) -> Self {}
}

macro_rules! semigroup_tuple {
    ($($idx:tt $t:tt),+) => {
        impl<$($t: Semigroup,)*> Semigroup for ($($t,)+)
        {
            #[inline]
            fn combine(self, other: Self) -> Self {
                ($(
                    $t :: combine(self.$idx, other.$idx),
                )+)
            }
        }
    };
}

semigroup_tuple!(0 A);
semigroup_tuple!(0 A, 1 B);
semigroup_tuple!(0 A, 1 B, 2 C);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K);
semigroup_tuple!(0 A, 1 B, 2 C, 3 D, 4 E, 5 F, 6 G, 7 H, 8 I, 9 J, 10 K, 11 L);

/// Macro to implement [Semigroup] for types with `append` method.
#[macro_export]
macro_rules! semigroup_append {
    ($name:ident) => {
        impl<T> Semigroup for $name<T> {
            #[inline]
            fn combine(mut self, mut other: Self) -> Self {
                self.append(&mut other);
                self
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<T: $ct $(+ $dt )*> Semigroup for $name<T> {
            #[inline]
            fn combine(mut self, mut other: Self) -> Self {
                self.append(&mut other);
                self
            }
        }
    };
}

/// Macro to implement [Semigroup] for types with `extend` method.
#[macro_export]
macro_rules! semigroup_extend {
    ($name:ident) => {
        impl<T> Semigroup for $name<T> {
            #[inline]
            fn combine(mut self, other: Self) -> Self {
                self.extend(other);
                self
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<T: $ct $(+ $dt )*> Semigroup for $name<T> {
            #[inline]
            fn combine(mut self, other: Self) -> Self {
                self.extend(other);
                self
            }
        }
    };
}

impl<T> Semigroup for PhantomData<T> {
    #[inline]
    fn combine(self, _other: Self) -> Self {
        PhantomData
    }
}

impl<T: Semigroup> Semigroup for Option<T> {
    #[inline]
    fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Some(lhs), Some(rhs)) => Some(lhs.combine(rhs)),
            (x, y) => x.or(y),
        }
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::string::String;
    use std::vec::Vec;

    impl Semigroup for String {
        #[inline]
        fn combine(self, other: Self) -> Self {
            self + &other
        }
    }

    impl<T: Semigroup> Semigroup for Box<T> {
        #[inline]
        fn combine(self, other: Self) -> Self {
            Box::new((*self).combine(*other))
        }
    }

    semigroup_extend!(Vec);
    semigroup_append!(LinkedList);
    semigroup_append!(VecDeque);
    semigroup_append!(BinaryHeap, Ord);
    semigroup_append!(BTreeSet, Ord);
    semigroup_extend!(HashSet, Eq + Hash);

    impl<K: Eq + Hash, V: Semigroup> Semigroup for HashMap<K, V> {
        #[inline]
        fn combine(self, other: Self) -> Self {
            let (mut acc, other) = if self.len() > other.len() {
                (self, other)
            } else {
                (other, self)
            };

            for (k, v) in other {
                if let Some(v_acc ) = acc.remove(&k){
                    acc.insert(k, v.combine(v_acc));
                } else {
                    acc.insert(k, v);
                }
            }

            acc
        }
    }
}
