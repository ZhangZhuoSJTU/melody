#![allow(dead_code)]

/// A half-open byte interval used by source-oriented experiments.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ByteSpan {
    start: usize,
    end: usize,
}

impl ByteSpan {
    /// Build a span only when its endpoints are in increasing order.
    pub(super) fn from_bounds(start: usize, end: usize) -> Option<Self> {
        (start <= end).then_some(Self { start, end })
    }

    /// Return the byte width; construction guarantees the subtraction is safe.
    pub(super) fn len(self) -> usize {
        self.end - self.start
    }

    /// Test membership using the usual half-open interval convention.
    pub(super) fn contains(self, offset: usize) -> bool {
        self.start <= offset && offset < self.end
    }
}
