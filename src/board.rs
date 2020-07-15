use crate::{
    an,
    piece::{Color, Name, Piece},
    utils,
};

use std::collections::HashMap;
use std::{char, fmt};

pub const BOARD_SIZE: usize = 8;
type BoardType = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

const DEFAULT_BOARD: BoardType = [
    [
        Some(Piece::new(Name::Rook, Color::Black)),
        Some(Piece::new(Name::Knight, Color::Black)),
        Some(Piece::new(Name::Bishop, Color::Black)),
        Some(Piece::new(Name::Queen, Color::Black)),
        Some(Piece::new(Name::King, Color::Black)),
        Some(Piece::new(Name::Bishop, Color::Black)),
        Some(Piece::new(Name::Knight, Color::Black)),
        Some(Piece::new(Name::Rook, Color::Black)),
    ],
    [Some(Piece::new(Name::Pawn, Color::Black)); BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [Some(Piece::new(Name::Pawn, Color::White)); BOARD_SIZE],
    [
        Some(Piece::new(Name::Rook, Color::White)),
        Some(Piece::new(Name::Knight, Color::White)),
        Some(Piece::new(Name::Bishop, Color::White)),
        Some(Piece::new(Name::Queen, Color::White)),
        Some(Piece::new(Name::King, Color::White)),
        Some(Piece::new(Name::Bishop, Color::White)),
        Some(Piece::new(Name::Knight, Color::White)),
        Some(Piece::new(Name::Rook, Color::White)),
    ],
];

#[derive(Debug)]
pub enum MoveErrors {
    NotOnBoard,
    InvalidMove,
    InvalidCapture,
    NoPiece,
    Collision,
    NoSelfCapture,
    Parse(an::Errors),
}

impl From<an::Errors> for MoveErrors {
    fn from(error: an::Errors) -> Self {
        MoveErrors::Parse(error)
    }
}

fn pieces_to_string(pieces: &[Piece]) -> String {
    pieces
        .iter()
        .map(|p| format!("{}", p))
        .collect::<Vec<String>>()
        .join(",")
}

#[derive(Debug, PartialEq)]
pub struct Board {
    inner: BoardType,
    captures: HashMap<Color, Vec<Piece>>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            inner: DEFAULT_BOARD,
            captures: [Color::White, Color::Black]
                .iter()
                .map(|&color| (color, Vec::new()))
                .collect(),
        }
    }

    pub fn move_piece_an(&mut self, source: &str, destination: &str) -> Result<(), MoveErrors> {
        self.move_piece(an::an_to_coord(source)?, an::an_to_coord(destination)?)
    }

    pub fn move_piece(
        &mut self,
        source: (i32, i32),
        destination: (i32, i32),
    ) -> Result<(), MoveErrors> {
        let (x, y) = source;
        let (ux, uy) = (x as usize, y as usize);
        let (dest_x, dest_y) = destination;
        let (dest_ux, dest_uy) = (dest_x as usize, dest_y as usize);
        let mut piece = match self.inner[uy][ux] {
            Some(piece) => piece,
            None => return Err(MoveErrors::NoPiece),
        };
        if dest_x < 0 || dest_x >= BOARD_SIZE as i32 || dest_y < 0 || dest_y >= BOARD_SIZE as i32 {
            return Err(MoveErrors::NotOnBoard);
        }
        if piece.name() != Name::Knight && self.collides_on_move(source, destination) {
            return Err(MoveErrors::Collision);
        }
        if let Some(dest_piece) = self.inner[dest_uy][dest_ux] {
            if dest_piece.color() == piece.color() {
                return Err(MoveErrors::NoSelfCapture);
            }
            if piece.is_capture_valid(source, destination) {
                self.captures
                    .get_mut(&!piece.color())
                    .unwrap()
                    .push(dest_piece);
            } else {
                return Err(MoveErrors::InvalidCapture);
            }
        } else {
            if !piece.is_move_valid(source, destination) {
                return Err(MoveErrors::InvalidMove);
            }
        }
        piece.has_moved = true;
        self.inner[uy][ux] = None;
        self.inner[dest_y as usize][dest_x as usize] = Some(piece);
        Ok(())
    }

    pub fn get_piece_color(&self, source: &str) -> Result<Color, MoveErrors> {
        let (x, y) = an::an_to_coord(source).map_err(|err| MoveErrors::Parse(err))?;
        Ok(self.inner[y as usize][x as usize]
            .ok_or_else(|| MoveErrors::NoPiece)?
            .color())
    }

    fn collides_on_move(&self, source: (i32, i32), destination: (i32, i32)) -> bool {
        let (dx, dy) = (
            utils::clamp(destination.0 - source.0, -1, 1),
            utils::clamp(destination.1 - source.1, -1, 1),
        );
        let (dest_x, dest_y) = destination;
        let (mut cx, mut cy) = source;
        cx += dx;
        cy += dy;
        while cx != dest_x || cy != dest_y {
            if self.inner[cy as usize][cx as usize].is_some() {
                return true;
            }
            cx += dx;
            cy += dy;
        }
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  ┏{}━━━━┓\n{}",
            "━━━━┳".repeat(BOARD_SIZE - 1),
            {
                let mut out = String::new();
                for (y, row) in self.inner.iter().enumerate() {
                    out += &format!("{} ┃", 8 - y);
                    for piece in row {
                        if let Some(c) = piece {
                            out += &format!(" {} ┃", c);
                        } else {
                            out += "    ┃";
                        }
                    }
                    out += "\n  ";
                    if y < self.inner.len() - 1 {
                        out += &format!("┣{}━━━━┫", "━━━━╋".repeat(BOARD_SIZE - 1));
                    } else {
                        out += &format!("┗{}━━━━┛", "━━━━┻".repeat(BOARD_SIZE - 1));
                    }
                    out += "\n";
                }
                for code in 0..(BOARD_SIZE as u32) {
                    out += "    ";
                    out.push(char::from_u32(97 + code).unwrap());
                }
                out += "\n";
                out += &format!(
                    "    White: {}  Black: {}",
                    pieces_to_string(&self.captures[&Color::White]),
                    pieces_to_string(&self.captures[&Color::Black])
                );
                out
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Board::new(),
            Board {
                inner: DEFAULT_BOARD,
                captures: [(Color::White, Vec::new()), (Color::Black, Vec::new())]
                    .iter()
                    .cloned()
                    .collect(),
            }
        );
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        assert!(board.move_piece((0, 1), (0, 3)).is_ok());
        assert!(board.move_piece((1, 0), (0, 2)).is_ok());
        assert!(board.move_piece_an("a2", "a4").is_ok());
        assert!(board.move_piece_an("c1", "d2").is_err());
        assert!(board.move_piece_an("e1", "f1").is_err());
    }
}
