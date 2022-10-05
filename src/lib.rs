#![doc = include_str!("../README.md")]

use std::fmt;

pub struct TruncatedSlice<'a, T> {
    max_len: usize,
    inner: &'a [T],
}

impl<'a, T> TruncatedSlice<'a, T> {
    pub fn new(wrapped: &'a [T], max_len: usize) -> Self {
        Self {
            max_len,
            inner: wrapped,
        }
    }
}

impl<'a, T> fmt::Debug for TruncatedSlice<'a, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dbg_len = std::cmp::min(self.inner.len(), self.max_len);
        let is_truncated = dbg_len < self.inner.len();

        f.write_str("[")?;
        for (i, entry) in self.inner[..dbg_len].iter().enumerate() {
            f.write_fmt(format_args!("{:?}", &entry))?;
            if i != dbg_len - 1 {
                f.write_str(", ")?;
            }
        }

        if is_truncated {
            f.write_fmt(format_args!(", ...({} more)", self.inner.len() - dbg_len))?;
        }

        f.write_str("]")
    }
}

pub trait TruncateSliceDebug<'a, T>
where
    T: fmt::Debug,
{
    /// For a collection of `T`, return a `TruncatedSlice` struct with a `Debug`
    /// implementation to only print `max_len` items
    fn truncate_debug(&self, max_len: usize) -> TruncatedSlice<'a, T>;
}

impl<'a, T> TruncateSliceDebug<'a, T> for &'a [T]
where
    T: fmt::Debug,
{
    /// `TruncateSliceDebug` implmenetation for any slice of items that implement `Debug`
    ///
    /// ```
    /// use truncate_slice_debug::TruncateSliceDebug;
    ///
    /// let values = vec![0, 1, 2, 3, 4, 5];
    ///
    /// let dbg_output = format!("{:?}", values.as_slice().truncate_debug(3));
    /// assert_eq!(&dbg_output, "[0, 1, 2, ...(3 more)]");
    ///
    /// let dbg_output = format!("{:?}", values.as_slice().truncate_debug(10));
    /// assert_eq!(&dbg_output, "[0, 1, 2, 3, 4, 5]");
    /// ```
    fn truncate_debug(&self, max_len: usize) -> TruncatedSlice<'a, T> {
        TruncatedSlice::new(self, max_len)
    }
}

#[test]
fn test_truncated_slice() {
    let values = vec![7; 10];

    let dbg10 = format!("{:?}", TruncatedSlice::new(&values, 5));
    assert_eq!(&dbg10, "[7, 7, 7, 7, 7, ...(5 more)]");

    let dbg10 = format!("{:?}", TruncatedSlice::new(&values, 20));
    assert_eq!(&dbg10, "[7, 7, 7, 7, 7, 7, 7, 7, 7, 7]");
}
