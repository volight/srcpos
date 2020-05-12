#[cfg(test)]
mod tests;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use core::clone::Clone;
use core::cmp::{Eq, Ord, PartialEq, PartialOrd};
use core::convert::{From, Into};
use core::default::Default;
use core::fmt;
use core::fmt::{Debug, Display};
use core::hash::Hash;
use core::marker::Copy;
use core::ops::{Range, RangeFrom, RangeTo};

/// Posation in source code
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Pos {
    /// nth of line
    pub line: usize,
    /// nth of characters in current line
    pub column: usize,
}
impl Pos {
    /// New zero
    #[inline]
    pub const fn zero() -> Self {
        Self::new_same(0)
    }
    /// New at
    #[inline]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
    /// New same value
    #[inline]
    pub const fn new_same(value: usize) -> Self {
        Self::new(value, value)
    }
}

impl Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "at {}:{}", self.line, self.column)
    }
}

impl Default for Pos {
    #[inline(always)]
    fn default() -> Self {
        Self::zero()
    }
}

impl From<(usize, usize)> for Pos {
    #[inline]
    fn from((line, column): (usize, usize)) -> Self {
        Self::new(line, column)
    }
}
impl From<[usize; 2]> for Pos {
    #[inline]
    fn from([line, column]: [usize; 2]) -> Self {
        Self::new(line, column)
    }
}
impl Into<(usize, usize)> for Pos {
    #[inline]
    fn into(self) -> (usize, usize) {
        (self.line, self.column)
    }
}
impl Into<[usize; 2]> for Pos {
    #[inline]
    fn into(self) -> [usize; 2] {
        [self.line, self.column]
    }
}
impl From<usize> for Pos {
    #[inline]
    fn from(value: usize) -> Self {
        Self::new_same(value)
    }
}
impl Into<usize> for Pos {
    #[inline]
    fn into(self) -> usize {
        self.line
    }
}
impl From<[usize; 1]> for Pos {
    #[inline]
    fn from([value]: [usize; 1]) -> Self {
        Self::new_same(value)
    }
}
impl Into<[usize; 1]> for Pos {
    #[inline]
    fn into(self) -> [usize; 1] {
        [self.line]
    }
}
impl From<()> for Pos {
    #[inline]
    fn from(_: ()) -> Self {
        Self::zero()
    }
}
impl<T> From<[T; 0]> for Pos {
    #[inline]
    fn from(_: [T; 0]) -> Self {
        Self::zero()
    }
}

/// Shorthand for Pos::new
pub const fn pos(line: usize, column: usize) -> Pos {
    Pos::new(line, column)
}

//\/////////////////////////////////////////////////////////////////////////////////////////////////

/// Range of Posation in source code
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Loc {
    /// from
    pub from: Pos,
    /// to
    pub to: Pos,
}
impl Loc {
    /// New at
    #[inline]
    pub const fn new(from: Pos, to: Pos) -> Self {
        Self { from, to }
    }
    /// New at
    #[inline]
    pub const fn new_at(
        from_line: usize,
        from_column: usize,
        to_line: usize,
        to_column: usize,
    ) -> Self {
        Self::new(
            Pos::new(from_line, from_column),
            Pos::new(to_line, to_column),
        )
    }
    /// New zero
    #[inline]
    pub const fn zero() -> Self {
        Self::new_same_pos(Pos::zero())
    }
    /// New same value
    #[inline]
    pub const fn new_same(value: usize) -> Self {
        Self::new_same_pos(Pos::new_same(value))
    }
    /// New same value
    #[inline]
    pub const fn new_same_pos(pos: Pos) -> Self {
        Self::new(pos, pos)
    }
}

impl Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "at {}:{} to {}:{}",
            self.from.line, self.from.column, self.to.line, self.to.column,
        )
    }
}

impl Default for Loc {
    #[inline(always)]
    fn default() -> Self {
        Self::zero()
    }
}

impl From<(usize, usize, usize, usize)> for Loc {
    #[inline]
    fn from((a1, b1, a2, b2): (usize, usize, usize, usize)) -> Self {
        Self::new_at(a1, b1, a2, b2)
    }
}
impl Into<(usize, usize, usize, usize)> for Loc {
    #[inline]
    fn into(self) -> (usize, usize, usize, usize) {
        (
            self.from.line,
            self.from.column,
            self.to.line,
            self.to.column,
        )
    }
}
impl From<[usize; 4]> for Loc {
    #[inline]
    fn from([a1, b1, a2, b2]: [usize; 4]) -> Self {
        Self::new_at(a1, b1, a2, b2)
    }
}
impl Into<[usize; 4]> for Loc {
    fn into(self) -> [usize; 4] {
        [
            self.from.line,
            self.from.column,
            self.to.line,
            self.to.column,
        ]
    }
}
impl From<((usize, usize), (usize, usize))> for Loc {
    #[inline]
    fn from(((a1, b1), (a2, b2)): ((usize, usize), (usize, usize))) -> Self {
        Self::new_at(a1, b1, a2, b2)
    }
}
impl Into<((usize, usize), (usize, usize))> for Loc {
    #[inline]
    fn into(self) -> ((usize, usize), (usize, usize)) {
        (
            (self.from.line, self.from.column),
            (self.to.line, self.to.column),
        )
    }
}
impl From<[[usize; 2]; 2]> for Loc {
    #[inline]
    fn from([[a1, b1], [a2, b2]]: [[usize; 2]; 2]) -> Self {
        Self::new_at(a1, b1, a2, b2)
    }
}
impl Into<[[usize; 2]; 2]> for Loc {
    #[inline]
    fn into(self) -> [[usize; 2]; 2] {
        [
            [self.from.line, self.from.column],
            [self.to.line, self.to.column],
        ]
    }
}
impl From<[(usize, usize); 2]> for Loc {
    #[inline]
    fn from([(a1, b1), (a2, b2)]: [(usize, usize); 2]) -> Self {
        Self::new_at(a1, b1, a2, b2)
    }
}
impl Into<[(usize, usize); 2]> for Loc {
    #[inline]
    fn into(self) -> [(usize, usize); 2] {
        [
            (self.from.line, self.from.column),
            (self.to.line, self.to.column),
        ]
    }
}
impl From<([usize; 2], [usize; 2])> for Loc {
    #[inline]
    fn from(([a1, b1], [a2, b2]): ([usize; 2], [usize; 2])) -> Self {
        Self::new_at(a1, b1, a2, b2)
    }
}
impl Into<([usize; 2], [usize; 2])> for Loc {
    #[inline]
    fn into(self) -> ([usize; 2], [usize; 2]) {
        (
            [self.from.line, self.from.column],
            [self.to.line, self.to.column],
        )
    }
}
impl From<(usize, usize)> for Loc {
    #[inline]
    fn from((line, column): (usize, usize)) -> Self {
        Self::new_same_pos(Pos::new(line, column))
    }
}
impl Into<(usize, usize)> for Loc {
    fn into(self) -> (usize, usize) {
        (self.from.line, self.from.column)
    }
}
impl From<[usize; 2]> for Loc {
    #[inline]
    fn from([line, column]: [usize; 2]) -> Self {
        Self::new_same_pos(Pos::new(line, column))
    }
}
impl Into<[usize; 2]> for Loc {
    #[inline]
    fn into(self) -> [usize; 2] {
        [self.from.line, self.from.column]
    }
}
impl From<usize> for Loc {
    #[inline]
    fn from(value: usize) -> Self {
        Self::new_same(value)
    }
}
impl Into<usize> for Loc {
    #[inline]
    fn into(self) -> usize {
        self.from.line
    }
}
impl From<[usize; 1]> for Loc {
    #[inline]
    fn from([value]: [usize; 1]) -> Self {
        Self::new_same(value)
    }
}
impl Into<[usize; 1]> for Loc {
    #[inline]
    fn into(self) -> [usize; 1] {
        [self.from.line]
    }
}
impl From<()> for Loc {
    #[inline]
    fn from(_: ()) -> Self {
        Self::zero()
    }
}
impl<T> From<[T; 0]> for Loc {
    #[inline]
    fn from(_: [T; 0]) -> Self {
        Self::zero()
    }
}
impl From<Range<Pos>> for Loc {
    #[inline]
    fn from(r: Range<Pos>) -> Self {
        Self::new(r.start, r.end)
    }
}
impl Into<Range<Pos>> for Loc {
    #[inline]
    fn into(self) -> Range<Pos> {
        self.from..self.to
    }
}
impl From<RangeTo<Pos>> for Loc {
    #[inline]
    fn from(r: RangeTo<Pos>) -> Self {
        Self::new(Pos::zero(), r.end)
    }
}
impl Into<RangeTo<Pos>> for Loc {
    #[inline]
    fn into(self) -> RangeTo<Pos> {
        ..self.to
    }
}
impl Into<RangeFrom<Pos>> for Loc {
    #[inline]
    fn into(self) -> RangeFrom<Pos> {
        self.to..
    }
}
impl From<(Pos, Pos)> for Loc {
    #[inline]
    fn from((from, to): (Pos, Pos)) -> Self {
        Self::new(from, to)
    }
}
impl Into<(Pos, Pos)> for Loc {
    #[inline]
    fn into(self) -> (Pos, Pos) {
        (self.from, self.to)
    }
}
impl From<[Pos; 2]> for Loc {
    #[inline]
    fn from([from, to]: [Pos; 2]) -> Self {
        Self::new(from, to)
    }
}
impl Into<[Pos; 2]> for Loc {
    #[inline]
    fn into(self) -> [Pos; 2] {
        [self.from, self.to]
    }
}
impl From<Pos> for Loc {
    #[inline]
    fn from(pos: Pos) -> Self {
        Self::new_same_pos(pos)
    }
}
impl Into<Pos> for Loc {
    #[inline]
    fn into(self) -> Pos {
        self.from
    }
}
impl From<[Pos; 1]> for Loc {
    #[inline]
    fn from([pos]: [Pos; 1]) -> Self {
        Self::new_same_pos(pos)
    }
}
impl Into<[Pos; 1]> for Loc {
    #[inline]
    fn into(self) -> [Pos; 1] {
        [self.from]
    }
}

/// Shorthand for Loc::new
#[inline]
pub const fn loc(from: Pos, to: Pos) -> Loc {
    Loc::new(from, to)
}

/// Expands to the pos on which it was invoked.
/// ## See
/// - [`line!`](https://doc.rust-lang.org/std/macro.line.html)
/// - [`column!`](https://doc.rust-lang.org/std/macro.column.html)
/// # Examples
/// ```rust
/// # use srcpos::*;
/// let current_pos = pos!();
/// println!("defined on pos: {}", current_pos);
/// ```
#[macro_export]
macro_rules! pos {
    () => {
        $crate::pos(line!() as usize, column!() as usize)
    };
}
