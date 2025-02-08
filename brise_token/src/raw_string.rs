use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RawString(Rc<str>);

impl<T: Into<Rc<str>>> From<T> for RawString {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl RawString {
    pub fn new(raw_string: Rc<str>) -> Self {
        Self(raw_string)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
