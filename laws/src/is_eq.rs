pub struct IsEq<T: Eq> {
    lhs: T,
    rhs: T,
}

impl<T: Eq> IsEq<T> {
    pub fn equal_under_law(lhs: T, rhs: T) -> Self {
        IsEq { lhs, rhs }
    }

    pub fn holds(&self) -> bool {
        self.lhs == self.rhs
    }
}
