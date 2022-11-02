use core::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    PositionOutOfBounds,
    InvalidPositionFormat,
    InvalidMove,
}

const TILES_SIZE: usize = 8;
const VALID_COLUMNS: [&str; 8] = ["A", "B", "C", "D", "E", "F", "G", "H"];

#[derive(Debug)]
pub struct ChessGame {
    // tiles[row][column]
    // tiles[0][0] is left-down
    tiles: [[Option<Piece>; TILES_SIZE]; TILES_SIZE],
    next_turn: Turn,
}

impl ChessGame {
    pub fn new_game() -> Self {
        ChessGame {
            next_turn: Turn::WhitePlays,
            tiles: [
                [
                    // row 1
                    Some(Piece::White(PieceType::Rook)),
                    Some(Piece::White(PieceType::Knight)),
                    Some(Piece::White(PieceType::Bishop)),
                    Some(Piece::White(PieceType::Queen)),
                    Some(Piece::White(PieceType::King)),
                    Some(Piece::White(PieceType::Bishop)),
                    Some(Piece::White(PieceType::Knight)),
                    Some(Piece::White(PieceType::Rook)),
                ],
                [
                    // row 2
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                    Some(Piece::White(PieceType::Pawn)),
                ],
                [
                    // row 3
                    None, None, None, None, None, None, None, None,
                ],
                [
                    // row 4
                    None, None, None, None, None, None, None, None,
                ],
                [
                    // row 5
                    None, None, None, None, None, None, None, None,
                ],
                [
                    // row 6
                    None, None, None, None, None, None, None, None,
                ],
                [
                    // row 7
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                    Some(Piece::Black(PieceType::Pawn)),
                ],
                [
                    // row 8
                    Some(Piece::Black(PieceType::Rook)),
                    Some(Piece::Black(PieceType::Knight)),
                    Some(Piece::Black(PieceType::Bishop)),
                    Some(Piece::Black(PieceType::Queen)),
                    Some(Piece::Black(PieceType::King)),
                    Some(Piece::Black(PieceType::Bishop)),
                    Some(Piece::Black(PieceType::Knight)),
                    Some(Piece::Black(PieceType::Rook)),
                ],
            ],
        }
    }
    pub fn get_field(&self, pos: Position) -> Option<Piece> {
        self.get_field_ref(&pos)
    }
    pub fn get_field_ref(&self, pos: &Position) -> Option<Piece> {
        if pos.get_x() >= TILES_SIZE || pos.get_y() >= TILES_SIZE {
            None
        } else {
            self.tiles[pos.get_y()][pos.get_x()]
        }
    }
    fn check_pieces_between(&self, src: &Position, dst: &Position) -> Result<(), Error> {
        let (dist_x, dist_y) = src.distance_from(dst);

        // check pieces in between
        let between_pieces: Vec<Option<Piece>> = (1..dist_x.abs())
            .map(|x| {
                self.get_field(
                    Position::new(
                        (src.get_x() as i8 + dist_x.signum() * x) as usize,
                        (src.get_y() as i8 + dist_y.signum() * x) as usize,
                    )
                    .unwrap(),
                )
            })
            .collect();

        if between_pieces
            .iter()
            .map(|x| *x != None)
            .fold(false, |acc, x| acc || x)
        {
            return Err(Error::InvalidMove);
        }

        Ok(())
    }
    fn update_turn(&mut self) {
        self.next_turn = match self.next_turn {
            Turn::BlackPlays => Turn::WhitePlays,
            Turn::WhitePlays => Turn::BlackPlays,
        }
    }
    pub fn current_player(&self) -> Turn {
        self.next_turn
    }

    pub fn make_move(&mut self, src: Position, dst: Position) -> Result<Option<Piece>, Error> {
        self.make_move_ref(&src, &dst)
    }

    fn make_move_ref(&mut self, src: &Position, dst: &Position) -> Result<Option<Piece>, Error> {
        let mut returned_piece: Option<Piece> = None;

        if src == dst {
            return Err(Error::InvalidMove);
        }

        let (dist_x, dist_y) = src.distance_from(dst);

        let moving_piece = match self.tiles[src.get_y()][src.get_x()] {
            Some(piece) => piece,
            None => return Err(Error::InvalidMove),
        };

        if Turn::WhitePlays == self.next_turn {
            if let Piece::Black(_) = moving_piece {
                return Err(Error::InvalidMove);
            }

            // cannot move to position of piece with the same color
            if let Some(piece) = self.get_field_ref(dst) {
                if let Piece::White(_) = piece {
                    return Err(Error::InvalidMove);
                }
            }

            // ! TODO: check the code below
            // check for validity
            match moving_piece {
                Piece::White(piece_type) => match piece_type {
                    PieceType::Bishop => {
                        if dist_x.abs() != dist_y.abs() {
                            return Err(Error::InvalidMove);
                        }

                        let _ = self.check_pieces_between(src, dst)?;
                    }
                    PieceType::King => {
                        if dist_x.abs() + dist_y.abs() != 1 {
                            return Err(Error::InvalidMove);
                        }
                    }
                    PieceType::Knight => {
                        if !((dist_x.abs() == 3 && dist_y.abs() == 1)
                            || (dist_x.abs() == 1 && dist_y.abs() == 3))
                        {
                            return Err(Error::InvalidMove);
                        }
                    }
                    PieceType::Pawn => {}
                    PieceType::Queen => {
                        if !(
                            ( dist_x == 0 && dist_y.abs() > 0 ) || ( dist_x.abs() > 0 && dist_y == 0 ) // moving like Rook
                            ||
                            dist_x == dist_y
                            // moving like Bishop
                        ) {
                            return Err(Error::InvalidMove);
                        }

                        let _ = self.check_pieces_between(src, dst)?;
                    }
                    PieceType::Rook => {
                        if !((dist_x == 0 && dist_y.abs() > 0) || (dist_x.abs() > 0 && dist_y == 0))
                        {
                            return Err(Error::InvalidMove);
                        }

                        let _ = self.check_pieces_between(src, dst)?;
                    }
                },
                _ => panic!("How did you get here, sneaky little hobbitses?"),
            }
        } else {
            // Turn::Black
            if let Piece::White(_) = moving_piece {
                return Err(Error::InvalidMove);
            }

            // cannot move to position of piece with the same color
            if let Some(piece) = self.get_field_ref(dst) {
                if let Piece::Black(_) = piece {
                    return Err(Error::InvalidMove);
                }
            }

            // check for validity
            match moving_piece {
                Piece::Black(piece_type) => match piece_type {
                    PieceType::Bishop => {
                        if dist_x.abs() != dist_y.abs() {
                            return Err(Error::InvalidMove);
                        }

                        let _ = self.check_pieces_between(src, dst)?;
                    }
                    PieceType::King => {
                        if dist_x.abs() + dist_y.abs() != 1 {
                            return Err(Error::InvalidMove);
                        }
                    }
                    PieceType::Knight => {
                        if !((dist_x.abs() == 3 && dist_y.abs() == 1)
                            || (dist_x.abs() == 1 && dist_y.abs() == 3))
                        {
                            return Err(Error::InvalidMove);
                        }
                    }
                    PieceType::Pawn => {}
                    PieceType::Queen => {
                        if !(
                            ( dist_x == 0 && dist_y.abs() > 0 ) || ( dist_x.abs() > 0 && dist_y == 0 ) // moving like Rook
                            ||
                            dist_x == dist_y
                            // moving like Bishop
                        ) {
                            return Err(Error::InvalidMove);
                        }

                        let _ = self.check_pieces_between(src, dst)?;
                    }
                    PieceType::Rook => {
                        if !((dist_x == 0 && dist_y.abs() > 0) || (dist_x.abs() > 0 && dist_y == 0))
                        {
                            return Err(Error::InvalidMove);
                        }

                        let _ = self.check_pieces_between(src, dst)?;
                    }
                },
                _ => panic!("How did you get here, sneaky little hobbitses?"),
            }
        }

        // everything is valid
        // make the move
        let dst_piece_opt = self.get_field_ref(dst);
        if let Some(dst_piece) = dst_piece_opt {
            returned_piece = Some(dst_piece);
        }

        self.tiles[dst.get_y()][dst.get_x()] = Some(moving_piece);
        self.tiles[src.get_y()][src.get_x()] = None;

        self.update_turn();

        Ok(returned_piece)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(s: &str) -> Result<Position, Error> {
        if s.len() != 2 {
            return Err(Error::InvalidPositionFormat);
        }

        let column = s.chars().nth(0).unwrap().to_string();
        let row: usize;
        if let Ok(num) = s.chars().nth(1).unwrap().to_string().parse::<usize>() {
            if num >= TILES_SIZE {
                return Err(Error::InvalidPositionFormat);
            } else {
                row = num - 1;
            }
        } else {
            return Err(Error::InvalidPositionFormat);
        }

        if !VALID_COLUMNS.contains(&column.as_str()) || row >= TILES_SIZE {
            return Err(Error::InvalidPositionFormat);
        }

        Ok(Position {
            x: VALID_COLUMNS.iter().position(|x| x == &column).unwrap(),
            y: row,
        })
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Result<Self, Error> {
        if x >= TILES_SIZE || y >= TILES_SIZE {
            Err(Error::PositionOutOfBounds)
        } else {
            Ok(Position { x, y })
        }
    }
    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
    pub fn distance_from(&self, other: &Position) -> (i8, i8) {
        (
            self.x as i8 - other.get_x() as i8, // x distance
            self.y as i8 - other.get_y() as i8, // y distance
        )
    }
}

fn max(u1: usize, u2: usize) -> usize {
    if u1 > u2 {
        u1
    } else {
        u2
    }
}

fn min(u1: usize, u2: usize) -> usize {
    if u1 < u2 {
        u1
    } else {
        u2
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Turn {
    WhitePlays,
    BlackPlays,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Rook,   // veza
    Knight, // kon
    Bishop, // strelec
    Queen,  // kralovna
    King,   // kral
    Pawn,   // pesiak
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}
