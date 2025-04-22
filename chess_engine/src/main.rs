mod chess;
mod engine;

use crate::chess::Board;

fn main() {
    println!("Hello, world!");
    let board = Board::init();
    board.show_board();
}
