/// A pair of line and column representing a location within a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

impl Location {

    /// Constructs a location representing the beginning of a file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use charsloc::Location;
    ///
    /// let location = Location::start();
    /// assert_eq!(location.line, 1);
    /// assert_eq!(location.column, 1);
    /// ```
    #[inline]
    pub fn start() -> Location {
        Location {
            line: 1,
            column: 1,
        }
    }

    /// Advances a location to the new line and resets the column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use charsloc::Location;
    ///
    /// let mut location = Location { line: 3, column: 5 };
    /// location.next_line();
    /// assert_eq!(location.line, 4);
    /// assert_eq!(location.column, 1);
    /// ```
    #[inline]
    pub fn next_line(&mut self) {
        self.column = 1;
        self.line += 1;
    }

    /// Advances a location to the next column.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use charsloc::Location;
    ///
    /// let mut location = Location { line: 3, column: 5 };
    /// location.next_column();
    /// assert_eq!(location.line, 3);
    /// assert_eq!(location.column, 6);
    /// ```
    #[inline]
    pub fn next_column(&mut self) {
        self.column += 1;
    }
}
