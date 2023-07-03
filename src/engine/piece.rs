extern crate num;
use super::player::Player;
use core::fmt::Debug;
use num_derive::FromPrimitive;

#[derive(Debug, FromPrimitive)]
#[repr(u8)]
pub enum PieceType {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive)]
#[repr(u8)]
pub enum Piece {
    WhiteKing = (Player::White as u8) | (PieceType::King as u8),
    WhiteQueen = (Player::White as u8) | (PieceType::Queen as u8),
    WhiteRook = (Player::White as u8) | (PieceType::Rook as u8),
    WhiteBishop = (Player::White as u8) | (PieceType::Bishop as u8),
    WhiteKnight = (Player::White as u8) | (PieceType::Knight as u8),
    WhitePawn = (Player::White as u8) | (PieceType::Pawn as u8),
    BlackKing = (Player::Black as u8) | (PieceType::King as u8),
    BlackQueen = (Player::Black as u8) | (PieceType::Queen as u8),
    BlackRook = (Player::Black as u8) | (PieceType::Rook as u8),
    BlackBishop = (Player::Black as u8) | (PieceType::Bishop as u8),
    BlackKnight = (Player::Black as u8) | (PieceType::Knight as u8),
    BlackPawn = (Player::Black as u8) | (PieceType::Pawn as u8),
}

impl Piece {
    pub fn build(player: Player, piece_type: PieceType) -> Piece {
        let primitive = (player as u8) | (piece_type as u8);

        let element = num::FromPrimitive::from_u8(primitive);

        match element {
            Some(element) => element,
            None => panic!("Invalid piece"),
        }
    }

    pub fn piece_type(&self) -> PieceType {
        let primitive = (*self as u8) & 0b0000_0111;

        let element = num::FromPrimitive::from_u8(primitive);

        match element {
            Some(element) => element,
            None => panic!("Invalid piece type"),
        }
    }

    pub fn player(&self) -> Player {
        let primitive = (*self as u8) & 0b000011000;

        let element = num::FromPrimitive::from_u8(primitive);

        match element {
            Some(element) => element,
            None => panic!("Invalid player"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::assert_matches::assert_matches;

    #[test]
    fn test_piece_instantiation() {
        let player = Player::White;
        let piece_type = PieceType::Knight;

        let piece = Piece::build(player, piece_type);
        assert_matches!(piece, Piece::WhiteKnight);
    }
}
