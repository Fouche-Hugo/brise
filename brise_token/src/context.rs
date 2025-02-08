use std::{path::Path, rc::Rc};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Line(usize);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Column(usize);

impl From<usize> for Column {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for Line {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct BriseFile(Rc<Path>);

impl<T: Into<Rc<Path>>> From<T> for BriseFile {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BriseContext {
    line: Line,
    col: Column,
    file: BriseFile,
}

impl BriseContext {
    pub fn new(file: BriseFile, line: impl Into<Line>, col: impl Into<Column>) -> Self {
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

    pub fn file(&self) -> &BriseFile {
        &self.file
    }
}
