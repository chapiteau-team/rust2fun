use super::NEVec;

macro_rules! __impl_slice_eq {
    ([$($vars:tt)*] $rhs:ty) => {
        impl<T, U, $($vars)*> PartialEq<$rhs> for NEVec<T>
            where
                T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                self.len() == other.len() && self.head == other[0] && self.tail == &other[1..]
            }
        }

        impl<T, U, $($vars)*> PartialEq<NEVec<T>> for $rhs
            where
                T: PartialEq<U>,
        {
            #[inline]
            fn eq(&self, other: &NEVec<T>) -> bool {
                other == self
            }
        }
    };
}

__impl_slice_eq! { [] [U] }
__impl_slice_eq! { [] &[U] }
__impl_slice_eq! { [] &mut [U] }
__impl_slice_eq! { [const N: usize] [U; N] }
__impl_slice_eq! { [const N: usize] &[U; N] }

impl<T, U> PartialEq<NEVec<U>> for NEVec<T>
    where
        T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &NEVec<U>) -> bool {
        self.head == other.head && self.tail == other.tail
    }
}