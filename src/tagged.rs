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
    let tagged = Tagged::new("foo".chars()).collect::<Vec<_>>();
    assert_eq!(tagged, vec!(
        ('f', Location { line: 1, column: 1 }),
        ('o', Location { line: 1, column: 2 }),
        ('o', Location { line: 1, column: 3 }),
    ));
}

#[test]
fn test_multi_line() {
    let tagged = Tagged::new("a\nbc\nd".chars()).collect::<Vec<_>>();
    assert_eq!(tagged, vec!(
        ('a', Location { line: 1, column: 1 }),
        ('\n', Location { line: 1, column: 2 }),
        ('b', Location { line: 2, column: 1 }),
        ('c', Location { line: 2, column: 2 }),
        ('\n', Location { line: 2, column: 3 }),
        ('d', Location { line: 3, column: 1 }),
    ));
}

#[test]
fn test_carriage_newline() {
    let tagged = Tagged::new("a\r\n\nb".chars()).collect::<Vec<_>>();
    assert_eq!(tagged, vec!(
        ('a', Location { line: 1, column: 1 }),
        ('\r', Location { line: 1, column: 2 }),
        ('\n', Location { line: 1, column: 3 }),
        ('\n', Location { line: 2, column: 1 }),
        ('b', Location { line: 3, column: 1 }),
    ));
    ;
}

#[test]
fn test_dots() {
    let tagged = Tagged::new("\n\n.\n.\n\n..\n".chars()).collect::<Vec<_>>();
    assert_eq!(tagged, vec!(
        ('\n', Location { line: 1, column: 1 }),
        ('\n', Location { line: 2, column: 1 }),
        ('.', Location { line: 3, column: 1 }),
        ('\n', Location { line: 3, column: 2 }),
        ('.', Location { line: 4, column: 1 }),
        ('\n', Location { line: 4, column: 2 }),
        ('\n', Location { line: 5, column: 1 }),
        ('.', Location { line: 6, column: 1 }),
        ('.', Location { line: 6, column: 2 }),
        ('\n', Location { line: 6, column: 3 }),
    ));
}
