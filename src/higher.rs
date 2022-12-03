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

/// Implementation of Lightweight Higher Kinded Type for a type of kind `* -> *`.
pub trait Higher {
    /// Type parameter abstracted by Higher, i.e `Option<Param>`.
    type Param;
    /// Swapped higher type, i.e Target = Option<T>.
    type Target<T>: Higher<Param = T>;
}

/// Macro implementing `Higher` for a given type of kind `* -> *`.
///
/// # Example
///
/// ```
/// use rust2fun::higher;
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

impl<P, E> Higher for Result<P, E> {
    type Param = P;
    type Target<T> = Result<T, E>;
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
}
