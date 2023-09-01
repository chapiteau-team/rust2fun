use std::vec::Vec;

use super::*;

impl<T> From<NEVec<T>> for Vec<T> {
    #[inline]
    fn from(nevec: NEVec<T>) -> Self {
        nevec.into_vec()
    }
}

impl<T: Clone> From<&NEVec<T>> for Vec<T> {
    #[inline]
    fn from(nevec: &NEVec<T>) -> Self {
        nevec.to_vec()
    }
}

impl<T> From<NEVec<T>> for (T, Vec<T>) {
    #[inline]
    fn from(nevec: NEVec<T>) -> Self {
        (nevec.head, nevec.tail)
    }
}

impl<T> From<(T, Vec<T>)> for NEVec<T> {
    #[inline]
    fn from((head, tail): (T, Vec<T>)) -> Self {
        Self { head, tail }
    }
}

impl<T> TryFrom<Vec<T>> for NEVec<T> {
    type Error = Vec<T>;

    #[inline]
    fn try_from(mut vec: Vec<T>) -> Result<Self, Self::Error> {
        if vec.is_empty() { Err(vec) } else {
            Ok(Self {
                head: vec.remove(0),
                tail: vec,
            })
        }
    }
}

impl<T> FromIterator<T> for NEVec<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        iter.next().map(|head| {
            let tail = iter.collect();
            NEVec { head, tail }
        }).unwrap_or_else(|| non_empty_invariant_failed())
    }
}