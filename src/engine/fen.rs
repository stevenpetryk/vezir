use super::{
    player::Player::*,
    position::{CastlingRights, Position},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct FENParseError {
    fen: String,
}

impl fmt::Display for FENParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid FEN string: {}", self.fen)
    }
}

impl Position {
    pub fn from_fen(fen: &String) -> Result<Position, FENParseError> {
        let parse_error = FENParseError {
            fen: fen.to_string(),
        };

        let fen_parts: Vec<&str> = fen.split(" ").collect();

        let to_move = match fen_parts.get(1) {
            Some(&"w") => White,
            Some(&"b") => Black,
            _ => return Err(parse_error),
        };

        let castling_rights = {
            let castling_string = *fen_parts.get(2).unwrap();

            CastlingRights {
                white_king: castling_string.contains("K"),
                white_queen: castling_string.contains("Q"),
                black_king: castling_string.contains("k"),
                black_queen: castling_string.contains("q"),
            }
        };

        Ok(Position {
            occupancies: [const { None }; 64],
            to_move,
            en_passant_square: None,
            castling_rights,
        })
    }
}

#[cfg(test)]
mod tests {}
