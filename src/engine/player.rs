use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive, PartialEq)]
#[repr(u8)]
pub enum Player {
    White = 8,
    Black = 16,
}

impl Player {
    pub fn opponent(&self) -> Self {
        match self {
            Player::White => Player::Black,
            Player::Black => Player::White,
        }
    }
}
