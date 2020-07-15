#[macro_use]
extern crate clap;

mod an;
mod board;
mod piece;
mod utils;

use board::Board;
use piece::{Color, Name, Piece, TextType};
use std::io::{self, Write};

fn pause() {
    let mut buf = String::new();
    println!("Press enter to continue...");
    let _ = io::stdin()
        .read_line(&mut buf)
        .expect("unable to pause terminal!");
}

fn main() {
    let matches = clap_app!((env!("CARGO_PKG_NAME")) =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@arg acsii: -a --acsii "sets mode to use acsii character set")
    )
    .get_matches();
    let text_type = if matches.is_present("acsii") { TextType::ACSII } else { TextType::UTF8 };
    let mut chessboard = Board::new(text_type);
    let mut white_turn = true;
    loop {
        println!(
            "      White: {}  Black: {}  |{}'s Turn|",
            Piece::new(Name::King, Color::White, text_type),
            Piece::new(Name::King, Color::Black, text_type),
            if white_turn { "White" } else { "Black" }
        );
        println!("{}", chessboard);
        print!("Enter move: ");
        io::stdout().flush().expect("failed to flush stdout!");
        let mut line = String::new();
        let _ = io::stdin()
            .read_line(&mut line)
            .expect("unable to read line!");
        if let [source, destination] =
            &line.split("to").map(|mv| mv.trim()).collect::<Vec<&str>>()[..]
        {
            let piece_color = match chessboard.get_piece_color(source) {
                Ok(color) => color,
                Err(err) => {
                    eprintln!("Invalid piece: {:?}", err);
                    pause();
                    continue;
                }
            };
            if white_turn && piece_color == Color::White
                || !white_turn && piece_color == Color::Black
            {
                if let Err(err) = chessboard.move_piece_an(source, destination) {
                    eprintln!("Invalid move: {:?}", err);
                    pause();
                    continue;
                }
            } else {
                eprintln!("That isn't your piece!");
                pause();
                continue;
            }
            white_turn = !white_turn;
        } else {
            eprintln!("Invalid move string");
            pause();
        };
    }
}
