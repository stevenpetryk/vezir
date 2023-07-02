#[derive(Debug)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}
use core::fmt::{Debug, Error, Formatter};

use PieceType::*;

#[derive(Debug)]
enum Color {
    White,
    Black,
}
use Color::*;

struct Piece {
    piece_type: PieceType,
    color: Color,
}

impl Debug for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.to_fen_char())
    }
}

impl Piece {
    fn new(color: Color, piece_type: PieceType) -> Piece {
        Self {
            piece_type: piece_type,
            color: color,
        }
    }

    fn from_fen_char(char: char) -> Result<Piece, String> {
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

    fn to_fen_char(&self) -> char {
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

pub struct Board {
    pieces: [Option<Piece>; 64],
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let mut board_string = String::new();

        for (index, piece) in self.pieces.iter().enumerate() {
            if index % 8 == 0 {
                board_string.push_str("\n");
            }

            match piece {
                Some(piece) => board_string.push_str(&format!("{:?}", piece)),
                None => board_string.push_str("."),
            }
        }

        write!(f, "{}", board_string)
    }
}

impl Board {
    pub fn new() -> Board {
        Self::from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string()).unwrap()
    }

    pub fn from_fen(fen: &String) -> Result<Board, String> {
        let mut board = Board {
            pieces: [const { None }; 64],
        };

        let mut index = 0;

        for character in fen.chars() {
            match character {
                '/' => continue,
                '1'..='8' => {
                    let offset = character.to_digit(10).unwrap() as usize;
                    index += offset;
                    continue;
                }
                _ => {
                    let piece = Piece::from_fen_char(character)?;
                    board.pieces[index] = Some(piece)
                }
            }

            index += 1;
        }

        Ok(board)
    }
}
