pub(crate) trait FromSys<T> {
    fn from_sys(s: T) -> Self;
    fn into_sys(self) -> T;
}
