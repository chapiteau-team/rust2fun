//! A library for functional programming in Rust

#![no_std]
#![deny(missing_docs)]

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

pub mod combinator;
pub mod functor;
pub mod higher;
pub mod invariant;

/// Convenience re-export of common members of the library.
pub mod prelude {
    pub use crate::combinator::*;
    pub use crate::functor::*;
    pub use crate::higher::*;
    pub use crate::invariant::*;
}
