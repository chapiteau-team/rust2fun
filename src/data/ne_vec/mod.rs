//! A non-empty growable vector.
//!
//! This vector is guaranteed to have at least one element.
//! The first element is known as the head, and the remaining elements are known as the tail:
//! ```
//! # #[allow(dead_code)]
//! pub struct NEVec<T> {
//!     pub head: T,
//!     pub tail: Vec<T>,
//! }
//! ```
//!
//! The tail may be empty.
//! The length of the NEVec is always at least one.
//!
//! # Examples
//!
//! You can create a `NEVec` with [`NEVec::new`]:
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! let nevec = NEVec::new(1);
//! ```
//!
//! ...or with the [`ne_vec!`] macro by providing at least one element:
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! let nevec = ne_vec![1, 2, 3];
//! let nevec = ne_vec![1; 3]; // equivalent to ne_vec![1, 1, 1]
//! ```
//!
//! You can push and pop elements onto the tail:
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! let mut nevec = ne_vec![1];
//! nevec.tail.push(2);
//! nevec.tail.push(3);
//! assert_eq!(nevec, [1, 2, 3]);
//!
//! assert_eq!(nevec.tail.pop(), Some(3));
//! assert_eq!(nevec, [1, 2]);
//! ```
//!
//! Non-empty vectors implement many of the same methods as [`Vec`] like ['NEVec::len`],
//! [`NEVec::first`], [`NEVec::last`], [`NEVec::get`], [`NEVec::get_mut`], [`NEVec::insert`],
//! [`NEVec::remove`], [`NEVec::swap_remove`], etc.
//!
//! # Iteration
//!
//! You can iterate over the elements of a `NEVec` with [`NEVec::iter`]:
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! let a = ne_vec!["1", "22", "333"];
//! let b = a.iter().map(|x| x.len()).collect::<NEVec<_>>();
//! assert_eq!(b, ne_vec![1, 2, 3]);
//!
//! let a = ne_vec![1, 2, 3];
//! let b = a.iter().map(|x| x.to_string()).collect::<NEVec<_>>();
//! assert_eq!(b, ne_vec!["1", "2", "3"]);
//! ```
//!
//! Note, `collect` panics if the iterator is empty.
//!
//! ```should_panic
//! use rust2fun::prelude::*;
//!
//! let _panics = ne_vec![1, 2, 3].into_iter().filter(|&x| x == 0).collect::<NEVec<_>>();
//! ```
//!
//! # Indexing
//!
//! Non-empty vectors support indexing (through the [`Index`] and [`IndexMut`] traits).
//! The `head` element is at index `0`, and the `tail` elements are at indices `1` through
//! `len() - 1`.
//!
//! ```
//! use rust2fun::prelude::*;
//!
//! let nevec = ne_vec![1, 2, 3];
//! assert_eq!(nevec[0], 1);
//! assert_eq!(nevec[1], 2);
//! assert_eq!(nevec[2], 3);
//! ```
//!
//! [`ne_vec!`]: crate::ne_vec
use core::num::NonZeroUsize;
use std::ops::{Index, IndexMut};
use std::vec::Vec;
use std::{mem, ptr, vec};

use crate::functor::Functor;
use crate::pure::Pure;
use crate::semigroup::Semigroup;
use crate::{
    and_then_flat_map, apply_iter, flatmap_iter, higher, invariant_functor, semigroup_extend,
    semigroupal_iter,
};

mod from;
mod iter;
mod partial_eq;

/// A non-empty vector. The first element is `head`, and the remaining elements are `tail`.
/// The length of the NEVec is always at least one. The tail may be empty.
///
/// See the [module-level documentation](self) for more details.
#[allow(clippy::len_without_is_empty)]
#[derive(Clone, Debug, Eq, Hash, PartialOrd, Ord)]
pub struct NEVec<T> {
    /// The first element of the NEVec, known as the head.
    /// This is always present.
    pub head: T,
    /// The remaining elements of the NEVec, known as the tail.
    /// This may be empty.
    pub tail: Vec<T>,
}

impl<T> NEVec<T> {
    /// Constructs a new `NEVec<T>`.
    /// The first element of the NEVec is `head`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = NEVec::new(1);
    /// assert_eq!(nevec.head, 1);
    /// assert_eq!(nevec.tail, []);
    /// assert_eq!(nevec, [1]);
    /// ```
    #[inline]
    pub const fn new(head: T) -> Self {
        Self {
            head,
            tail: Vec::new(),
        }
    }

    /// Constructs a new `NEVec<T>` with the given `head` and the capacity for `tail`.
    /// The tail will be empty.
    /// The capacity for `tail` is a lower bound; the `NEVec<T>` may hold more, but will not
    /// reallocate until it exceeds this value.
    /// If `tail_capacity` is `0`, the tail will not allocate.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = NEVec::with_tail_capacity(1, 2);
    /// assert_eq!(nevec.head, 1);
    /// assert_eq!(nevec.tail, []);
    /// assert_eq!(nevec.tail.capacity(), 2);
    /// assert_eq!(nevec, [1]);
    /// ```
    #[inline]
    pub fn with_tail_capacity(head: T, tail_capacity: usize) -> Self {
        Self {
            head,
            tail: Vec::with_capacity(tail_capacity),
        }
    }

    /// Constructs a new `NEVec<T>` with the given element repeated `n` times.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::num::NonZeroUsize;
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = NEVec::from_elem(1, NonZeroUsize::new(3).unwrap());
    /// assert_eq!(nevec.head, 1);
    /// assert_eq!(nevec.tail, [1, 1]);
    /// assert_eq!(nevec, [1, 1, 1]);
    /// ```
    #[inline]
    pub fn from_elem(elem: T, n: NonZeroUsize) -> Self
    where
        T: Clone,
    {
        Self {
            head: elem.clone(),
            tail: vec![elem; n.get() - 1],
        }
    }

    /// Constructs a new `NEVec<T>` from a given [`Vec<T>`].
    /// Returns `None` if the given `Vec<T>` is empty.
    /// Otherwise, returns `Some(nevec)`, where `nevec` is the `NEVec<T>` constructed from the
    /// given `Vec<T>`.
    ///
    /// [`Vec<T>`]: std::vec::Vec
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(NEVec::from_vec(vec![1, 2, 3]), Some(ne_vec![1, 2, 3]));
    /// assert_eq!(NEVec::<bool>::from_vec(vec![]), None);
    /// ```
    #[inline]
    pub fn from_vec(mut vec: Vec<T>) -> Option<Self> {
        if vec.is_empty() {
            None
        } else {
            Some(Self {
                head: vec.remove(0),
                tail: vec,
            })
        }
    }

    /// Constructs a new `NEVec<T>` from a given slice.
    /// Returns `None` if the given slice is empty.
    /// Otherwise, returns `Some(nevec)`, where `nevec` is the `NEVec<T>` constructed from the
    /// given slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// assert_eq!(NEVec::from_slice(&[1, 2, 3]), Some(ne_vec![1, 2, 3]));
    /// assert_eq!(NEVec::<bool>::from_slice(&[]), None);
    /// ```
    #[inline]
    pub fn from_slice(slice: &[T]) -> Option<Self>
    where
        T: Clone,
    {
        slice.split_first().map(|(h, t)| Self {
            head: h.clone(),
            tail: t.to_vec(),
        })
    }

    /// Removes the element at the given index and returns it.
    ///
    /// The removed element is replaced by the last element of the NEVec.
    ///
    /// This does not preserve ordering, but is O(1).
    /// If you need to preserve ordering, use [`remove`] instead.
    ///
    /// [`remove`]: NEVec::remove
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = NEVec::new(0);
    /// nevec.swap_remove(1);
    /// ```
    ///
    /// Panics if the NEVec contains only one element.
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = NEVec::new(0);
    /// nevec.swap_remove(0);
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = ne_vec![1, 2, 3, 4];
    ///
    /// assert_eq!(nevec.swap_remove(1), 2);
    /// assert_eq!(nevec, ne_vec![1, 4, 3]);
    ///
    /// assert_eq!(nevec.swap_remove(0), 1);
    /// assert_eq!(nevec, ne_vec![3, 4]);
    ///
    /// assert_eq!(nevec.swap_remove(1), 4);
    /// assert_eq!(nevec, ne_vec![3]);
    /// ```
    #[inline]
    pub fn swap_remove(&mut self, index: usize) -> T {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("swap_remove index (is {index}) should be < len (is {len})");
        }

        let dst = if index == 0 {
            if self.tail.is_empty() {
                non_empty_invariant_failed();
            }

            &mut self.head
        } else {
            if index >= self.len() {
                assert_failed(index, self.len());
            };

            unsafe { self.tail.as_mut_ptr().add(index - 1) }
        };

        unsafe {
            let result = ptr::read(dst);
            let new_tail_len = self.tail.len() - 1;
            ptr::copy(self.tail.as_ptr().add(new_tail_len), dst, 1);
            self.tail.set_len(new_tail_len);
            result
        }
    }

    /// Inserts an element at position `index` within the NEVec, shifting all
    /// elements after it to the right.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the length of the NEVec.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = ne_vec![1];
    /// nevec.insert(1, 2);
    /// assert_eq!(nevec, [1, 2]);
    ///
    /// let mut nevec = ne_vec![1];
    /// nevec.insert(0, 2);
    /// assert_eq!(nevec, [2, 1]);
    /// nevec.insert(2, 3);
    /// assert_eq!(nevec, [2, 1, 3]);
    /// nevec.insert(0, 4);
    /// assert_eq!(nevec, [4, 2, 1, 3]);
    /// nevec.insert(1, 5);
    /// assert_eq!(nevec, [4, 5, 2, 1, 3]);
    /// ```
    #[inline]
    pub fn insert(&mut self, index: usize, element: T) {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("insertion index (is {index}) should be <= len (is {len})");
        }

        if index == 0 {
            self.tail.insert(0, mem::replace(&mut self.head, element));
        } else if index <= self.len() {
            self.tail.insert(index - 1, element);
        } else {
            assert_failed(index, self.len());
        }
    }

    /// Removes and returns the element at position `index` within the NEVec,
    /// shifting all elements after it to the left.
    ///
    /// Note that this function is O(n) because it needs to shift all elements
    /// in the NEVec by one to the left.If you don't need to preserve ordering,
    /// use [`swap_remove`] instead.
    ///
    /// [`swap_remove`]: NEVec::swap_remove
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = NEVec::new(0);
    /// nevec.remove(1);
    /// ```
    ///
    /// Panics if the NEVec contains only one element.
    ///
    /// ```should_panic
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = NEVec::new(0);
    /// nevec.remove(0);
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = ne_vec![1, 2, 3];
    /// assert_eq!(nevec.remove(0), 1);
    /// assert_eq!(nevec, [2, 3]);
    ///
    /// assert_eq!(nevec.remove(1), 3);
    /// assert_eq!(nevec, [2]);
    /// ```
    #[inline]
    pub fn remove(&mut self, index: usize) -> T {
        #[cold]
        #[inline(never)]
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {index}) should be < len (is {len})");
        }

        if index == 0 {
            if self.tail.is_empty() {
                non_empty_invariant_failed();
            }

            mem::replace(&mut self.head, self.tail.remove(0))
        } else if index < self.len() {
            self.tail.remove(index - 1)
        } else {
            assert_failed(index, self.len());
        }
    }

    /// Returns the number of elements in the NEVec, including the head.
    /// This is always at least one.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = ne_vec![1, 2, 3];
    /// assert_eq!(nevec.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.tail.len() + 1
    }

    /// Returns the first element of the NEVec. This is always the head.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = ne_vec![1, 2, 3];
    /// assert_eq!(nevec.first(), &1);
    /// ```
    #[inline]
    pub fn first(&self) -> &T {
        &self.head
    }

    /// Returns a mutable reference to the first element of the NEVec. This is always the head.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = ne_vec![1, 2, 3];
    /// *nevec.first_mut() = 4;
    /// assert_eq!(nevec, [4, 2, 3]);
    /// ```
    #[inline]
    pub fn first_mut(&mut self) -> &mut T {
        &mut self.head
    }

    /// Returns the last element of the NEVec. If the NEVec has length `1`, this is the head.
    /// Otherwise, it is the last element of the tail.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = ne_vec![1, 2, 3];
    /// assert_eq!(nevec.last(), &3);
    ///
    /// let nevec = ne_vec![1];
    /// assert_eq!(nevec.last(), &1);
    /// ```
    #[inline]
    pub fn last(&self) -> &T {
        self.tail.last().unwrap_or(&self.head)
    }

    /// Returns a mutable reference to the last element of the NEVec. If the NEVec has length `1`,
    /// this is the head. Otherwise, it is the last element of the tail.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = ne_vec![1, 2, 3];
    /// *nevec.last_mut() = 4;
    /// assert_eq!(nevec, [1, 2, 4]);
    ///
    /// let mut nevec = ne_vec![1];
    /// *nevec.last_mut() = 2;
    /// assert_eq!(nevec, [2]);
    /// ```
    #[inline]
    pub fn last_mut(&mut self) -> &mut T {
        self.tail.last_mut().unwrap_or(&mut self.head)
    }

    /// Returns a reference to an element, or `None` if out of bounds.
    /// If the index is `0`, this is the head.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = ne_vec![1, 2, 3];
    /// assert_eq!(nevec.get(0), Some(&1));
    /// assert_eq!(nevec.get(2), Some(&3));
    /// assert_eq!(nevec.get(3), None);
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 {
            Some(&self.head)
        } else {
            self.tail.get(index - 1)
        }
    }

    /// Returns a mutable reference to an element, or `None` if out of bounds.
    /// If the index is `0`, this is the head.
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let mut nevec = ne_vec![1, 2, 3];
    /// assert_eq!(nevec.get_mut(0), Some(&mut 1));
    /// assert_eq!(nevec.get_mut(2), Some(&mut 3));
    /// assert_eq!(nevec.get_mut(3), None);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index == 0 {
            Some(&mut self.head)
        } else {
            self.tail.get_mut(index - 1)
        }
    }

    /// Copies `self` into a new [`Vec`].
    ///
    /// [`Vec`]: std::vec::Vec
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = ne_vec![1, 2, 3];
    /// let vec = nevec.to_vec();
    /// assert_eq!(vec, [1, 2, 3]);
    /// assert_eq!(nevec, [1, 2, 3]);
    /// ```
    #[inline]
    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut vec = Vec::with_capacity(self.len());
        vec.push(self.head.clone());
        vec.extend_from_slice(&self.tail);
        vec
    }

    /// Converts `self` into a [`Vec`].
    ///
    /// [`Vec`]: std::vec::Vec
    ///
    /// # Examples
    ///
    /// ```
    /// use rust2fun::prelude::*;
    ///
    /// let nevec = ne_vec![1, 2, 3];
    /// let vec = nevec.into_vec();
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    #[inline]
    pub fn into_vec(self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.len());
        vec.push(self.head);
        vec.extend(self.tail);
        vec
    }
}

impl<T: Default> Default for NEVec<T> {
    #[inline]
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> Extend<T> for NEVec<T> {
    #[inline]
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.tail.extend(iter);
    }
}

impl<T> Index<usize> for NEVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 {
            &self.head
        } else {
            &self.tail[index - 1]
        }
    }
}

impl<T> IndexMut<usize> for NEVec<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.head
        } else {
            &mut self.tail[index - 1]
        }
    }
}

#[cold]
#[inline(never)]
fn non_empty_invariant_failed() -> ! {
    panic!("NEVec cannot be empty");
}

/// Creates a [`NEVec`] containing the arguments.
///
/// `ne_vec!` allows `NEVec`s to be defined with the same syntax as array expressions.
/// There are two forms of this macro:
///
/// - Create a [`NEVec`] containing a given list of elements:
///
/// ```
/// use rust2fun::prelude::*;
///
/// let nevec = ne_vec![1, 2, 3];
/// assert_eq!(nevec.head, 1);
/// assert_eq!(nevec.tail, [2, 3]);
/// ```
///
/// - Create a [`NEVec`] from a given element and size:
///
/// ```
/// use rust2fun::prelude::*;
///
/// let nevec = ne_vec![1; 3];
/// assert_eq!(nevec, [1, 1, 1]);
/// ```
///
/// Note that unlike array expressions this syntax supports all elements
/// which implement [`Clone`] and the number of elements doesn't have to be
/// a constant, but it has to be nonzero.
///
/// This will use `clone` to duplicate an expression, so one should be careful
/// using this with types having a nonstandard `Clone` implementation. For
/// example, `ne_vec![Rc::new(1); 5]` will create a vector of five references
/// to the same boxed integer value, not five references pointing to independently
/// boxed integers.
///
/// Also, note that `ne_vec![expr; 0]` is not allowed, and will panic, because
/// it violates the invariant that a `NEVec` cannot be empty.
#[macro_export]
macro_rules! ne_vec {
    ($head:expr) => (
        $crate::data::ne_vec::NEVec::new($head)
    );
    ($head:expr, $($tail:expr),* $(,)?) => (
        $crate::data::ne_vec::NEVec {
            head: $head,
            tail: vec![$($tail),*],
        }
    );
    ($elem:expr; $n:expr) => (
        $crate::data::ne_vec::NEVec::from_elem(
            $elem,
            core::num::NonZeroUsize::new($n).expect("NEVec cannot be empty"))
    );
}

higher!(NEVec);
apply_iter!(NEVec);
flatmap_iter!(NEVec);
semigroupal_iter!(NEVec);
semigroup_extend!(NEVec);
invariant_functor!(NEVec<T>);
and_then_flat_map!(NEVec<T>);

impl<A, B> Functor<B> for NEVec<A> {
    #[inline]
    fn map(self, mut f: impl FnMut(A) -> B) -> NEVec<B> {
        NEVec {
            head: f(self.head),
            tail: self.tail.map(f),
        }
    }
}

impl<T> Pure for NEVec<T> {
    #[inline]
    fn pure(x: T) -> Self {
        NEVec::new(x)
    }
}
