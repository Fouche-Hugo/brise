use std::{
    fmt::Display,
    num::NonZeroUsize,
    ops::{Add, AddAssign},
    path::Path,
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Line(NonZeroUsize);

impl Default for Line {
    fn default() -> Self {
        Self(NonZeroUsize::MIN)
    }
}

impl From<NonZeroUsize> for Line {
    fn from(value: NonZeroUsize) -> Self {
        Self(value)
    }
}

impl Add<Line> for Line {
    type Output = Line;

    fn add(self, rhs: Line) -> Self::Output {
        self + rhs.0.get()
    }
}

impl Add<usize> for Line {
    type Output = Line;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0.saturating_add(rhs))
    }
}

impl AddAssign for Line {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<usize> for Line {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Column(NonZeroUsize);

impl Default for Column {
    fn default() -> Self {
        Self(NonZeroUsize::MIN)
    }
}

impl From<NonZeroUsize> for Column {
    fn from(value: NonZeroUsize) -> Self {
        Self(value)
    }
}

impl Add<Column> for Column {
    type Output = Column;

    fn add(self, rhs: Column) -> Self::Output {
        Self(self.0.saturating_add(rhs.0.get()))
    }
}

impl Add<usize> for Column {
    type Output = Column;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0.saturating_add(rhs))
    }
}

impl AddAssign for Column {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl AddAssign<usize> for Column {
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct BriseFile(Rc<Path>);

impl<T: Into<Rc<Path>>> From<T> for BriseFile {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl BriseFile {
    pub fn as_path(&self) -> &Path {
        &self.0
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone)]
pub struct BriseContext {
    line: Line,
    col: Column,
    file: Option<BriseFile>,
}

impl Display for BriseContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(file) = &self.file {
            f.write_fmt(format_args!(
                "[{}, {}:{}]",
                file.0.display(),
                self.line.0,
                self.col.0
            ))
        } else {
            f.write_fmt(format_args!("[{}:{}]", self.line.0, self.col.0))
        }
    }
}

impl BriseContext {
    pub fn new(file: Option<BriseFile>, line: impl Into<Line>, col: impl Into<Column>) -> Self {
        Self {
            file,
            line: line.into(),
            col: col.into(),
        }
    }

    pub fn line(&self) -> Line {
        self.line
    }

    pub fn col(&self) -> Column {
        self.col
    }

    pub fn file(&self) -> &Option<BriseFile> {
        &self.file
    }
}
