use ::Location;

pub struct Located<I: Iterator> {
    iter: I,
    location: Location,
}

impl<I: Iterator> Located<I> {

    #[inline]
    pub fn new(iter: I) -> Located<I> {
        Located {
            iter: iter,
            location: Location::start(),
        }
    }

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
