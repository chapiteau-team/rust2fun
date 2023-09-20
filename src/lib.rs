//! A library for functional programming in Rust

#![no_std]
#![deny(missing_docs)]
#![allow(clippy::too_many_arguments)]

extern crate rust2fun_macros;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
macro_rules! if_std {
	( $( $code:tt )* ) => {
		$( $code )*
	}
}

#[cfg(not(feature = "std"))]
macro_rules! if_std {
    ( $( $code:tt )* ) => {};
}

pub mod and_then;
pub mod ap_n;
pub mod applicative;
pub mod apply;
pub mod bifunctor;
pub mod combinator;
pub mod contravariant;
pub mod data;
pub mod flatmap;
pub mod functor;
pub mod higher;
pub mod invariant;
pub mod map_n;
pub mod monad;
pub mod monoid;
pub mod pure;
pub mod semigroup;
pub mod semigroupal;

/// Convenience re-export of common members of the library.
pub mod prelude {
    pub use crate::and_then::*;
    pub use crate::ap_n::*;
    pub use crate::applicative::*;
    pub use crate::apply::*;
    pub use crate::bifunctor::*;
    pub use crate::combinator::*;
    pub use crate::contravariant::*;
    pub use crate::data::*;
    pub use crate::flatmap::*;
    pub use crate::functor::*;
    pub use crate::higher::*;
    pub use crate::invariant::*;
    pub use crate::map_n::*;
    pub use crate::monad::*;
    pub use crate::monoid::*;
    pub use crate::pure::*;
    pub use crate::semigroup::*;
    pub use crate::semigroupal::*;
    pub use crate::*;
}
