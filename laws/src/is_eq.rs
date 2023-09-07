pub struct IsEq<T> {
    lhs: T,
    rhs: T,
}

impl<T> IsEq<T> {
    pub fn equal_under_law(lhs: T, rhs: T) -> Self {
        IsEq { lhs, rhs }
    }
}

impl<T: Eq> IsEq<T> {
    pub fn holds(self) -> bool {
        self.lhs == self.rhs
    }
}
