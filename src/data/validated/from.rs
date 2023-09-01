use super::*;

impl<T, E> From<Result<T, E>> for Validated<T, E> {
    #[inline]
    fn from(result: Result<T, E>) -> Self {
        match result {
            Ok(x) => Valid(x),
            Err(x) => Invalid(x),
        }
    }
}

impl<T, E> From<Validated<T, E>> for Result<T, E> {
    #[inline]
    fn from(validated: Validated<T, E>) -> Self {
        validated.into_result()
    }
}

if_std! {
    use crate::data::NEVec;

    impl<T, E> From<Validated<T, E>> for ValidatedNev<T, E> {
        #[inline]
        fn from(validated: Validated<T, E>) -> Self {
            validated.map_err(|e| NEVec::new(e))
        }
    }

    impl<T, E> From<Result<T, E>> for ValidatedNev<T, E> {
        #[inline]
        fn from(result: Result<T, E>) -> Self {
            match result {
                Ok(x) => Valid(x),
                Err(e) => Invalid(NEVec::new(e)),
            }
        }
    }
}
