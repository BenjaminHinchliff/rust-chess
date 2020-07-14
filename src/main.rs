mod board;
mod piece;

use board::Board;

fn main() {
    let board = Board::new();
    println!("{}", board);
}
