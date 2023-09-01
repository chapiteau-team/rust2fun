//! Data types.

pub use validated::*;

if_std! {
    pub use ne_vec::*;

    pub mod ne_vec;
}

pub mod validated;
