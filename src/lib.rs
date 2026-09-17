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
        let mut board = Board {
            squares: [None; 64],
        };

        // White pieces
        board.set(
            Pos { x: 0, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Rook,
            },
        );
        board.set(
            Pos { x: 1, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Knight,
            },
        );
        board.set(
            Pos { x: 2, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Bishop,
            },
        );
        board.set(
            Pos { x: 3, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Queen,
            },
        );
        board.set(
            Pos { x: 4, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::King,
            },
        );
        board.set(
            Pos { x: 5, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Bishop,
            },
        );
        board.set(
            Pos { x: 6, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Knight,
            },
        );
        board.set(
            Pos { x: 7, y: 0 },
            Piece {
                is_white: true,
                piece_type: PieceType::Rook,
            },
        );

        // White pawns
        for x in 0..8 {
            board.set(
                Pos { x, y: 1 },
                Piece {
                    is_white: true,
                    piece_type: PieceType::Pawn,
                },
            );
        }

        // Black pieces
        board.set(
            Pos { x: 0, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Rook,
            },
        );
        board.set(
            Pos { x: 1, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Knight,
            },
        );
        board.set(
            Pos { x: 2, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Bishop,
            },
        );
        board.set(
            Pos { x: 3, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Queen,
            },
        );
        board.set(
            Pos { x: 4, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::King,
            },
        );
        board.set(
            Pos { x: 5, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Bishop,
            },
        );
        board.set(
            Pos { x: 6, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Knight,
            },
        );
        board.set(
            Pos { x: 7, y: 7 },
            Piece {
                is_white: false,
                piece_type: PieceType::Rook,
            },
        );

        // Black pawns
        for x in 0..8 {
            board.set(
                Pos { x, y: 6 },
                Piece {
                    is_white: false,
                    piece_type: PieceType::Pawn,
                },
            );
        }

        board // Return the board with pieces set up
    }

    pub fn get(&self, pos: Pos) -> Option<Piece> {
        let index = (pos.y * 8 + pos.x) as usize;
        self.squares[index]
    }

    pub fn set(&mut self, pos: Pos, piece: Piece) {
        let index = (pos.y * 8 + pos.x) as usize;
        self.squares[index] = Some(piece);
    }

    pub fn move_piece(&mut self, from: Pos, to: Pos) {
        let piece = self.get(from);

        let from_index = (from.y * 8 + from.x) as usize;
        let to_index = (to.y * 8 + to.x) as usize;

        self.squares[from_index] = None;
        self.squares[to_index] = piece;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_piece_works() {
        let mut board = Board::new();

        board.move_piece(Pos { x: 4, y: 1 }, Pos { x: 4, y: 3 });

        assert!(board.get(Pos { x: 4, y: 1 }).is_none());
        assert!(board.get(Pos { x: 4, y: 3 }).is_some());
    }
}
