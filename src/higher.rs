//! Implementation of Higher Kinded Types for Rust.
//!
//! A higher kinded type is a concept that reifies a type constructor as an actual type.
//!
//! A type constructor can be thought of in these analogies:
//! * like a function in the type universe
//! * as a type with a "hole" in it
//! * as a container containing type(s)
//! * as a generic type, parameterised over other types
//! * as an endofunctor in the category of types
//!
//! To be able to use them in places where concrete "proper" types are usually expected, a language
//! must support the concept of higher kinded types. Although Rust lacks in a native support for HKT,
//! we always have a walk around called Lightweight Higher Kinded Type.
//!
//! # See also
//!
//! * [Lightweight Higher Kinded Type](https://www.cl.cam.ac.uk/~jdy22/papers/lightweight-higher-kinded-polymorphism.pdf)
//! * [Rust/Haskell: Higher-Kinded Types (HKT)](https://gist.github.com/CMCDragonkai/a5638f50c87d49f815b8)

use core::marker::PhantomData;

/// Implementation of Lightweight Higher Kinded Type for a type of kind `* -> *`.
pub trait Higher {
    /// Type parameter abstracted by Higher, i.e. `Option<Param>`.
    type Param;
    /// Swapped higher type, i.e. Target = Option<T>.
    type Target<T>: Higher<Param = T>;

    /// Unsafe cast from one [Higher] type to another. This is a safe operation as long as the
    /// resulting type is the same as the original type. Might be useful for building abstractions.
    fn unsafe_cast<T, R>(self) -> R
    where
        Self: Higher<Param = T> + Sized,
        R: Higher<Param = T>,
    {
        let ptr = &self as *const _ as *const R;
        let result = unsafe { core::ptr::read_volatile(ptr) };
        core::mem::forget(self);
        result
    }
}

/// Implementation of Higher Kinded Type for a type of kind `*, * -> *, *`.
pub trait Higher2 {
    /// First type parameter abstracted by Higher2, i.e. `Result<Param1, _>`.
    type Param1;
    /// Second type parameter abstracted by Higher2, i.e. `Result<_, Param2>`.
    type Param2;
    /// Swapped higher type for 2 types, i.e Target = Result<T1, T2>.
    type Target<T1, T2>: Higher2<Param1 = T1, Param2 = T2>;
}

/// Macro implementing `Higher` for a given type of kind `* -> *`.
///
/// # Example
///
/// ```
/// use rust2fun::prelude::*;
///
/// struct Unary<T>(T);
/// higher!(Unary);
/// ```
/// This will implement `Higher` for `Unary` as follows:
/// ```
/// use rust2fun::prelude::*;
///
/// struct Unary<T>(T);
///
/// impl<P> Higher for Unary<P> {
///     type Param = P;
///     type Target<T> = Unary<T>;
/// }
/// ```
#[macro_export]
macro_rules! higher {
    ($t:ident) => {
        impl<P> $crate::higher::Higher for $t<P> {
            type Param = P;
            type Target<T> = $t<T>;
        }
    };
}

higher!(Option);
higher!(PhantomData);

impl<P, E> Higher for Result<P, E> {
    type Param = P;
    type Target<T> = Result<T, E>;
}

impl<P, E> Higher2 for Result<P, E> {
    type Param1 = P;
    type Param2 = E;
    type Target<TP, TE> = Result<TP, TE>;
}

impl<A, B> Higher2 for (A, B) {
    type Param1 = A;
    type Param2 = B;
    type Target<TA, TB> = (TA, TB);
}

if_std! {
    use std::boxed::Box;
    use std::collections::*;
    use std::vec::Vec;

    higher!(Vec);
    higher!(Box);
    higher!(LinkedList);
    higher!(BinaryHeap);
    higher!(BTreeSet);
    higher!(VecDeque);
    higher!(HashSet);

    impl<K, V> Higher for HashMap<K, V> {
        type Param = V;
        type Target<T> = HashMap<K, T>;
    }

    impl<K, V> Higher2 for HashMap<K, V>{
        type Param1 = K;
        type Param2 = V;
        type Target<TK, TV> = HashMap<TK, TV>;
    }
}
