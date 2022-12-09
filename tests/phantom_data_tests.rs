extern crate rust2fun_laws;

use std::marker::PhantomData;

use rust2fun_laws::contravariant_laws::{
    contravariant_composition, contravariant_identity, lift_contravariant_composition,
    lift_contravariant_identity,
};
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;

use crate::common::{parse, print};

mod common;

#[test]
fn test_invariant() {
    assert!(invariant_identity(PhantomData::<bool>).holds());

    let invariant_composition_for = |x| invariant_composition(x, print, parse, parse::<i32>, print);
    assert!(invariant_composition_for(PhantomData::<i32>).holds());
}

#[test]
fn test_functor() {
    assert!(covariant_identity(PhantomData::<u32>).holds());

    let covariant_composition_for = |x| covariant_composition(x, print, parse::<u32>);
    assert!(covariant_composition_for(PhantomData::<i32>).holds());

    assert!(lift_identity(PhantomData::<u32>).holds());

    let lift_composition_for = |x| lift_composition(x, print, parse::<i64>);
    assert!(lift_composition_for(PhantomData::<i32>).holds());
}

#[test]
fn test_contravariant() {
    assert!(contravariant_identity(PhantomData::<u32>).holds());

    let covariant_composition_for = |x| contravariant_composition(x, parse::<i32>, print::<u32>);
    assert!(covariant_composition_for(PhantomData::<i32>).holds());

    assert!(lift_contravariant_identity(PhantomData::<u32>).holds());

    let lift_contravariant_composition_for =
        |x| lift_contravariant_composition(x, parse::<i32>, print::<u32>);
    assert!(lift_contravariant_composition_for(PhantomData::<i32>).holds());
}
