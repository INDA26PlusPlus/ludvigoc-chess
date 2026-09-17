#[derive(Clone, Copy, PartialEq)]
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

#[derive(Clone)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub white_turn: bool,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            squares: [None; 64],
            white_turn: true,
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

    pub fn is_legal_move(&self, from: Pos, to: Pos) -> bool {
        let piece = match self.get(from) {
            Some(piece) => piece,
            None => return false,
        };

        // Check so that the piece has the same color as the player whose turn it is
        if piece.is_white != self.white_turn {
            return false;
        }

        // You cannot capture your own pieces
        if let Some(target) = self.get(to) {
            if target.is_white == piece.is_white {
                return false;
            }
        }

        let dx = (to.x as i8 - from.x as i8).abs();
        let dy = (to.y as i8 - from.y as i8).abs();

        let movement_is_valid = match piece.piece_type {
            PieceType::Knight => (dx == 2 && dy == 1) || (dx == 1 && dy == 2),

            PieceType::King => dx <= 1 && dy <= 1 && (dx != 0 || dy != 0),

            PieceType::Bishop => dx == dy && self.path_is_clear(from, to),

            PieceType::Rook => (dx == 0 || dy == 0) && self.path_is_clear(from, to),

            PieceType::Queen => (dx == dy || dx == 0 || dy == 0) && self.path_is_clear(from, to),

            PieceType::Pawn => {
                let direction: i8 = if piece.is_white { 1 } else { -1 };

                let move_y = to.y as i8 - from.y as i8;

                if dx == 0 && move_y == direction {
                    self.get(to).is_none()
                } else if dx == 0 && move_y == direction * 2 {
                    let middle_y = (from.y as i8 + direction) as u8;

                    from.y == if piece.is_white { 1 } else { 6 }
                        && self.get(to).is_none()
                        && self
                            .get(Pos {
                                x: from.x,
                                y: middle_y,
                            })
                            .is_none()
                } else if dx == 1 && move_y == direction {
                    match self.get(to) {
                        Some(target) => target.is_white != piece.is_white,
                        None => false,
                    }
                } else {
                    false
                }
            }
        };

        if !movement_is_valid {
            return false;
        }

        // Play the move on a test board
        let mut test_board = self.clone();
        test_board.move_piece(from, to);

        // A legal move cannot leave the player's own king in check
        !test_board.is_in_check(piece.is_white)
    }

    fn path_is_clear(&self, from: Pos, to: Pos) -> bool {
        let dx = (to.x as i8 - from.x as i8).signum();
        let dy = (to.y as i8 - from.y as i8).signum();

        let mut x = from.x as i8 + dx;
        let mut y = from.y as i8 + dy;

        while x != to.x as i8 || y != to.y as i8 {
            if self
                .get(Pos {
                    x: x as u8,
                    y: y as u8,
                })
                .is_some()
            {
                return false;
            }

            x += dx;
            y += dy;
        }

        true
    }

    fn is_in_check(&self, white: bool) -> bool {
        let mut king_position = None;

        for y in 0..8 {
            for x in 0..8 {
                let pos = Pos { x, y };

                if let Some(piece) = self.get(pos) {
                    if piece.is_white == white && piece.piece_type == PieceType::King {
                        king_position = Some(pos);
                    }
                }
            }
        }

        let king_position = match king_position {
            Some(pos) => pos,
            None => return true,
        };

        for y in 0..8 {
            for x in 0..8 {
                let from = Pos { x, y };

                if let Some(piece) = self.get(from) {
                    if piece.is_white != white {
                        if self.attacks_square(from, king_position) {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn attacks_square(&self, from: Pos, to: Pos) -> bool {
        let piece = match self.get(from) {
            Some(piece) => piece,
            None => return false,
        };

        let dx = (to.x as i8 - from.x as i8).abs();
        let dy = (to.y as i8 - from.y as i8).abs();

        match piece.piece_type {
            PieceType::Knight => (dx == 2 && dy == 1) || (dx == 1 && dy == 2),

            PieceType::King => dx <= 1 && dy <= 1,

            PieceType::Bishop => dx == dy && self.path_is_clear(from, to),

            PieceType::Rook => (dx == 0 || dy == 0) && self.path_is_clear(from, to),

            PieceType::Queen => (dx == dy || dx == 0 || dy == 0) && self.path_is_clear(from, to),

            PieceType::Pawn => {
                let direction: i8 = if piece.is_white { 1 } else { -1 };
                let move_y = to.y as i8 - from.y as i8;

                move_y == direction && dx == 1
            }
        }
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
