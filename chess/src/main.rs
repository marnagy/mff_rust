mod solution;

use solution::ChessGame;
use solution::Error;
use solution::Piece::*;
use solution::PieceType::*;
use solution::Turn;

use crate::solution::Position;

fn main() -> Result<(), Error> {
    println!("Program works!");

    let mut game = ChessGame::new_game();

    let column = "A";
    let row = 1;

    let pos_fmt = format!("{}{}", column, row);

    let position: Position = pos_fmt
        .as_str()
        .try_into()?;
    
    let text = match game.get_field(position) {
        Some(White(Rook)) => "white rook",
        Some(White(Knight)) => "white knight",
        Some(White(Bishop)) => "white bishop",
        Some(White(Queen)) => "white queen",
        Some(White(King)) => "white king",
        Some(White(Pawn)) => "white pawn",
        Some(Black(Rook)) => "black rook",
        Some(Black(Knight)) => "black knight",
        Some(Black(Bishop)) => "black bishop",
        Some(Black(Queen)) => "black queen",
        Some(Black(King)) => "black king",
        Some(Black(Pawn)) => "black pawn",
        None => "empty field",
    };

    println!("Value on position {}: {}", pos_fmt, text);

    let text = match game.current_player() {
        Turn::WhitePlays => "white plays",
        Turn::BlackPlays => "black plays",
    };

    println!("Next turn: {}", text);

    let src: Position = format!("{}{}", "A", 2)
        .as_str()
        .try_into()?;
    
    let dst: Position = format!("{}{}", "A", 3)
        .as_str()
        .try_into()?;
    
    let text = match game.make_move(src, dst) {
        Ok(None) => "valid move",
        Ok(Some(White(Pawn))) => "valid move & white pawn taken",
        Err(Error::InvalidMove) => "invalid move",
        _ => "other",
    };

    println!("Mave resolution: {}", text);

    Ok(())
}
