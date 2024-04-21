use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Side {
	White,
	Black
}

impl Side {
	pub fn to_string(&self) -> String {
		match self {
			Side::White => String::from("w"),
			Side::Black => String::from("b"),
		}
	}

	pub fn other(&self) -> Side {
		match self {
			Side::White => Side::Black,
			Side::Black => Side::White,
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

impl PieceType {
	fn to_char(&self) -> char {
		match self {
			PieceType::Pawn => 'p',
			PieceType::Knight => 'n',
			PieceType::Bishop => 'b',
			PieceType::Rook => 'r',
			PieceType::Queen => 'q',
			PieceType::King => 'k',
		}
	}
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
	side: Side,
	piece_type: PieceType,
}

impl Piece {
	pub fn to_char(&self) -> char {
		match self.side {
			Side::White => self.piece_type.to_char(),
			Side::Black => self.piece_type.to_char().to_ascii_uppercase(),
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Move {
	pub from: (i8, i8),
	pub to: (i8, i8),
	pub promo: Option<PieceType>,
}

impl Move {
	pub fn new(from: (i8, i8), to: (i8, i8)) -> Move {
		Move {
			from,
			to,
			promo: None,
		}
	}

	pub fn new_with_promo(from: (i8, i8), to: (i8, i8), promo: PieceType) -> Move {
		Move {
			from,
			to,
			promo: Some(promo)
		}
	}

	pub fn from_str(string: &str) -> Move {
		let mut parts = string.split('-');
		Move {
			from: Board::coordinates_from_name(parts.next().expect("Should have been a valid move.")),
			to: Board::coordinates_from_name(parts.next().expect("Should have been a valid move.")),
			promo: None,
		}
	}

	pub fn to_string(&self) -> String {
		match self.promo {
			Some(promo_type) => format!("{}-{}={}", Board::name_from_coordinates(self.from), Board::name_from_coordinates(self.to), promo_type.to_char()),
			None => format!("{}-{}", Board::name_from_coordinates(self.from), Board::name_from_coordinates(self.to))
		}
	}
}

#[derive(Clone)]
pub struct Board {
	squares: [[Option<Piece>; 8]; 8],
	sides: HashMap<Side, HashSet<(i8, i8)>>,
	white_king_location: (i8, i8),
	black_king_location: (i8, i8),
}

impl Board {
	pub fn new() -> Board {
		let mut board = Board::new_blank();
		board.place_piece_on_square(Piece { piece_type: PieceType::Rook, side: Side::White}, "a1");
		board.place_piece_on_square(Piece { piece_type: PieceType::Knight, side: Side::White}, "b1");
		board.place_piece_on_square(Piece { piece_type: PieceType::Bishop, side: Side::White}, "c1");
		board.place_piece_on_square(Piece { piece_type: PieceType::Queen, side: Side::White}, "d1");
		board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::White}, "e1");
		board.place_piece_on_square(Piece { piece_type: PieceType::Bishop, side: Side::White}, "f1");
		board.place_piece_on_square(Piece { piece_type: PieceType::Knight, side: Side::White}, "g1");
		board.place_piece_on_square(Piece { piece_type: PieceType::Rook, side: Side::White}, "h1");
		for file in ["a", "b", "c", "d", "e", "f", "g", "h"] {
			board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::White}, format!("{}2", file).as_str());
			board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::Black}, format!("{}7", file).as_str());
		}
		board.place_piece_on_square(Piece { piece_type: PieceType::Rook, side: Side::Black}, "a8");
		board.place_piece_on_square(Piece { piece_type: PieceType::Knight, side: Side::Black}, "b8");
		board.place_piece_on_square(Piece { piece_type: PieceType::Bishop, side: Side::Black}, "c8");
		board.place_piece_on_square(Piece { piece_type: PieceType::Queen, side: Side::Black}, "d8");
		board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::Black}, "e8");
		board.place_piece_on_square(Piece { piece_type: PieceType::Bishop, side: Side::Black}, "f8");
		board.place_piece_on_square(Piece { piece_type: PieceType::Knight, side: Side::Black}, "g8");
		board.place_piece_on_square(Piece { piece_type: PieceType::Rook, side: Side::Black}, "h8");
		board
	}

	pub fn new_blank() -> Board {
		let mut sides = HashMap::new();
		sides.insert(Side::White, HashSet::new());
		sides.insert(Side::Black, HashSet::new());
		Board {
			squares: [[None; 8]; 8],
			sides: sides,
			white_king_location: (-1, -1),
			black_king_location: (-1, -1),
		}
	}
	
	pub fn extract_file_from_name(square_name: &str) -> i8 {
		(square_name.as_bytes()[0] as i8) - 97
	}

	pub fn extract_rank_from_name(square_name: &str) -> i8 {
		(square_name.as_bytes()[1] as i8) - 48 - 1
	}

	pub fn coordinates_from_name(square_name: &str) -> (i8, i8) {
		(Board::extract_rank_from_name(square_name), Board::extract_file_from_name(square_name))
	}

	pub fn name_from_coordinates(coordinates: (i8, i8)) -> String {
		format!("{}{}", ((coordinates.1 as u8) + 97) as char, coordinates.0 + 1)
	}

	pub fn within_bounds(coordinates: (i8, i8)) -> bool {
		coordinates.0 >= 0 && coordinates.0 < 8 && coordinates.1 >= 0 && coordinates.1 < 8
	}

	pub fn piece_at(&self, coordinates: (i8, i8)) -> Option<Piece> {
		self.squares[coordinates.0 as usize][coordinates.1 as usize]
	}

	pub fn piece_at_square_name(&self, square_name: &str) -> Option<Piece> {
		let coordinates = Board::coordinates_from_name(square_name);
		self.piece_at(coordinates)
	}

	pub fn king_location(&self, side: Side) -> (i8, i8) {
		match side {
			Side::White => self.white_king_location,
			Side::Black => self.black_king_location,
		}
	}

	pub fn place_piece(&mut self, piece: Piece, coordinates: (i8, i8)) {
		self.sides.get_mut(&piece.side).unwrap().insert(coordinates);
		self.squares[coordinates.0 as usize][coordinates.1 as usize] = Some(piece);
		if let PieceType::King = piece.piece_type {
			match piece.side {
				Side::White => {
					self.white_king_location = coordinates;
				},
				Side::Black => {
					self.black_king_location = coordinates;
				}
			}
		}
	}

	pub fn place_piece_on_square(&mut self, piece: Piece, square_name: &str) {
		self.place_piece(piece, Board::coordinates_from_name(square_name));
	}

	pub fn remove_piece(&mut self, coordinates: (i8, i8)) {
		for (_side, roster) in &mut self.sides {
			roster.remove(&coordinates);
		}
		self.squares[coordinates.0 as usize][coordinates.1 as usize] = None;
	}

	pub fn remove_piece_from_square(&mut self, square_name: &str) {
		self.remove_piece(Board::coordinates_from_name(square_name));
	}

	pub fn move_piece(&mut self, m: Move) {
		let piece = self.piece_at(m.from).unwrap();
		self.remove_piece(m.to);
		match m.promo {
			Some(promo_type) => self.place_piece(Piece {
				side: piece.side,
				piece_type: promo_type,
			}, m.to),
			None => self.place_piece(piece, m.to)
		}
		self.remove_piece(m.from);

		let roster = self.sides.get_mut(&piece.side).unwrap();
		roster.remove(&m.from);
		roster.insert(m.to);
	}

	pub fn to_fen(&self) -> String {
		let mut fen_string = String::from("");
		for rank in self.squares.iter() {
			for piece in rank {
				fen_string.push(match piece {
					Some(p) => p.to_char(),
					None => ' ',
				});
			}
			fen_string.push('/');
		}
		fen_string
	}

	pub fn to_fen_grid(&self) -> String {
		let mut fen_string = String::from("");
		for rank in self.squares.iter().rev() {
			for piece in rank {
				fen_string.push(match piece {
					Some(p) => p.to_char(),
					None => ' ',
				});
			}
			fen_string.push('\n');
		}
		fen_string
	}
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_fen_grid())
	}
}

#[derive(Clone)]
pub struct CastlingAvailability {
	white_can_castle_kingside: bool,
	white_can_castle_queenside: bool,
	black_can_castle_kingside: bool,
	black_can_castle_queenside: bool,
}

impl CastlingAvailability {
	pub fn all() -> CastlingAvailability {
		CastlingAvailability {
			white_can_castle_kingside: true,
			white_can_castle_queenside: true,
			black_can_castle_kingside: true,
			black_can_castle_queenside: true,
		}
	}

	pub fn none() -> CastlingAvailability {
		CastlingAvailability {
			white_can_castle_kingside: false,
			white_can_castle_queenside: false,
			black_can_castle_kingside: false,
			black_can_castle_queenside: false,
		}
	}

	pub fn to_string(&self) -> String {
		let mut string = String::from("");
		if self.white_can_castle_kingside { string.push('K') }
		if self.white_can_castle_queenside { string.push('Q') }
		if self.black_can_castle_kingside { string.push('k') }
		if self.black_can_castle_queenside { string.push('q') }
		if string.is_empty() { string.push('-') }
		string
	}
}

#[derive(Clone)]
pub struct GameState {
	pub board: Board,
	pub side_to_move: Side,
	pub castling_availability: CastlingAvailability,
	pub en_passant_square: Option<(i8, i8)>,
}

impl GameState {
	pub fn new() -> GameState {
		GameState {
			board: Board::new(),
			side_to_move: Side::White,
			castling_availability: CastlingAvailability::all(),
			en_passant_square: None,
		}
	}

	pub fn to_fen(&self) -> String {
		format!("{} {} {} {}", 
			self.board.to_fen(), 
			self.side_to_move.to_string(),
			self.castling_availability.to_string(),
			match &self.en_passant_square {
				None => String::from("-"),
				Some(sq) => Board::name_from_coordinates(*sq),
			}
		)
	}

	pub fn make_move(&mut self, m: Move) {
		if let PieceType::Pawn = self.board.piece_at(m.from).unwrap().piece_type {
			if let Some(sq) = self.en_passant_square {
				if sq == m.to {
					let pawn_to_remove = (m.from.0, sq.1);
					self.board.remove_piece(pawn_to_remove);
				}
			}
		}
		self.en_passant_square = None;
		if let PieceType::Pawn = self.board.piece_at(m.from).unwrap().piece_type {
			if m.from.0 - m.to.0 == 2 {
				self.en_passant_square = Some((m.from.0 - 1, m.from.1));
			}
			if m.from.0 - m.to.0 == -2 {
				self.en_passant_square = Some((m.from.0 + 1, m.from.1));
			}
		}

		if let PieceType::King = self.board.piece_at(m.from).unwrap().piece_type {
			if m.to.1 - m.from.1 == 2 { // Kingside castling
				let move_the_rook_to_f1_or_f8 = Move::new((m.from.0, 7), (m.from.0, 5));
				self.board.move_piece(move_the_rook_to_f1_or_f8);
			}
			if m.to.1 - m.from.1 == -2 { // Queenside castling
				let move_the_rook_to_d1_or_d8 = Move::new((m.from.0, 0), (m.from.0, 3));
				self.board.move_piece(move_the_rook_to_d1_or_d8);
			}
		}
		if m.from == Board::coordinates_from_name("a1") || m.from == Board::coordinates_from_name("e1") {
			self.castling_availability.white_can_castle_queenside = false;
		}
		if m.from == Board::coordinates_from_name("h1") || m.from == Board::coordinates_from_name("e1") {
			self.castling_availability.white_can_castle_kingside = false;
		}
		if m.from == Board::coordinates_from_name("a8") || m.from == Board::coordinates_from_name("e8") {
			self.castling_availability.black_can_castle_queenside = false;
		}
		if m.from == Board::coordinates_from_name("h8") || m.from == Board::coordinates_from_name("e8") {
			self.castling_availability.black_can_castle_kingside = false;
		}

		self.board.move_piece(m);

		self.side_to_move = Side::other(&self.side_to_move);
	}

	pub fn make_move_on_copy(&self, m: Move) -> GameState {
		let mut copy = self.clone();
		copy.make_move(m);
		copy
	}

	pub fn is_in_check(&self, side: Side) -> bool {
		for square in self.board.sides.get(&Side::other(&side)).unwrap() {
			for m in self.get_possible_moves_from(*square) {
				if m.to == self.board.king_location(side) {
					return true;
				}
			}
		}
		false
	}

	pub fn is_in_checkmate(&self, side: Side) -> bool {
		self.side_to_move == side && self.is_in_check(side) && self.get_legal_moves().is_empty()
	}

	pub fn is_in_stalemate(&self) -> bool {
		!self.is_in_check(self.side_to_move) && self.get_legal_moves().is_empty()
	}

	pub fn move_would_put_self_in_check(&self, m: Move) -> bool {
		let side_making_move = self.board.piece_at(m.from).unwrap().side;
		let hypothetical_board = self.make_move_on_copy(m);
		let check = hypothetical_board.is_in_check(side_making_move);
		check
	}

	pub fn get_legal_moves(&self) -> Vec<Move> {
		let mut moves = Vec::new();
		for candidate in self.get_possible_moves() {
			if !self.move_would_put_self_in_check(candidate) {
				moves.push(candidate);
			}
		}
		for candidate in self.get_castling_moves() {
			moves.push(candidate);
		}
		moves
	}

	pub fn get_possible_moves(&self) -> Vec<Move> {
		let mut moves = Vec::new();
		let origin_squares = self.board.sides.get(&self.side_to_move).unwrap();
		for origin in origin_squares {
			moves.append(&mut self.get_possible_moves_from(*origin));
		}
		moves
	}

	pub fn get_possible_moves_from(&self, current: (i8, i8)) -> Vec<Move> {
		let mut moves = Vec::new();
		let file = current.1;
		let rank = current.0;

		match self.board.piece_at(current) {
			None => (),
			Some(piece) => match piece.piece_type {
				PieceType::Pawn => {
					let direction = match piece.side {
						Side::White => 1,
						Side::Black => -1,
					};
					let on_initial_rank = match piece.side {
						Side::White => rank == 1,
						Side::Black => rank == 6,
					};
					let one_ahead = (rank + direction, file);
					let two_ahead = (rank + (direction * 2), file);
					let on_seventh = !Board::within_bounds(two_ahead);
					if let None = self.board.piece_at(one_ahead) {
						if on_seventh {
							for promo_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
								moves.push(Move::new_with_promo(current, one_ahead, promo_type));
							}
						} else {
							moves.push(Move::new(current, one_ahead));
							if Board::within_bounds(two_ahead) {
								if let None = self.board.piece_at(two_ahead) {
									if on_initial_rank {
										moves.push(Move::new(current, two_ahead));
									}
								}	
							}
						}
					}
					let forward_left = (rank + direction, file - 1);
					if Board::within_bounds(forward_left) {
						match self.board.piece_at(forward_left) {
							None => {
								if let Some(en_passant_square) = self.en_passant_square {
									if en_passant_square == forward_left {
										moves.push(Move::new(current, forward_left));
									}
								}
							},
							Some(other_piece) => {
								if piece.side != other_piece.side {
									if on_seventh {
										for promo_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
											moves.push(Move::new_with_promo(current, forward_left, promo_type));
										}
									} else {
										moves.push(Move::new(current, forward_left));
									}
								}
							},
						}
					}
					let forward_right = (rank + direction, file + 1);
					if Board::within_bounds(forward_right) {
						match self.board.piece_at(forward_right) {
							None => {
								if let Some(en_passant_square) = self.en_passant_square {
									if en_passant_square == forward_right {
										moves.push(Move::new(current, forward_right));
									}
								}
							},
							Some(other_piece) => {
								if piece.side != other_piece.side {
									if on_seventh {
										for promo_type in [PieceType::Knight, PieceType::Bishop, PieceType::Rook, PieceType::Queen] {
											moves.push(Move::new_with_promo(current, forward_right, promo_type));
										}
									} else {
										moves.push(Move::new(current, forward_right));
									}
								}
							}
						}
					}	
				},
				PieceType::Knight => {
					let directions = [(2, 1), (2, -1), (1, 2), (1, -2), (-1, 2), (-1, -2), (-2, 1), (-2, -1)];
					for direction in directions {
						moves.append(&mut self.get_possible_moves_in_direction(current, direction, 1));
					}
				},
				PieceType::Bishop => {
					let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
					for direction in directions {
						moves.append(&mut self.get_possible_moves_in_direction(current, direction, 7));
					}
				},
				PieceType::Rook => {
					let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
					for direction in directions {
						moves.append(&mut self.get_possible_moves_in_direction(current, direction, 7));
					}
				},
				PieceType::Queen => {
					let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)];
					for direction in directions {
						moves.append(&mut self.get_possible_moves_in_direction(current, direction, 7));
					}
				},
				PieceType::King => {
					let directions = [(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)];
					for direction in directions {
						moves.append(&mut self.get_possible_moves_in_direction(current, direction, 1));
					}
					
				},
			}
		}
		moves
	}

	fn get_possible_moves_in_direction(&self, origin: (i8, i8), direction: (i8, i8), max_distance: i8) -> Vec<Move> {
		let mut moves = Vec::new();
		match self.board.piece_at(origin) {
			None => (),
			Some(piece) => {
				for distance in 1..=max_distance {
					let destination = (origin.0 + (direction.0 * distance), origin.1 + (direction.1 * distance));
					if !Board::within_bounds(destination) { break; }
					match self.board.piece_at(destination) {
						None => {
							moves.push(Move::new(origin, destination));
						},
						Some(other_piece) => {
							if piece.side != other_piece.side {
								moves.push(Move::new(origin, destination));
							}
							break;
						}
					}
				}
			}
		}
		moves
	}

	fn get_castling_moves(&self) -> Vec<Move> {
		let mut moves = Vec::new();
		let king_initial_square = match self.side_to_move {
			Side::White => Board::coordinates_from_name("e1"),
			Side::Black => Board::coordinates_from_name("e8"),
		};
		if self.is_in_check(self.side_to_move) {
			return moves;
		}
		if let Some(piece) = self.board.piece_at(king_initial_square) {
			if let PieceType::King = piece.piece_type {
				match piece.side {
					Side::White => {
						if self.castling_availability.white_can_castle_kingside {
							let kf1 = Move::from_str("e1-f1");
							let kg1 = Move::from_str("e1-g1");
							if (self.board.piece_at_square_name("f1") == None && !self.move_would_put_self_in_check(kf1)) &&
								(self.board.piece_at_square_name("g1") == None && !self.move_would_put_self_in_check(kg1)) {
								moves.push(kg1);
							}
						}
						if self.castling_availability.white_can_castle_queenside {
							let kd1 = Move::from_str("e1-d1");
							let kc1 = Move::from_str("e1-c1");
							if (self.board.piece_at_square_name("d1") == None && !self.move_would_put_self_in_check(kd1)) &&
								(self.board.piece_at_square_name("c1") == None && !self.move_would_put_self_in_check(kc1)) &&
								self.board.piece_at_square_name("b1") == None {
								moves.push(kc1);
							}
						}
					},
					Side::Black => {
						if self.castling_availability.black_can_castle_kingside {
							let kf8 = Move::from_str("e8-f8");
							let kg8 = Move::from_str("e8-g8");
							if (self.board.piece_at_square_name("f8") == None && !self.move_would_put_self_in_check(kf8)) &&
								(self.board.piece_at_square_name("g8") == None && !self.move_would_put_self_in_check(kg8)) {
								moves.push(kg8);
							}
						}
						if self.castling_availability.black_can_castle_queenside {
							let kd8 = Move::from_str("e8-d8");
							let kc8 = Move::from_str("e8-c8");
							if (self.board.piece_at_square_name("d8") == None && !self.move_would_put_self_in_check(kd8)) &&
								(self.board.piece_at_square_name("c8") == None && !self.move_would_put_self_in_check(kc8)) && 
								self.board.piece_at_square_name("b8") == None {
								moves.push(kc8);
							}
						}
					},
				}
			}
		}
		moves
	}

	pub fn move_is_legal(&self, candidate: Move) -> bool {
		match self.board.piece_at(candidate.from) {
			None => false,
			Some(_p) => self.get_possible_moves_from(candidate.from).contains(&candidate),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_coordinates() {
		assert_eq!("c2", Board::name_from_coordinates((1, 2)));
		assert_eq!("e6", Board::name_from_coordinates(Board::coordinates_from_name("e6")));
		assert_eq!("c2-c4", Move::from_str("c2-c4").to_string());
	}

	#[test]
	fn test_board_setup() {
		let mut game = GameState::new();
		assert_eq!(game.side_to_move, Side::White);
		assert_eq!(game.board.piece_at_square_name("c1").unwrap().piece_type, PieceType::Bishop);
		assert_eq!(game.board.piece_at_square_name("c1").unwrap().side, Side::White);
		assert_eq!(game.board.piece_at_square_name("f7").unwrap().piece_type, PieceType::Pawn);
		assert_eq!(game.board.piece_at_square_name("f7").unwrap().side, Side::Black);
		game.board.remove_piece_from_square("f7");
		assert_eq!(game.board.piece_at_square_name("f7"), None);
	}

	#[test]
	fn test_making_moves() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		assert_eq!(game.side_to_move, Side::Black);
		assert_eq!(game.board.piece_at_square_name("e2"), None);
		assert_eq!(game.board.piece_at_square_name("e4").unwrap().piece_type, PieceType::Pawn);
		assert_eq!(game.board.piece_at_square_name("e4").unwrap().side, Side::White);
		game.make_move(Move::from_str("c7-c5"));
		assert_eq!(game.side_to_move, Side::White);
		assert_eq!(game.board.piece_at_square_name("c7"), None);
		assert_eq!(game.board.piece_at_square_name("c5").unwrap().piece_type, PieceType::Pawn);
		assert_eq!(game.board.piece_at_square_name("c5").unwrap().side, Side::Black);
	}

	#[test]
	fn test_legal_moves() {
		let mut game = GameState::new();
		assert!(game.move_is_legal(Move::from_str("g1-f3")));
		assert!(game.move_is_legal(Move::from_str("a2-a3")));
		assert!(game.move_is_legal(Move::from_str("e2-e4")));
		assert!(game.move_is_legal(Move::from_str("h2-h4")));
		game.make_move(Move::from_str("d2-d4"));
		game.make_move(Move::from_str("g8-f6"));
		assert!(game.move_is_legal(Move::from_str("c1-f4")));
		assert!(!game.move_is_legal(Move::from_str("f1-c4")));
		assert!(game.move_is_legal(Move::from_str("d1-d3")));
		assert!(!game.move_is_legal(Move::from_str("d1-d5")));
		assert!(!game.move_is_legal(Move::from_str("d1-h5")));
		game.make_move(Move::from_str("a2-a4"));
		game.make_move(Move::from_str("g7-g6"));
		assert!(game.move_is_legal(Move::from_str("a1-a3")));
		assert!(!game.move_is_legal(Move::from_str("a1-a4")));
		assert!(game.move_is_legal(Move::from_str("e1-d2")));
		assert!(!game.move_is_legal(Move::from_str("e1-d1")));
	}

	
	#[test]
	fn test_pawns() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("g1-f3"));
		game.make_move(Move::from_str("g8-f6"));
		assert!(game.move_is_legal(Move::from_str("e2-e4")));
		assert!(!game.move_is_legal(Move::from_str("f2-f4")));
	}

	#[test]
	fn test_legal_move_generator() {
		let mut game = GameState::new();
		assert!(game.get_possible_moves().contains(&Move::from_str("a2-a3")));
		assert!(game.get_possible_moves().contains(&Move::from_str("a2-a4")));
		assert!(game.get_possible_moves().contains(&Move::from_str("g1-h3")));
		assert!(game.get_possible_moves().contains(&Move::from_str("g1-f3")));
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("e7-e5"));
		assert!(game.get_possible_moves().contains(&Move::from_str("f1-a6")));
		assert!(game.get_possible_moves().contains(&Move::from_str("d1-g4")));
		assert!(!game.get_possible_moves().contains(&Move::from_str("f1-h3")));
		assert!(!game.get_possible_moves().contains(&Move::from_str("f1-g2")));
		assert!(!game.get_possible_moves().contains(&Move::from_str("d1-d2")));
	}

	#[test]
	fn test_illegal_moves() {
		let game = GameState::new();
		assert!(!game.move_is_legal(Move::from_str("g1-g3")));
		assert!(!game.move_is_legal(Move::from_str("g1-e2")));
		assert!(!game.move_is_legal(Move::from_str("a2-b3")));
		assert!(!game.move_is_legal(Move::from_str("e2-e2")));
		assert!(!game.move_is_legal(Move::from_str("e2-e5")));
	}

	#[test]
	fn test_check() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("e7-e5"));
		game.make_move(Move::from_str("f2-f4"));
		assert!(!game.is_in_check(Side::White));
		game.make_move(Move::from_str("d8-h4"));
		assert!(game.is_in_check(Side::White));
		game.make_move(Move::from_str("g2-g3"));
		assert!(!game.is_in_check(Side::White));
	}

	#[test]
	fn test_checkmate() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("e7-e5"));
		game.make_move(Move::from_str("d1-h5"));
		game.make_move(Move::from_str("b8-c6"));
		game.make_move(Move::from_str("f1-c4"));
		game.make_move(Move::from_str("g8-f6"));
		game.make_move(Move::from_str("h5-f7"));
		assert!(game.is_in_checkmate(Side::Black));
	}

	#[test]
	fn test_checkmate2() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("f2-f3"));
		game.make_move(Move::from_str("e7-e5"));
		game.make_move(Move::from_str("g2-g4"));
		game.make_move(Move::from_str("d8-h4"));
		assert!(game.is_in_checkmate(Side::White));
	}

	#[test]
	fn test_checkmate3() {
		let mut game = GameState {
			board: Board::new_blank(),
			side_to_move: Side::Black,
			castling_availability: CastlingAvailability::none(),
			en_passant_square: None,			
		};
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Rook, side: Side::White}, "a8");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::White}, "g1");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::Black}, "g8");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::Black}, "f7");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::Black}, "g7");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::Black}, "h7");
		assert!(game.is_in_checkmate(Side::Black));
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Bishop, side: Side::Black}, "e4");
		assert!(!game.is_in_checkmate(Side::Black));
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::Black}, "d5");
		assert!(game.is_in_checkmate(Side::Black));
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Knight, side: Side::Black}, "b6");
		assert!(!game.is_in_checkmate(Side::Black));
	}

	#[test]
	fn test_stalemate() {
		let mut game = GameState {
			board: Board::new_blank(),
			side_to_move: Side::Black,
			castling_availability: CastlingAvailability::none(),
			en_passant_square: None,			
		};
		game.board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::White}, "h1");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Queen, side: Side::White}, "c7");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::Black}, "a8");
		assert!(game.is_in_stalemate());
	}

	#[test]
	fn test_en_passant() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("a7-a6"));
		game.make_move(Move::from_str("e4-e5"));
		game.make_move(Move::from_str("d7-d5"));
		assert!(game.get_legal_moves().contains(&Move::from_str("e5-d6")));
		game.make_move(Move::from_str("e5-d6"));
		assert_eq!(game.board.piece_at_square_name("d5"), None);
	}

	#[test]
	fn test_en_passant2() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("a7-a6"));
		game.make_move(Move::from_str("e4-e5"));
		game.make_move(Move::from_str("f7-f5"));
		assert!(game.get_legal_moves().contains(&Move::from_str("e5-f6")));
		game.make_move(Move::from_str("e5-f6"));
		assert_eq!(game.board.piece_at_square_name("f5"), None);
	}

	#[test]
	fn test_castling() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("e7-e5"));
		game.make_move(Move::from_str("g1-f3"));
		game.make_move(Move::from_str("b8-c6"));
		assert!(!game.get_legal_moves().contains(&Move::from_str("e1-g1")));
		game.make_move(Move::from_str("f1-c4"));
		game.make_move(Move::from_str("f8-c5"));
		assert!(game.get_legal_moves().contains(&Move::from_str("e1-g1")));
		game.make_move(Move::from_str("e1-g1"));
		assert_eq!(game.board.piece_at_square_name("g1").unwrap().piece_type, PieceType::King);
		assert_eq!(game.board.piece_at_square_name("f1").unwrap().piece_type, PieceType::Rook);
		assert_eq!(game.board.piece_at_square_name("h1"), None);
	}

	#[test]
	fn test_castling2() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("d2-d4"));
		game.make_move(Move::from_str("g8-f6"));
		game.make_move(Move::from_str("b1-c3"));
		game.make_move(Move::from_str("g7-g6"));
		game.make_move(Move::from_str("c1-f4"));
		game.make_move(Move::from_str("f8-g7"));
		game.make_move(Move::from_str("d1-d2"));
		assert!(game.get_legal_moves().contains(&Move::from_str("e8-g8")));
		game.make_move(Move::from_str("e8-g8"));
		assert_eq!(game.board.piece_at_square_name("g8").unwrap().piece_type, PieceType::King);
		assert_eq!(game.board.piece_at_square_name("f8").unwrap().piece_type, PieceType::Rook);
		assert_eq!(game.board.piece_at_square_name("h8"), None);
		assert!(game.get_legal_moves().contains(&Move::from_str("e1-c1")));
		game.make_move(Move::from_str("e1-c1"));
		assert_eq!(game.board.piece_at_square_name("c1").unwrap().piece_type, PieceType::King);
		assert_eq!(game.board.piece_at_square_name("d1").unwrap().piece_type, PieceType::Rook);
		assert_eq!(game.board.piece_at_square_name("a1"), None);
	}

	#[test]
	fn test_castling3() {
		let mut game = GameState::new();
		game.make_move(Move::from_str("e2-e4"));
		game.make_move(Move::from_str("e7-e5"));
		game.make_move(Move::from_str("g1-f3"));
		game.make_move(Move::from_str("g8-f6"));
		game.make_move(Move::from_str("f1-c4"));
		game.make_move(Move::from_str("h7-h6"));
		game.make_move(Move::from_str("d2-d3"));
		game.make_move(Move::from_str("f8-b4"));
		assert!(game.is_in_check(Side::White));
		assert!(!game.get_legal_moves().contains(&Move::from_str("e1-g1")));
		game.make_move(Move::from_str("c2-c3"));
		game.make_move(Move::from_str("b4-a5"));
		assert!(game.get_legal_moves().contains(&Move::from_str("e1-g1")));
	}

	#[test]
	fn test_promotion() {
		let mut game = GameState {
			board: Board::new_blank(),
			side_to_move: Side::White,
			castling_availability: CastlingAvailability::none(),
			en_passant_square: None,			
		};
		game.board.place_piece_on_square(Piece { piece_type: PieceType::Pawn, side: Side::White}, "a7");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::White}, "b7");
		game.board.place_piece_on_square(Piece { piece_type: PieceType::King, side: Side::Black}, "d7");
		println!("{:?}", game.get_legal_moves());
		let a8q = Move{from: Board::coordinates_from_name("a7"), to: Board::coordinates_from_name("a8"), promo: Some(PieceType::Queen)};
		assert!(game.get_legal_moves().contains(&a8q));
		assert!(!game.get_legal_moves().contains(&Move::from_str("a7-a8")));
		game.make_move(a8q);
		assert_eq!(game.board.piece_at_square_name("a8").unwrap().piece_type, PieceType::Queen);
	}
}