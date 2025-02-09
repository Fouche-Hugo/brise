use std::{fmt::Display, rc::Rc};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RawString(Rc<str>);

impl Display for RawString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

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
