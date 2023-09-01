use core::iter;
use std::{slice, vec};

use super::*;

impl<T> IntoIterator for NEVec<T> {
    type Item = T;
    type IntoIter = iter::Chain<iter::Once<T>, vec::IntoIter<Self::Item>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self.head).chain(self.tail)
    }
}

impl<'a, T> IntoIterator for &'a NEVec<T> {
    type Item = &'a T;
    type IntoIter = iter::Chain<iter::Once<&'a T>, slice::Iter<'a, T>>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(&self.head).chain(self.tail.iter())
    }
}

impl<T> NEVec<T> {
    /// Returns an iterator over the elements of the NEVec. The iterator is double-ended.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let v = NEVec::from((1, vec![2, 3]));
    /// let mut iter = v.iter();
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&3));
    /// assert_eq!(iter.next(), None);
    ///
    /// let v = ne_vec![1, 2, 3];
    /// let mut iter = v.iter();
    /// assert_eq!(iter.next_back(), Some(&3));
    /// assert_eq!(iter.next_back(), Some(&2));
    /// assert_eq!(iter.next_back(), Some(&1));
    /// assert_eq!(iter.next_back(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> iter::Chain<iter::Once<&T>, slice::Iter<T>> {
        iter::once(&self.head).chain(self.tail.iter())
    }
}