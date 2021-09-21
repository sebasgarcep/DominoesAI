#[derive(Copy, Clone)]
pub struct Piece {
    pub left: usize,
    pub right: usize,
}

impl Piece {
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