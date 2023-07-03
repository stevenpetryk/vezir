use super::{
    piece::Piece,
    piece::PieceType::*,
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

        let occupancies = {
            let mut occupancies = [const { None }; 64];

            let mut index: usize = 0;
            let occupancy_string = *fen_parts.get(0).unwrap();

            for char in occupancy_string.chars() {
                match char {
                    '/' => {}
                    '0'..='8' => index += char.to_digit(10).unwrap() as usize,
                    _ => {
                        let piece = Piece::from_fen_char(char);
                        match piece {
                            Ok(piece) => {
                                occupancies[index] = Some(piece);
                                index += 1
                            }
                            Err(..) => return Err(parse_error),
                        }
                    }
                }
            }

            occupancies
        };

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
            occupancies,
            to_move,
            en_passant_square: None,
            castling_rights,
        })
    }
}

impl Piece {
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

#[cfg(test)]
mod tests {}
