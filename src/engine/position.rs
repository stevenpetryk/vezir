use super::{piece::Piece, player::Player};
use core::fmt::{Debug, Error, Formatter};

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

            match piece {
                Some(piece) => board_string.push_str(&format!("{}", piece.to_fen_char())),
                None => board_string.push_str("."),
            }
        }

        write!(f, "{}", board_string)
    }
}
