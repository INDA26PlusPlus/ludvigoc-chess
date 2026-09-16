#[derive(Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub is_white: bool,
    pub piece_type: PieceType,
}

pub struct Board {
    pub squares: [Option<Piece>; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [None; 64],
        }
    }
    pub fn get(&self, pos: Pos) -> Option<Piece> {
        let index = (pos.y * 8 + pos.x) as usize;
        self.squares[index]
    }
}
