//! A library for functional programming in Rust

#![no_std]
#![deny(missing_docs)]

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

pub mod applicative;
pub mod apply;
pub mod combinator;
pub mod contravariant;
pub mod data;
pub mod flatmap;
pub mod functor;
pub mod higher;
pub mod invariant;
pub mod monad;
pub mod semigroup;
pub mod semigroupal;

/// Convenience re-export of common members of the library.
pub mod prelude {
    pub use crate::applicative::*;
    pub use crate::apply::*;
    pub use crate::combinator::*;
    pub use crate::contravariant::*;
    pub use crate::data::*;
    pub use crate::flatmap::*;
    pub use crate::functor::*;
    pub use crate::higher::*;
    pub use crate::invariant::*;
    pub use crate::monad::*;
    pub use crate::semigroup::*;
    pub use crate::semigroupal::*;
    pub use crate::*;
}
