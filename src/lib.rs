//! A library for functional programming in Rust

#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

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

pub mod higher;

/// Convenience re-export of common members of the library.
pub mod prelude {
	pub use crate::higher::*;
}

