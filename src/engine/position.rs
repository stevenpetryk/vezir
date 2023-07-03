use super::{piece::Piece, player::Player};
use core::fmt::{Debug, Error, Formatter};

const DARK_SQUARE: &str = "\x1b[38;5;240m■ \x1b[0m";

#[derive(Debug)]
pub struct CastlingRights {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

// #[derive(Debug)]
pub struct Position {
    pub occupancies: [Option<Piece>; 64],
    pub to_move: Player,
    pub en_passant_square: Option<usize>,
    pub castling_rights: CastlingRights,
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut board_string = String::new();

        for (index, piece) in self.occupancies.iter().enumerate() {
            if index % 8 == 0 {
                board_string.push_str("\n");
            }

            let dark_square = ((index / 8) + (index % 8)) % 2 == 0;

            match piece {
                Some(piece) => board_string.push_str(&format!("{} ", piece.to_unicode())),
                None => board_string.push_str(if dark_square { DARK_SQUARE } else { "  " }),
            }
        }

        write!(f, "{}", board_string)
    }
}
