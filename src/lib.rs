#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy, PartialEq)]
pub struct Piece {
    pub is_white: bool,
    pub piece_type: PieceType,
}

#[derive(Clone)]
pub struct Board {
    pub squares: [Option<Piece>; 64],
    pub white_turn: bool,

    pub white_king_moved: bool,
    pub black_king_moved: bool,
    pub white_rook_left_moved: bool,
    pub white_rook_right_moved: bool,
    pub black_rook_left_moved: bool,
    pub black_rook_right_moved: bool,

    pub en_passant: Option<Pos>,
}

impl Board {
    pub fn new() -> Board {
        let mut board = Board {
            squares: [None; 64],
            white_turn: true,
            white_king_moved: false,
            black_king_moved: false,
            white_rook_left_moved: false,
            white_rook_right_moved: false,
            black_rook_left_moved: false,
            black_rook_right_moved: false,
            en_passant: None,
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

    // Functions for getting information about the board state
    pub fn white_to_move(&self) -> bool {
        self.white_turn
    }

    pub fn piece_at(&self, pos: Pos) -> Option<Piece> {
        self.get(pos)
    }

    fn move_piece(&mut self, from: Pos, to: Pos) {
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
            if target.piece_type == PieceType::King {
                return false;
            }
        }

        if piece.piece_type == PieceType::King
            && (to.x as i8 - from.x as i8).abs() == 2
            && from.y == to.y
        {
            return self.can_castle(from, to);
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

    pub fn make_move(
        &mut self,
        from: Pos,
        to: Pos,
        promotion: Option<PieceType>,
    ) -> bool {
        if !self.is_legal_move(from, to) {
            return false;
        }

        let piece = match self.get(from) {
            Some(piece) => piece,
            None => return false,
        };

        let is_castling = piece.piece_type == PieceType::King
            && (to.x as i8 - from.x as i8).abs() == 2;

        self.move_piece(from, to);

        // Mark the king as having moved
        if piece.piece_type == PieceType::King {
            if piece.is_white {
                self.white_king_moved = true;
            } else {
                self.black_king_moved = true;
            }
        }

        // Mark rooks as having moved
        if piece.piece_type == PieceType::Rook {
            if piece.is_white {
                if from == (Pos { x: 0, y: 0 }) {
                    self.white_rook_left_moved = true;
                }

                if from == (Pos { x: 7, y: 0 }) {
                    self.white_rook_right_moved = true;
                }
            } else {
                if from == (Pos { x: 0, y: 7 }) {
                    self.black_rook_left_moved = true;
                }

                if from == (Pos { x: 7, y: 7 }) {
                    self.black_rook_right_moved = true;
                }
            }
        }

        // Move the rook when castling
        if is_castling {
            let y = from.y;

            if to.x == 6 {
                self.move_piece(
                    Pos { x: 7, y },
                    Pos { x: 5, y },
                );
            } else if to.x == 2 {
                self.move_piece(
                    Pos { x: 0, y },
                    Pos { x: 3, y },
                );
            }
        }

        // Promotion
        if piece.piece_type == PieceType::Pawn && (to.y == 0 || to.y == 7) {
            let new_type = match promotion {
                Some(piece_type) => piece_type,
                None => PieceType::Queen,
            };

            self.squares[(to.y * 8 + to.x) as usize] = Some(Piece {
                is_white: piece.is_white,
                piece_type: new_type,
            });
        }

        self.white_turn = !self.white_turn;

        true
    }
    fn can_castle(&self, from: Pos, to: Pos) -> bool {
        let piece = match self.get(from) {
            Some(piece) => piece,
            None => return false,
        };

        if piece.piece_type != PieceType::King {
            return false;
        }

        if self.is_in_check(piece.is_white) {
            return false;
        }

        let y = if piece.is_white { 0 } else { 7 };

        // Short castle
        if from == (Pos { x: 4, y }) && to == (Pos { x: 6, y }) {
            if piece.is_white && self.white_king_moved {
                return false;
            }

            if !piece.is_white && self.black_king_moved {
                return false;
            }

            if piece.is_white && self.white_rook_right_moved {
                return false;
            }

            if !piece.is_white && self.black_rook_right_moved {
                return false;
            }

            if self.get(Pos { x: 5, y }).is_some()
                || self.get(Pos { x: 6, y }).is_some()
            {
                return false;
            }

            if self.get(Pos { x: 7, y })
                != Some(Piece {
                    is_white: piece.is_white,
                    piece_type: PieceType::Rook,
                })
            {
                return false;
            }

            if self.square_is_attacked(Pos { x: 5, y }, !piece.is_white)
                || self.square_is_attacked(Pos { x: 6, y }, !piece.is_white)
            {
                return false;
            }

            return true;
        }

        // Long castle
        if from == (Pos { x: 4, y }) && to == (Pos { x: 2, y }) {
            if piece.is_white && self.white_king_moved {
                return false;
            }

            if !piece.is_white && self.black_king_moved {
                return false;
            }

            if piece.is_white && self.white_rook_left_moved {
                return false;
            }

            if !piece.is_white && self.black_rook_left_moved {
                return false;
            }

            if self.get(Pos { x: 1, y }).is_some()
                || self.get(Pos { x: 2, y }).is_some()
                || self.get(Pos { x: 3, y }).is_some()
            {
                return false;
            }

            if self.get(Pos { x: 0, y })
                != Some(Piece {
                    is_white: piece.is_white,
                    piece_type: PieceType::Rook,
                })
            {
                return false;
            }

            if self.square_is_attacked(Pos { x: 3, y }, !piece.is_white)
                || self.square_is_attacked(Pos { x: 2, y }, !piece.is_white)
            {
                return false;
            }

            return true;
        }

        false
    }

    fn square_is_attacked(&self, pos: Pos, by_white: bool) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                let from = Pos { x, y };

                if let Some(piece) = self.get(from) {
                    if piece.is_white == by_white
                        && self.attacks_square(from, pos)
                    {
                        return true;
                    }
                }
            }
        }

        false
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

        let moved = board.make_move(
            Pos { x: 4, y: 1 },
            Pos { x: 4, y: 3 },
            None,
        );

        assert!(moved);
        assert!(board.get(Pos { x: 4, y: 1 }).is_none());
        assert!(board.get(Pos { x: 4, y: 3 }).is_some());
    }

    #[test]
    fn make_move_changes_turn() {
        let mut board = Board::new();

        let moved = board.make_move(Pos { x: 4, y: 1 }, Pos { x: 4, y: 3 }, None);

        assert!(moved);
        assert!(!board.white_turn);
    }

    #[test]
    fn cannot_move_wrong_color() {
        let board = Board::new();

        assert!(!board.is_legal_move(Pos { x: 0, y: 6 }, Pos { x: 0, y: 5 },));
    }

    #[test]
    fn knight_can_jump() {
        let board = Board::new();

        assert!(board.is_legal_move(Pos { x: 1, y: 0 }, Pos { x: 2, y: 2 },));
    }

    #[test]
    fn cannot_move_through_piece() {
        let board = Board::new();

        assert!(!board.is_legal_move(Pos { x: 0, y: 0 }, Pos { x: 0, y: 3 },));
    }
}
