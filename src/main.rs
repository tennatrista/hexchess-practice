pub mod chess;
pub mod ai;

use chess::*;

fn main() {
	let game = GameState::new();
    print!("{}", game.board.to_fen_grid());
	let square_name = "f7";
	match game.board.piece_at_square_name(square_name) {
		None => println!("No piece at {}!", square_name),
		Some(p) => println!("Piece at {} is {}", square_name, String::from(p.to_char())),
	};
}
