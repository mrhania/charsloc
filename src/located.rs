use ::Location;

/// An wrapper around an iterator of characters that also provides information
/// about the current location within a file.
///
/// # Examples
///
/// ``` rust
/// # use charsloc::{Located, Location};
/// let mut iter = Located::new("ab\nd".chars());
///
/// assert_eq!(iter.location(), Location { line: 1, column: 1 });
/// assert_eq!(iter.next(), Some('a'));
///
/// assert_eq!(iter.location(), Location { line: 1, column: 2 });
/// assert_eq!(iter.next(), Some('b'));
///
/// assert_eq!(iter.location(), Location { line: 1, column: 3 });
/// assert_eq!(iter.next(), Some('\n'));
///
/// assert_eq!(iter.location(), Location { line: 2, column: 1 });
/// assert_eq!(iter.next(), Some('d'));
/// ```
pub struct Located<I: Iterator> {
    iter: I,
    location: Location,
}

impl<I: Iterator> Located<I> {

    /// Constructs new iterator wrapper.
    #[inline]
    pub fn new(iter: I) -> Located<I> {
        Located {
            iter: iter,
            location: Location::start(),
        }
    }

    /// Retrieves the location of the next character being returned by the
    /// underlying iterator.
    #[inline]
    pub fn location(&self) -> Location {
        self.location
    }

    /// Returns reference to the underlying iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use charsloc::Located;
    /// let iter = Located::new("a".chars());
    /// assert_eq!(iter.iter().size_hint(), iter.size_hint());
    /// ```
    #[inline]
    pub fn iter(&self) -> &I {
        &self.iter
    }

    /// Returns mutable reference to the underlying iterator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use charsloc::Located;
    /// let mut iter = Located::new("a".chars().peekable());
    /// assert_eq!(iter.iter_mut().peek(), Some(&'a'));
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> &mut I {
        &mut self.iter
    }
}

impl<I: Iterator<Item=char>> Iterator for Located<I> {
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<I::Item> {
        let result = self.iter.next();
        if let Some(c) = result {
            if c == '\n' {
                self.location.next_line();
            } else {
                self.location.next_column();
            }
        }
        result
    }

    #[inline]
    fn count(self) -> usize {
        self.iter.count()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[cfg(test)]
macro_rules! assert_next_loc {
    ($iter:ident, $value:expr, line: $line:expr, column: $column:expr) => {{
        assert_eq!($iter.location().line, $line);
        assert_eq!($iter.location().column, $column);
        assert_eq!($iter.next(), Some($value));
    }}
}

#[test]
fn test_single_line() {
    let mut iter = Located::new("foo".chars());
    assert_next_loc!(iter, 'f', line: 1, column: 1);
    assert_next_loc!(iter, 'o', line: 1, column: 2);
    assert_next_loc!(iter, 'o', line: 1, column: 3);
}

#[test]
fn test_multi_line() {
    let mut iter = Located::new("a\nbc\nd".chars());
    assert_next_loc!(iter, 'a', line: 1, column: 1);
    assert_next_loc!(iter, '\n', line: 1, column: 2);
    assert_next_loc!(iter, 'b', line: 2, column: 1);
    assert_next_loc!(iter, 'c', line: 2, column: 2);
    assert_next_loc!(iter, '\n', line: 2, column: 3);
    assert_next_loc!(iter, 'd', line: 3, column: 1);
}

#[test]
fn test_carriage_newline() {
    let mut iter = Located::new("a\r\n\nb".chars());
    assert_next_loc!(iter, 'a', line: 1, column: 1);
    assert_next_loc!(iter, '\r', line: 1, column: 2);
    assert_next_loc!(iter, '\n', line: 1, column: 3);
    assert_next_loc!(iter, '\n', line: 2, column: 1);
    assert_next_loc!(iter, 'b', line: 3, column: 1);
}
