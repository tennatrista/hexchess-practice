use crate::{GameState, Move};
use rand::prelude::*;

pub fn next_move(game: &GameState) -> Option<Move> {
	let mut rng = rand::thread_rng();
	let legal_moves = game.get_legal_moves();
	if legal_moves.is_empty() {
		None
	} else {
		Some(legal_moves[rng.gen_range(0..legal_moves.len())])
	}
}

#[cfg(test)]
mod ai_tests {
	use super::*;

	#[test]
	fn sanity_test() {
		for _ in 0..100 {
			println!("new game");
			let mut game = GameState::new();
			for _ in 0..100 {
				for potential_move in game.get_legal_moves() {
					print!(" {} ", potential_move.to_string());
				}
				println!("");
				let m = next_move(&game);
				print!("{:?}", game.side_to_move);
				match m {
					Some(m) => {
						game.make_move(m);
						println!("{:?}", m.to_string());
					},
					None => break
				}
				println!("{}", game.board);
			}
		}
	}
}