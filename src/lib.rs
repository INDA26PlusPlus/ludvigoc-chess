pub struct Pos{
    pub x: i8,
    pub y: i8
}

pub struct Pawn {
    pub is_white: bool,
    pub has_moved: bool,
    pub pos: Pos
}

