//! Monad.

use crate::prelude::{Applicative, FlatMap};

/// A monad. Allows composition of dependent effectful functions.
pub trait Monad<B>: FlatMap<B> + Applicative {}

impl<T, B> Monad<B> for T where T: FlatMap<B> + Applicative {}
