use super::{game_move::GameMove, piece::Piece, player::Player};
use core::fmt;

const DARK_SQUARE: &str = "\x1b[38;5;240m■ \x1b[0m";

#[derive(Debug, Clone, Copy)]
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
    pub castling_rights: CastlingRights,
    pub en_passant_square: Option<usize>,
    pub halfmove_clock: usize,
    pub fullmove_counter: usize,
}

impl Position {
    pub fn apply_move(&self, game_move: GameMove) -> Position {
        let mut new_position = Position {
            occupancies: self.occupancies.clone(),
            to_move: self.to_move.opponent(),
            castling_rights: self.castling_rights,
            en_passant_square: None,
            halfmove_clock: self.halfmove_clock + 1,
            fullmove_counter: self.fullmove_counter + 1,
        };

        new_position.occupancies[game_move.to.index] =
            new_position.occupancies[game_move.from.index];
        new_position.occupancies[game_move.from.index] = None;
        new_position
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
