#[macro_use]
extern crate clap;

mod an;
mod board;
mod piece;
mod utils;

use board::Board;
use std::io::{self, Write};

fn pause() {
    let mut buf = String::new();
    println!("Press any key to continue...");
    let _ = io::stdin()
        .read_line(&mut buf)
        .expect("unable to pause terminal!");
}

fn main() {
    let _matches = clap_app!((env!("CARGO_PKG_NAME")) =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
    )
    .get_matches();
    let mut chessboard = Board::new();
    loop {
        println!("{}", chessboard);
        print!("Enter move: ");
        io::stdout().flush().expect("failed to flush stdout!");
        let mut line = String::new();
        let _ = io::stdin()
            .read_line(&mut line)
            .expect("unable to read line!");
        if let [source, destination] =
            &line.split("=>").map(|mv| mv.trim()).collect::<Vec<&str>>()[..]
        {
            if let Err(err) = chessboard.move_piece_an(source, destination) {
                eprintln!("Invalid move: {:?}", err);
                pause();
                continue;
            }

        } else {
            eprintln!("Invalid move string");
            pause();
        };
    }
}
