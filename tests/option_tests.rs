extern crate rust2fun_laws;

use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;

use crate::common::{parse, print};

mod common;

#[test]
fn test_invariant() {
    assert!(invariant_identity(None::<bool>).holds());
    assert!(invariant_identity(Some(1)).holds());

    let invariant_composition_for = |x| invariant_composition(x, print, parse, parse::<i32>, print);
    assert!(invariant_composition_for(None).holds());
    assert!(invariant_composition_for(Some(1)).holds());
}

#[test]
fn test_functor() {
    assert!(covariant_identity(None::<u32>).holds());
    assert!(covariant_identity(Some(1)).holds());

    let covariant_composition_for = |x| covariant_composition(x, print, parse::<u32>);
    assert!(covariant_composition_for(None).holds());
    assert!(covariant_composition_for(Some(1)).holds());

    assert!(lift_identity(None::<u32>).holds());
    assert!(lift_identity(Some(1)).holds());

    let lift_composition_for = |x| lift_composition(x, print, parse::<i64>);
    assert!(lift_composition_for(None).holds());
    assert!(lift_composition_for(Some(1)).holds());
}
