#![allow(dead_code)]
extern crate rust2fun_laws;

use std::fmt::Debug;
use std::str::FromStr;

#[macro_export]
#[cfg(feature = "std")]
macro_rules! if_std {
	( $( $code:tt )* ) => {
		$( $code )*
	}
}

#[macro_export]
#[cfg(not(feature = "std"))]
macro_rules! if_std {
    ( $( $code:tt )* ) => {};
}

pub fn parse<T: FromStr>(x: String) -> T
where
    <T as FromStr>::Err: Debug,
{
    FromStr::from_str(x.as_str()).unwrap()
}

pub fn print<T: ToString>(x: T) -> String {
    x.to_string()
}
