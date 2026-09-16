#[derive(Clone, Copy)]
pub enum PieceType{
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

#[derive(Clone, Copy)]
pub struct Pos{
    pub x: u8,
    pub y: u8
}

#[derive(Clone, Copy)]
pub struct Piece {
    pub is_white: bool,
    pub piece_type: PieceType
}

