#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {

    #[inline]
    pub fn start() -> Location {
        Location {
            line: 1,
            column: 1,
        }
    }

    #[inline]
    pub fn next_line(&mut self) {
        self.column = 1;
        self.line += 1;
    }

    #[inline]
    pub fn next_column(&mut self) {
        self.column += 1;
    }
}
