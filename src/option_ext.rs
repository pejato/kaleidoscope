pub trait OptionExt {
    fn discard(&self) -> ();
}

impl<T> OptionExt for &Option<T> {
    fn discard(&self) -> () {}
}

impl<T> OptionExt for Option<T> {
    fn discard(&self) -> () {}
}
