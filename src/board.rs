use crate::piece::{Color, Names, Piece};
use crate::utils::clamp;
use std::fmt;

pub const BOARD_SIZE: usize = 8;
type BoardType = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

const DEFAULT_BOARD: BoardType = [
    [
        Some(Piece::new(Names::Rook, Color::Black)),
        Some(Piece::new(Names::Knight, Color::Black)),
        Some(Piece::new(Names::Bishop, Color::Black)),
        Some(Piece::new(Names::Queen, Color::Black)),
        Some(Piece::new(Names::King, Color::Black)),
        Some(Piece::new(Names::Bishop, Color::Black)),
        Some(Piece::new(Names::Knight, Color::Black)),
        Some(Piece::new(Names::Rook, Color::Black)),
    ],
    [Some(Piece::new(Names::Pawn, Color::Black)); BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [None; BOARD_SIZE],
    [Some(Piece::new(Names::Pawn, Color::White)); BOARD_SIZE],
    [
        Some(Piece::new(Names::Rook, Color::White)),
        Some(Piece::new(Names::Knight, Color::White)),
        Some(Piece::new(Names::Bishop, Color::White)),
        Some(Piece::new(Names::Queen, Color::White)),
        Some(Piece::new(Names::King, Color::White)),
        Some(Piece::new(Names::Bishop, Color::White)),
        Some(Piece::new(Names::Knight, Color::White)),
        Some(Piece::new(Names::Rook, Color::White)),
    ],
];

#[derive(Debug)]
pub enum MoveErrors {
    NotOnBoard,
    InvalidMove,
    NoPiece,
    Collision,
}

#[derive(Debug, PartialEq)]
pub struct Board {
    inner: BoardType,
}

impl Board {
    pub const fn new() -> Board {
        Board {
            inner: DEFAULT_BOARD,
        }
    }

    pub fn move_piece(
        &mut self,
        source: (i32, i32),
        destination: (i32, i32),
    ) -> Result<(), MoveErrors> {
        let (x, y) = source;
        let (ux, uy) = (x as usize, y as usize);
        let (dest_x, dest_y) = destination;
        let piece = match self.inner[uy][ux] {
            Some(piece) => piece,
            None => return Err(MoveErrors::NoPiece),
        };
        if dest_x < 0 || dest_x >= BOARD_SIZE as i32 || dest_y < 0 || dest_y >= BOARD_SIZE as i32 {
            return Err(MoveErrors::NotOnBoard);
        }
        if !piece.is_move_valid(source, destination) {
            return Err(MoveErrors::InvalidMove);
        }
        if piece.name() != Names::Knight && self.collides_on_move(source, destination) {
            return Err(MoveErrors::Collision);
        }
        self.inner[uy][ux] = None;
        self.inner[dest_y as usize][dest_x as usize] = Some(piece);
        Ok(())
    }

    fn collides_on_move(&self, source: (i32, i32), destination: (i32, i32)) -> bool {
        let (dx, dy) = (
            clamp(destination.0 - source.0, -1, 1),
            clamp(destination.1 - source.1, -1, 1),
        );
        let (dest_x, dest_y) = destination;
        let (mut cx, mut cy) = source;
        while cx != dest_x || cy != dest_y {
            cx += dx;
            cy += dy;
            if self.inner[cy as usize][cx as usize].is_some() {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "┏{}━━━━┓\n{}",
            "━━━━┳".repeat(BOARD_SIZE - 1),
            {
                let mut out = String::new();
                for (y, row) in self.inner.iter().enumerate() {
                    out += "┃";
                    for piece in row {
                        if let Some(c) = piece {
                            out += &format!(" {}  ┃", c);
                        } else {
                            out += "    ┃";
                        }
                    }
                    out += "\n";
                    if y < self.inner.len() - 1 {
                        out += &format!("┣{}━━━━┫\n", "━━━━╋".repeat(BOARD_SIZE - 1));
                    } else {
                        out += &format!("┗{}━━━━┛", "━━━━┻".repeat(BOARD_SIZE - 1));
                    }
                }
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
                inner: DEFAULT_BOARD
            }
        );
    }
}
