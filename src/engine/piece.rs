use core::fmt::{Debug, Error, Formatter};

use crate::engine::player::Player;
use crate::engine::player::Player::*;

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
use PieceType::*;

pub struct Piece {
    piece_type: PieceType,
    color: Player,
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

    pub fn from_fen_char(char: char) -> Result<Piece, String> {
        let color = if char.is_lowercase() { Black } else { White };

        let piece_type = match char.to_lowercase().collect::<Vec<char>>()[0] {
            'p' => Pawn,
            'n' => Knight,
            'b' => Bishop,
            'r' => Rook,
            'q' => Queen,
            'k' => King,
            _ => return Err("Invalid piece type".to_string()),
        };

        Ok(Piece::new(color, piece_type))
    }

    pub fn to_fen_char(&self) -> char {
        let char = match self.piece_type {
            Pawn => 'p',
            Knight => 'n',
            Bishop => 'b',
            Rook => 'r',
            Queen => 'q',
            King => 'k',
        };

        let with_capitalization = match self.color {
            White => char.to_uppercase().collect::<Vec<char>>()[0],
            Black => char,
        };

        with_capitalization
    }
}
