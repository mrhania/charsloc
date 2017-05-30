use ::{Located, Location};

pub struct Tagged<I: Iterator> {
    iter: Located<I>,
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
