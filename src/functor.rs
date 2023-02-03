//! Functor. The name is short for "covariant functor".
//!
//! A functor is a type constructor that supports mapping over its contents.
//!
//! # Example
//!
//! ```no_run
//! use rust2fun::prelude::*;
//! #
//! # struct CreditCard;
//! # struct User;
//!
//! fn get_user(id: u32) -> Option<User> {
//!     unimplemented!("Get a user from a storage by id if it exists")
//! }
//!
//! fn get_all_users() -> Vec<User> {
//!     unimplemented!("Get all users from a storage")
//! }
//!
//! fn get_credit_card(user: User) -> CreditCard {
//!     unimplemented!("Get a credit card that corresponds to the user")
//! }
//!
//! fn print_credit_card(card: CreditCard) {
//!     unimplemented!("Print a credit card")
//! }
//!
//! fn print_user_credit_card<F>(user: F)
//! where
//!     F: Functor<CreditCard, Param = User>,
//!     F::Target<CreditCard>: Functor<(), Param = CreditCard>,
//! {
//!     user.map(get_credit_card).map(print_credit_card);
//! }
//!
//! print_user_credit_card(get_user(1));
//! # #[cfg(feature = "std")]
//! print_user_credit_card(get_all_users());
//! ```

use core::marker::PhantomData;

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
/// let mut f = lift(|x: i32| x + 1);
/// assert_eq!(Some(2), f(Some(1)));
/// ```
pub fn lift<FA, B, F>(mut f: F) -> impl FnMut(FA) -> FA::Target<B>
where
    FA: Functor<B>,
    F: FnMut(FA::Param) -> B,
{
    move |fa| fa.map(&mut f)
}

/// Covariant functor. See [the module level documentation](self) for more.
pub trait Functor<B>: Invariant<B> {
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
    fn map(self, f: impl FnMut(Self::Param) -> B) -> Self::Target<B>;

    /// Alias for [map] if the implementing type already had a built-in `.map` method.
    #[inline]
    fn fmap<F>(self, f: F) -> Self::Target<B>
    where
        F: FnMut(Self::Param) -> B,
        Self: Sized,
    {
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
    fn fproduct<F>(self, mut f: F) -> Self::Target<(Self::Param, B)>
    where
        F: FnMut(&Self::Param) -> B,
        Self: Functor<(<Self as Higher>::Param, B)> + Sized,
    {
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
    fn fproduct_left<F>(self, mut f: F) -> Self::Target<(B, Self::Param)>
    where
        F: FnMut(&Self::Param) -> B,
        Self: Functor<(B, <Self as Higher>::Param)> + Sized,
    {
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
    fn map_const(self, b: B) -> Self::Target<B>
    where
        B: Copy,
        Self: Functor<B> + Sized,
    {
        self.map(constant1!(b))
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
    where
        Self: Functor<(), Target<()> = B> + Sized,
    {
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
    fn tuple_left(self, b: B) -> Self::Target<(B, Self::Param)>
    where
        B: Copy,
        Self: Functor<(B, <Self as Higher>::Param)> + Sized,
    {
        self.map(|a| (b, a))
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
    fn tuple_right(self, b: B) -> Self::Target<(Self::Param, B)>
    where
        B: Copy,
        Self: Functor<(<Self as Higher>::Param, B)> + Sized,
    {
        self.map(|a| (a, b))
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
    fn unzip<A>(self) -> (Self::Target<A>, Self::Target<B>)
    where
        Self: Higher<Param = (A, B)> + Functor<A> + Functor<B> + Copy + Sized,
    {
        (self.map(|x| x.0), self.map(|x| x.1))
    }

    /// `if` lifted into Functor.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let x = Some(true);
    /// assert_eq!(Some(1), x.if_f(constant!(1), constant!(0)));
    /// ```
    #[inline]
    fn if_f<T, F>(self, mut if_true: T, mut if_false: F) -> Self::Target<B>
    where
        T: FnMut() -> B,
        F: FnMut() -> B,
        Self: Functor<B, Param = bool> + Sized,
    {
        self.map(|x| if x { if_true() } else { if_false() })
    }
}

/// Macro to implement [Functor] for types with [Iterator] support.
#[macro_export]
macro_rules! functor_iter {
    ($name:ident) => {
        impl<A, B> $crate::functor::Functor<B> for $name<A> {
            #[inline]
            fn map(self, f: impl FnMut(A) -> B) -> Self::Target<B> {
                self.into_iter().map(f).collect::<$name<B>>()
            }
        }
    };
    ($name:ident, $ct:tt $(+ $dt:tt )*) => {
        impl<A, B: $ct $(+ $dt )*> $crate::functor::Functor<B> for $name<A> {
            #[inline]
            fn map(self, f: impl FnMut(A) -> B) -> Self::Target<B> {
                self.into_iter().map(f).collect::<$name<B>>()
            }
        }
    };
}

impl<A, B> Functor<B> for PhantomData<A> {
    #[inline]
    fn map(self, _f: impl FnMut(A) -> B) -> PhantomData<B> {
        PhantomData
    }
}

impl<A, B> Functor<B> for Option<A> {
    #[inline]
    fn map(self, f: impl FnMut(A) -> B) -> Option<B> {
        self.map(f)
    }
}

impl<A, B, E> Functor<B> for Result<A, E> {
    #[inline]
    fn map(self, f: impl FnMut(A) -> B) -> Result<B, E> {
        self.map(f)
    }
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::hash::Hash;
    use std::vec::Vec;

    impl<A, B> Functor<B> for Box<A> {
        #[inline]
        fn map(self, mut f: impl FnMut(A) -> B) -> Box<B> {
            Box::new(f(*self))
        }
    }

    functor_iter!(Vec);
    functor_iter!(LinkedList);
    functor_iter!(VecDeque);
    functor_iter!(BinaryHeap, Ord);
    functor_iter!(BTreeSet, Ord);
    functor_iter!(HashSet, Eq + Hash);

    impl<A, B, K: Eq + Hash> Functor<B> for HashMap<K, A> {
        #[inline]
        fn map(self, mut f: impl FnMut(A) -> B) -> HashMap<K, B> {
            self.into_iter().map(|(k, v)| (k, f(v))).collect()
        }
    }
}
