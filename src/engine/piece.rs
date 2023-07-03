use core::fmt::{Debug, Error, Formatter};

use crate::engine::player::Player;

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    pub piece_type: PieceType,
    pub color: Player,
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_fen_char())
    }
}

impl Piece {
    pub fn new(color: Player, piece_type: PieceType) -> Piece {
        Self {
            piece_type: piece_type,
            color: color,
        }
    }
}
