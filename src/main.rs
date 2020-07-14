mod an;
mod board;
mod piece;
mod utils;

use board::Board;

fn main() {
    let mut board = Board::new();
    println!("{}", board);
    board
        .move_piece((1, 0), (0, 2))
        .expect("unable to move piece");
    println!("{}", board);
    board
        .move_piece((0, 1), (0, 2))
        .expect("unable to move piece");
    println!("{}", board);
}
