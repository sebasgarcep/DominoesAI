#[derive(Copy, Clone)]
pub struct Piece {
    pub left: usize,
    pub right: usize,
}

impl Piece {
    pub fn new(left: usize, right: usize) -> Self {
        Piece {
            left,
            right,
        }
    }

    pub fn reverse(&self) -> Piece {
        Piece {
            left: self.right,
            right: self.left,
        }
    }

    pub fn value(&self) -> usize {
        self.left + self.right
    }
}

impl PartialEq for Piece {
    fn eq(&self, other: &Self) -> bool {
        (self.left == other.left && self.right == other.right) ||
        (self.left == other.right && self.right == other.left)
    }
}

impl Eq for Piece {}