extern crate rust2fun_laws;

use std::marker::PhantomData;

use rust2fun_laws::applicative_laws::*;
use rust2fun_laws::apply_laws::*;
use rust2fun_laws::contravariant_laws::*;
use rust2fun_laws::flatmap_laws::*;
use rust2fun_laws::functor_laws::*;
use rust2fun_laws::invariant_laws::*;
use rust2fun_laws::monad_laws::*;
use rust2fun_laws::semigroup_laws::*;
use rust2fun_laws::semigroupal_laws::*;

use crate::common::{parse, print};

mod common;

#[test]
fn test_invariant() {
    assert!(invariant_identity(PhantomData::<bool>).holds());
    assert!(invariant_composition(PhantomData::<u32>, print, parse, parse::<i32>, print).holds());
}

#[test]
fn test_functor() {
    assert!(covariant_identity(PhantomData::<u32>).holds());
    assert!(covariant_composition(PhantomData::<i32>, print, parse::<u32>).holds());
    assert!(lift_identity(PhantomData::<u32>).holds());
    assert!(lift_composition(PhantomData::<i32>, print, parse::<i64>).holds());
}

#[test]
fn test_contravariant() {
    assert!(contravariant_identity(PhantomData::<u32>).holds());
    assert!(contravariant_composition(PhantomData::<i32>, parse::<i32>, print::<u32>).holds());
    assert!(lift_contravariant_identity(PhantomData::<u32>).holds());
    assert!(lift_contravariant_composition(PhantomData::<i32>, parse::<i32>, print::<u32>).holds());
}

#[test]
fn test_semigroup() {
    assert!(repeat_0(PhantomData::<u32>).holds());
    assert!(repeat_1(PhantomData::<u32>).holds());
    assert!(
        semigroup_associativity(PhantomData::<u32>, PhantomData::<u32>, PhantomData::<u32>).holds()
    );
}

#[test]
fn test_semigroupal() {
    assert!(
        semigroupal_associativity(PhantomData::<u32>, PhantomData::<u32>, PhantomData::<u32>)
            .holds()
    );
}

#[test]
fn test_apply() {
    assert!(map2_product_consistency(PhantomData, PhantomData, |x: &str, l| x.len() == l).holds());
    assert!(product_r_consistency(PhantomData::<u32>, PhantomData::<u32>).holds());
    assert!(product_l_consistency(PhantomData::<u32>, PhantomData::<u32>).holds());
}

#[test]
fn test_applicative() {
    assert!(applicative_identity(PhantomData::<u32>).holds());
    assert!(applicative_homomorphism::<PhantomData<_>, _, _>(1, print).holds());
    assert!(applicative_map(PhantomData::<i32>, print).holds());
    assert!(ap_product_consistent(PhantomData, PhantomData::<fn(i32) -> String>).holds());
    assert!(applicative_unit::<PhantomData<_>>(1).holds());
}

#[test]
fn test_flatmap() {
    assert!(flat_map_associativity(
        PhantomData::<i32>,
        |_| PhantomData::<f32>,
        |_| PhantomData::<u32>
    )
    .holds());
    assert!(flat_map_consistent_apply(PhantomData, PhantomData::<fn(i32) -> u32>).holds());
    assert!(m_product_consistency(PhantomData, |_: bool| PhantomData::<u32>).holds());
}

#[test]
fn test_monad() {
    assert!(monad_left_identity::<PhantomData<_>, _, _>(1, |_| PhantomData::<u32>).holds());
    assert!(monad_right_identity(PhantomData::<i32>).holds());
    assert!(map_flat_map_coherence(PhantomData, print::<i32>).holds());
}
