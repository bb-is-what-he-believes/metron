use std::fmt::Display;
use crate::Measure;

pub trait Symbol<L>{
    fn fmt(f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result;
}

impl<N, U> Display for Measure<N, U>
    where
        N: Display,
        U: crate::fmt::Symbol<()>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.num.fmt(f)?;
        f.write_str(" ")?;
        U::fmt(f)
    }
}