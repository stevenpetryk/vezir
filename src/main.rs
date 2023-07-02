#![feature(inline_const)]

mod engine;

use crate::engine::Board;

fn main() {
    let board = Board::new();
    println!("{:?}", board);
}
