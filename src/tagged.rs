use ::{Located, Location};

/// An iterator of characters that tags each returned character with the
/// location at which it occurs within the file.
///
/// # Examples
///
/// ``` rust
/// # use charsloc::{Location, Tagged};
/// let mut iter = Tagged::new("ab\ncd".chars());
///
/// assert_eq!(iter.next(), Some(('a', Location { line: 1, column: 1 })));
/// assert_eq!(iter.next(), Some(('b', Location { line: 1, column: 2 })));
/// assert_eq!(iter.next(), Some(('\n', Location { line: 1, column: 3 })));
/// assert_eq!(iter.next(), Some(('c', Location { line: 2, column: 1 })));
/// assert_eq!(iter.next(), Some(('d', Location { line: 2, column: 2 })));
/// ```
pub struct Tagged<I: Iterator> {
    iter: Located<I>,
}

impl<I: Iterator> Tagged<I> {

    /// Constructs new iterator wrapper.
    #[inline]
    pub fn new(iter: I) -> Tagged<I> {
        Tagged {
            iter: Located::new(iter),
        }
    }
}

impl<I: Iterator<Item=char>> Iterator for Tagged<I> {
    type Item = (I::Item, Location);

    #[inline]
    fn next(&mut self) -> Option<(I::Item, Location)> {
        let location = self.iter.location();
        self.iter.next().map(|item| (item, location))
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

#[test]
fn test_single_line() {
    let mut iter = Tagged::new("foo".chars());
    assert_eq!(iter.next(), Some(('f', Location { line: 1, column: 1 })));
    assert_eq!(iter.next(), Some(('o', Location { line: 1, column: 2 })));
    assert_eq!(iter.next(), Some(('o', Location { line: 1, column: 3 })));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_multi_line() {
    let mut iter = Tagged::new("a\nbc\nd".chars());
    assert_eq!(iter.next(), Some(('a', Location { line: 1, column: 1 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 1, column: 2 })));
    assert_eq!(iter.next(), Some(('b', Location { line: 2, column: 1 })));
    assert_eq!(iter.next(), Some(('c', Location { line: 2, column: 2 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 2, column: 3 })));
    assert_eq!(iter.next(), Some(('d', Location { line: 3, column: 1 })));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_carriage_newline() {
    let mut iter = Tagged::new("a\r\n\nb".chars());
    assert_eq!(iter.next(), Some(('a', Location { line: 1, column: 1 })));
    assert_eq!(iter.next(), Some(('\r', Location { line: 1, column: 2 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 1, column: 3 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 2, column: 1 })));
    assert_eq!(iter.next(), Some(('b', Location { line: 3, column: 1 })));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_dots() {
    let mut iter = Tagged::new("\n\n.\n.\n\n..\n".chars());
    assert_eq!(iter.next(), Some(('\n', Location { line: 1, column: 1 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 2, column: 1 })));
    assert_eq!(iter.next(), Some(('.', Location { line: 3, column: 1 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 3, column: 2 })));
    assert_eq!(iter.next(), Some(('.', Location { line: 4, column: 1 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 4, column: 2 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 5, column: 1 })));
    assert_eq!(iter.next(), Some(('.', Location { line: 6, column: 1 })));
    assert_eq!(iter.next(), Some(('.', Location { line: 6, column: 2 })));
    assert_eq!(iter.next(), Some(('\n', Location { line: 6, column: 3 })));
    assert_eq!(iter.next(), None);
}
