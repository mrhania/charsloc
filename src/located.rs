use ::Location;

/// An wrapper around an iterator of characters that also provides information
/// about the current location within a file.
///
/// # Examples
///
/// ``` rust
/// # use charsloc::Located;
/// let mut iter = Located::new("ab\nd".chars());
///
/// assert_eq!(iter.location().line, 1);
/// assert_eq!(iter.location().column, 1);
/// assert_eq!(iter.next(), Some('a'));
///
/// assert_eq!(iter.location().line, 1);
/// assert_eq!(iter.location().column, 2);
/// assert_eq!(iter.next(), Some('b'));
///
/// assert_eq!(iter.location().line, 1);
/// assert_eq!(iter.location().column, 3);
/// assert_eq!(iter.next(), Some('\n'));
///
/// assert_eq!(iter.location().line, 2);
/// assert_eq!(iter.location().column, 1);
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
