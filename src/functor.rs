//! Functors. The name is short for "covariant functor".

use crate::constant1;
use crate::higher::Higher;
use crate::invariant::Invariant;

/// Lift a function f to operate on Functors.
///
/// # Examples
///
/// ```
/// use rust2fun::prelude::*;
///
/// let f = lift(|x: i32| x + 1);
/// assert_eq!(Some(2), f(Some(1)));
/// ```
pub fn lift<FA, B>(f: impl FnMut(FA::Param) -> B) -> impl FnOnce(FA) -> FA::Target<B>
    where FA: Functor<B> {
    |fa: FA| fa.map(f)
}

/// Covariant functor.
pub trait Functor<MapB>: Invariant<MapB> {
    /// Transform a `Self<A>` into a `Self<B>` by providing a transformation from `A` to `B`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some("1".to_string());
    /// let actual = x.map(|s| s.parse::<i32>().unwrap());
    /// assert_eq!(Some(1), actual);
    /// ```
    fn map(self, f: impl FnMut(Self::Param) -> MapB) -> Self::Target<MapB>;

    /// Alias for [map] if the implementing type already had a built-in `.map` method.
    #[inline]
    fn fmap<F>(self, f: F) -> Self::Target<MapB>
        where F: FnMut(Self::Param) -> MapB,
              Self: Sized {
        self.map(f)
    }

    /// Tuple the values in `Self<A>` with the result of applying a function with the value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let actual = x.fproduct(|x: &i32| x.to_string());
    /// assert_eq!(Some((1, "1".to_string())), actual);
    /// ```
    #[inline]
    fn fproduct<B, F>(self, mut f: F) -> Self::Target<(Self::Param, B)>
        where F: FnMut(&Self::Param) -> B,
              Self: Functor<(<Self as Higher>::Param, B), Target<(<Self as Higher>::Param, B)>=MapB> + Sized {
        self.map(|a| {
            let rhs = f(&a);
            (a, rhs)
        })
    }

    /// Pair the result of function application with the values in `Self<A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(1);
    /// let actual = x.fproduct_left(|x: &i32| x.to_string());
    /// assert_eq!(Some(("1".to_string(), 1)), actual);
    /// ```
    #[inline]
    fn fproduct_left<B, F>(self, mut f: F) -> Self::Target<(B, Self::Param)>
        where F: FnMut(&Self::Param) -> B,
              Self: Functor<(B, <Self as Higher>::Param), Target<(B, <Self as Higher>::Param)>=MapB> + Sized {
        self.map(|a| (f(&a), a))
    }

    /// Replaces the `A` value in `Self<A>` with the supplied value.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let actual = Some(1).map_const("foo");
    /// assert_eq!(Some("foo"), actual);
    /// ```
    #[inline]
    fn map_const<B>(self, b: B) -> Self::Target<B>
        where B: Clone,
              Self: Functor<B, Target<B>=MapB> + Sized {
        self.map(constant1!(b.clone()))
    }

    /// Empty the `Self<A>` of the values, preserving the structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(Some(()), Some(1).void());
    /// ```
    #[inline]
    fn void(self) -> Self::Target<()>
        where Self: Functor<(), Target<()>=MapB> + Sized {
        self.map_const(())
    }

    /// Tuples the `A` value in `Self<A>` with the supplied `B` value, with the `B` value on the left.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(Some(("foo", 1)), Some(1).tuple_left("foo"));
    /// ```
    #[inline]
    fn tuple_left<B>(self, b: B) -> Self::Target<(B, Self::Param)>
        where B: Clone,
              Self: Functor<(B, <Self as Higher>::Param), Target<(B, <Self as Higher>::Param)>=MapB> + Sized {
        self.map(|a| (b.clone(), a))
    }

    /// Tuples the `A` value in `Self<A>` with the supplied `B` value, with the `B` value on the right.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(Some((1, "foo")), Some(1).tuple_right("foo"));
    /// ```
    #[inline]
    fn tuple_right<B>(self, b: B) -> Self::Target<(Self::Param, B)>
        where B: Clone,
              Self: Functor<(<Self as Higher>::Param, B), Target<(<Self as Higher>::Param, B)>=MapB> + Sized {
        self.map(move |a| (a, b.clone()))
    }

    /// Un-zips an `Self<(A, B)>` consisting of element pairs into two separate Self's tupled.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some((1, "foo"));
    /// assert_eq!((Some(1), Some("foo")), Functor::unzip(x));
    /// ```
    #[inline]
    fn unzip<A, B>(self) -> (Self::Target<A>, Self::Target<B>)
        where Self: Functor<A, Param=(A, B), Target<A>=MapB> + Functor<B> + Clone + Sized {
        (self.clone().map(|x: (A, B)| x.0), self.map(|x: (A, B)| x.1))
    }

    /// Lifts `if` to Functor.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::{constant, constant1};
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(true);
    /// assert_eq!(Some(1), x.iff(constant!(1), constant!(0)));
    /// ```
    #[inline]
    fn iff<A, T, F>(self, mut if_true: T, mut if_false: F) -> Self::Target<A>
        where T: FnMut() -> A,
              F: FnMut() -> A,
              Self: Functor<A, Param=bool, Target<A>=MapB> + Sized {
        self.map(|x| if x { if_true() } else { if_false() })
    }
}

/// Macro to implement [Functor] for types with [Iterator] support.
#[macro_export]
macro_rules! functor_iter {
    ($t:ident) => {
        impl<A, B> $crate::functor::Functor<B> for $t<A> {
            #[inline]
            fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
                self.into_iter().map(f).collect::<$t<_>>()
            }
        }
    }
}

impl<A, B> Functor<B> for Option<A> {
    #[inline]
    fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
        self.map(f)
    }
}

impl<A, B, E> Functor<B> for Result<A, E> {
    #[inline]
    fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
        self.map(f)
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    functor_iter!(Vec);
    functor_iter!(LinkedList);
    functor_iter!(VecDeque);

    impl<A, B> Functor<B> for Box<A> {
        #[inline]
        fn map(self, mut f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
            Box::new(f(*self))
        }
    }

    impl<A, B: Ord> Functor<B> for BinaryHeap<A> {
        #[inline]
        fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
            self.into_iter().map(f).collect()
        }
    }

    impl<A, B: Ord> Functor<B> for BTreeSet<A> {
        #[inline]
        fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
            self.into_iter().map(f).collect()
        }
    }

    impl<A, B: Eq + Hash> Functor<B> for HashSet<A> {
        #[inline]
        fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B> {
            self.into_iter().map(f).collect()
        }
    }
}
