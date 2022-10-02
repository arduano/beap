#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BeapCoordinate {
    row: usize,
    pos: usize,
}

impl BeapCoordinate {
    pub(super) fn new(row: usize, pos: usize) -> Self {
        Self { row, pos }
    }

    // Same as the new function but checks that the coords are valid
    pub fn from_coords(row: usize, pos: usize) -> Option<Self> {
        if pos > row {
            None
        } else {
            Some(Self { row, pos })
        }
    }

    pub fn from_index(index: usize) -> Self {
        // subtract row sizes until fits
        let mut row = 1;
        let mut pos = index;
        while pos >= row {
            pos -= row;
            row += 1;
        }
        Self { row: row - 1, pos }
    }

    pub fn zero() -> Self {
        Self::new(0, 0)
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn array_index(&self) -> usize {
        self.row * (self.row + 1) / 2 + self.pos
    }

    pub fn left_parent(&self) -> Option<Self> {
        if self.row == 0 {
            return None;
        }
        if self.pos == 0 {
            return None;
        }
        Some(Self::new(self.row - 1, self.pos - 1))
    }

    pub fn right_parent(&self) -> Option<Self> {
        if self.row == 0 {
            return None;
        }
        if self.pos == self.row {
            return None;
        }
        Some(Self::new(self.row - 1, self.pos))
    }

    pub fn left_child(&self) -> Self {
        let pos = self.pos;
        Self::new(self.row + 1, pos)
    }

    pub fn right_child(&self) -> Self {
        let pos = self.pos;
        Self::new(self.row + 1, pos + 1)
    }
}
