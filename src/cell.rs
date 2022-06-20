#[derive(Debug, Hash)]
pub struct Cell {
    row: u32,
    column: u32,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Eq for Cell {}

impl Cell {
    pub fn new(row: u32, column: u32) -> Cell {
        Cell { row, column }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cell({},{})", self.row, self.column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let cell = Cell::new(2, 3);
        assert_eq!(cell.row, 2);
        assert_eq!(cell.column, 3);
    }

    #[test]
    fn equals() {
        let cell1 = Cell::new(2, 3);
        let cell2 = Cell::new(2, 3);
        assert_eq!(cell1, cell2);
    }

    #[test]
    fn format() {
        let cell = Cell::new(2, 3);
        assert_eq!(format!("{}", cell), "cell(2,3)")
    }
}
