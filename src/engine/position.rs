use super::{game_move::GameMove, piece::Piece, player::Player};
use core::fmt;

const DARK_SQUARE: &str = "\x1b[38;5;240m■ \x1b[0m";

#[derive(Debug)]
pub struct CastlingRights {
    pub white_king: bool,
    pub white_queen: bool,
    pub black_king: bool,
    pub black_queen: bool,
}

#[derive(Debug)]
pub struct Position {
    pub occupancies: [Option<Piece>; 64],
    pub to_move: Player,
    pub en_passant_square: Option<usize>,
    pub castling_rights: CastlingRights,
}

impl Position {
    pub fn apply_move(&mut self, game_move: GameMove) -> &Self {
        self.occupancies[game_move.to.index] = self.occupancies[game_move.from.index];
        self.occupancies[game_move.from.index] = None;
        self
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
