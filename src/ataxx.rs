mod zobrist;

use crate::{
    gamerules::{GameResult, GameRules},
    general::{bitboard::Bitboard, side::Side, square::Square},
};

pub const MAX_SQUARES: usize = 64;

/// splitmix64 finaliser
const fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}


#[derive(Clone, Copy)]
struct Irrecoverable<const WIDTH: usize, const HEIGHT: usize> {
    pub pieces: [Bitboard<WIDTH, HEIGHT>; 2],
    pub halfmoves: i32,
    pub hash: u64,
    pub hash_valid: bool,
}

#[derive(Clone)]
pub struct AtaxxPosition<const WIDTH: usize, const HEIGHT: usize> {
    stack: Vec<Irrecoverable<WIDTH, HEIGHT>>,
    pub pieces: [Bitboard<WIDTH, HEIGHT>; 2],
    pub blockers: Bitboard<WIDTH, HEIGHT>,
    pub turn: Side,
    pub halfmoves: i32,
    pub fullmoves: i32,
    hash: u64,
    hash_valid: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AtaxxMove<const WIDTH: usize, const HEIGHT: usize> {
    Pass,
    Single(Square<WIDTH, HEIGHT>),
    Double(Square<WIDTH, HEIGHT>, Square<WIDTH, HEIGHT>),
}

#[derive(Clone, Copy, PartialEq)]
pub enum Piece {
    Black,
    White,
}

pub type AtaxxPosition7x7 = AtaxxPosition<7, 7>;

impl<const WIDTH: usize, const HEIGHT: usize> Default for AtaxxPosition<WIDTH, HEIGHT> {
    fn default() -> Self {
        let () = Self::DIMENSIONS_ARE_SUPPORTED;

        let mut pos = Self {
            stack: vec![],
            pieces: [Bitboard::empty(); 2],
            blockers: Bitboard::empty(),
            turn: Side::Player1,
            halfmoves: 0,
            fullmoves: 0,
            hash: 0,
            hash_valid: true,
        };
        pos.hash = pos.calculate_hash();
        pos
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> AtaxxPosition<WIDTH, HEIGHT> {
    const DIMENSIONS_ARE_SUPPORTED: () =
        assert!(WIDTH * HEIGHT >= 2 && WIDTH * HEIGHT <= MAX_SQUARES);

    /// Hash key of a piece of `side` (0 and 1 for the players, 2 for blockers) on square
    /// index `idx`.
    ///
    /// 7x7 uses libataxx's keys so that hashes match there. Other sizes have no keys to
    /// match, so they're generated.
    const fn piece_key(side: usize, idx: usize) -> u64 {
        if WIDTH == 7 && HEIGHT == 7 {
            zobrist::PIECE_KEYS[side][idx]
        } else {
            mix((side as u64) << 8 | idx as u64)
        }
    }

    /// XORed into the hash when it's Player1's turn, as libataxx does for Black.
    const fn turn_key() -> u64 {
        if WIDTH == 7 && HEIGHT == 7 {
            zobrist::TURN_KEY
        } else {
            mix(u64::MAX)
        }
    }

    /// The hash of the position, which covers the pieces, blockers and side to move.
    ///
    /// It doesn't cover the halfmove or fullmove counters. It's kept up to date by
    /// `makemove`, `undomove`, `makenull` and `undonull`, and set once a FEN has been
    /// parsed. Setting `pieces`, `blockers` or `turn` directly leaves it stale; call
    /// `reset_hash` afterwards.
    ///
    /// Not valid between `makemove_with::<false>` and the matching `undomove`.
    #[must_use]
    pub fn get_hash(&self) -> u64 {
        debug_assert!(self.hash_valid, "Hash is stale after makemove_with::<false>");
        self.hash
    }

    /// Calculate the hash from scratch.
    #[must_use]
    pub fn calculate_hash(&self) -> u64 {
        let mut hash = if self.turn == Side::Player1 {
            Self::turn_key()
        } else {
            0
        };

        for sq in self.pieces[0] {
            hash ^= Self::piece_key(0, sq.get_index());
        }
        for sq in self.pieces[1] {
            hash ^= Self::piece_key(1, sq.get_index());
        }
        for sq in self.blockers {
            hash ^= Self::piece_key(2, sq.get_index());
        }

        hash
    }

    /// Recalculate the stored hash from scratch.
    pub fn reset_hash(&mut self) {
        self.hash = self.calculate_hash();
        self.hash_valid = true;
    }

    /// The hash the position would have after `mv`, without making the move.
    #[must_use]
    pub fn predict_hash(&self, mv: &AtaxxMove<WIDTH, HEIGHT>) -> u64 {
        self.get_hash() ^ self.hash_change(mv)
    }

    /// The XOR that turns the hash into the hash after `mv`.
    fn hash_change(&self, mv: &AtaxxMove<WIDTH, HEIGHT>) -> u64 {
        let us = self.turn as usize;
        let them = !self.turn as usize;

        let mut change = Self::turn_key();
        let (to, from) = match mv {
            AtaxxMove::Pass => return change,
            AtaxxMove::Single(to) => (to, None),
            AtaxxMove::Double(from, to) => (to, Some(from)),
        };

        change ^= Self::piece_key(us, to.get_index());
        if let Some(from) = from {
            change ^= Self::piece_key(us, from.get_index());
        }
        for sq in Bitboard::from_square(*to).adjacent() & self.get_them() {
            change ^= Self::piece_key(them, sq.get_index()) ^ Self::piece_key(us, sq.get_index());
        }

        change
    }

    /// Make a move, updating the hash only if `HASH` is true.
    ///
    /// With `HASH` false, `get_hash` must not be called until the move is undone.
    pub fn makemove_with<const HASH: bool>(&mut self, mv: &AtaxxMove<WIDTH, HEIGHT>) {
        self.stack.push(Irrecoverable {
            pieces: self.pieces,
            halfmoves: self.halfmoves,
            hash: self.hash,
            hash_valid: self.hash_valid,
        });
        if HASH {
            self.hash ^= self.hash_change(mv);
        } else {
            self.hash_valid = false;
        }
        match mv {
            AtaxxMove::Pass => self.halfmoves += 1,
            AtaxxMove::Single(to) => {
                let bb = Bitboard::from_square(*to);
                let captured = bb.adjacent() & self.get_them();
                self.pieces[self.turn] ^= bb | captured;
                self.pieces[!self.turn] ^= captured;
                self.halfmoves = 0;
            }
            AtaxxMove::Double(fr, to) => {
                let bb = Bitboard::from_square(*to);
                let captured = bb.adjacent() & self.get_them();
                self.pieces[self.turn] ^= bb | Bitboard::from_square(*fr) | captured;
                self.pieces[!self.turn] ^= captured;
                self.halfmoves += 1;
            }
        }
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    #[must_use]
    pub fn get_us(&self) -> Bitboard<WIDTH, HEIGHT> {
        self.pieces[self.turn]
    }

    #[must_use]
    pub fn get_them(&self) -> Bitboard<WIDTH, HEIGHT> {
        self.pieces[!self.turn]
    }

    #[must_use]
    pub fn get_black(&self) -> Bitboard<WIDTH, HEIGHT> {
        self.pieces[Side::Player1]
    }

    #[must_use]
    pub fn get_white(&self) -> Bitboard<WIDTH, HEIGHT> {
        self.pieces[Side::Player2]
    }

    #[must_use]
    pub fn get_both(&self) -> Bitboard<WIDTH, HEIGHT> {
        self.get_black() | self.get_white()
    }

    #[must_use]
    pub fn get_blockers(&self) -> Bitboard<WIDTH, HEIGHT> {
        self.blockers
    }

    #[must_use]
    pub fn get_empty(&self) -> Bitboard<WIDTH, HEIGHT> {
        !(self.get_black() | self.get_white() | self.get_blockers())
    }

    #[must_use]
    pub fn get_piece_on(&self, sq: Square<WIDTH, HEIGHT>) -> Option<Piece> {
        if self.get_black().is_square_set(sq) {
            Some(Piece::Black)
        } else if self.get_white().is_square_set(sq) {
            Some(Piece::White)
        } else {
            None
        }
    }

    #[must_use]
    pub fn piece_change(&self, mv: &AtaxxMove<WIDTH, HEIGHT>) -> i32 {
        match mv {
            AtaxxMove::Pass => 0,
            AtaxxMove::Single(to) => {
                (Bitboard::from_square(*to).adjacent() & self.get_them()).count() + 1
            }
            AtaxxMove::Double(_, to) => {
                (Bitboard::from_square(*to).adjacent() & self.get_them()).count()
            }
        }
    }

    #[must_use]
    pub fn get_piece_on_optional(&self, sq: Option<Square<WIDTH, HEIGHT>>) -> Option<Piece> {
        sq.and_then(|sq| self.get_piece_on(sq))
    }

    #[must_use]
    pub fn get_square_char(&self, sq: Square<WIDTH, HEIGHT>) -> Option<char> {
        match self.get_piece_on(sq) {
            Some(Piece::Black) => Some('x'),
            Some(Piece::White) => Some('o'),
            None if self.get_blockers().is_square_set(sq) => Some('-'),
            None => None,
        }
    }

    #[must_use]
    fn is_finished(&self) -> bool {
        self.halfmoves >= 100 || self.get_black().is_empty() || self.get_white().is_empty()
    }

    #[must_use]
    fn reachable_by(&self, side: Bitboard<WIDTH, HEIGHT>) -> Bitboard<WIDTH, HEIGHT> {
        side.reach2() & self.get_empty()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> GameRules for AtaxxPosition<WIDTH, HEIGHT> {
    type MoveType = AtaxxMove<WIDTH, HEIGHT>;
    const WIDTH: i32 = WIDTH as i32;
    const HEIGHT: i32 = HEIGHT as i32;

    fn startpos() -> Self {
        let () = Self::DIMENSIONS_ARE_SUPPORTED;

        let (right, top) = (WIDTH as i32 - 1, HEIGHT as i32 - 1);
        let bottom_left = Bitboard::from_coords(0, 0);
        let bottom_right = Bitboard::from_coords(right, 0);
        let top_left = Bitboard::from_coords(0, top);
        let top_right = Bitboard::from_coords(right, top);

        let is_a_line = WIDTH == 1 || HEIGHT == 1;
        let mut pos = Self {
            stack: vec![],
            pieces: if is_a_line {
                [top_right, bottom_left]
            } else {
                [top_left | bottom_right, bottom_left | top_right]
            },
            blockers: Bitboard::empty(),
            turn: Side::Player1,
            halfmoves: 0,
            fullmoves: 1,
            hash: 0,
            hash_valid: true,
        };
        pos.hash = pos.calculate_hash();
        pos
    }

    fn count_moves(&self) -> u64 {
        if self.is_finished() {
            return 0;
        }

        let us = self.get_us();
        let empty = self.get_empty();
        let mut nodes = 0;

        // Single moves
        nodes += (us.adjacent() & empty).count();

        // Double moves
        for from in us {
            nodes += (Bitboard::from_square(from).dist2() & empty).count();
        }

        // Pass
        if nodes == 0 && self.reachable_by(self.get_them()).is_occupied() {
            nodes = 1;
        }

        nodes as u64
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _: usize) -> bool {
        let bb = Bitboard::from_coords(x, y);
        match c {
            'x' => self.pieces[0] ^= bb,
            'o' => self.pieces[1] ^= bb,
            '-' => self.blockers ^= bb,
            _ => {}
        }
        true
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        if self.is_finished() {
            return;
        }

        let us = self.get_us();
        let empty = self.get_empty();
        let mut num_moves = 0;

        // Singles
        for sq in us.adjacent() & empty {
            if func(Self::MoveType::Single(sq)) {
                return;
            }
            num_moves += 1;
        }

        // Doubles
        for fr in us {
            for to in Bitboard::from_square(fr).dist2() & empty {
                if func(Self::MoveType::Double(fr, to)) {
                    return;
                }
                num_moves += 1;
            }
        }

        // Pass
        if num_moves == 0 && self.reachable_by(self.get_them()).is_occupied() {
            func(Self::MoveType::Pass);
        }
    }

    fn is_gameover(&self) -> bool {
        self.is_finished() || self.reachable_by(self.get_both()).is_empty()
    }

    fn get_result(&self) -> Option<GameResult> {
        let num_player1 = self.get_black().count();
        let num_player2 = self.get_white().count();

        if num_player1 == 0 && num_player2 == 0 {
            Some(GameResult::Draw)
        } else if num_player2 == 0 {
            Some(GameResult::Win(Side::Player1))
        } else if num_player1 == 0 {
            Some(GameResult::Win(Side::Player2))
        } else if self.halfmoves >= 100 {
            Some(GameResult::Draw)
        } else if self.reachable_by(self.get_both()).is_occupied() {
            None
        } else if num_player1 > num_player2 {
            Some(GameResult::Win(Side::Player1))
        } else if num_player2 > num_player1 {
            Some(GameResult::Win(Side::Player2))
        } else {
            Some(GameResult::Draw)
        }
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        self.makemove_with::<true>(mv);
    }

    fn undomove(&mut self, _mv: &Self::MoveType) {
        let data = self
            .stack
            .pop()
            .expect("Can't undo a move that was never made");
        self.pieces = data.pieces;
        self.halfmoves = data.halfmoves;
        self.hash = data.hash;
        self.hash_valid = data.hash_valid;
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
    }

    fn makenull(&mut self) {
        self.hash ^= Self::turn_key();
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undonull(&mut self) {
        self.hash ^= Self::turn_key();
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        let sq = Square::from_coords(x, y);
        self.get_square_char(sq).map(|c| c.to_string())
    }

    fn get_fen(&self) -> String {
        format!(
            "{} {} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "x",
                Side::Player2 => "o",
            },
            self.halfmoves,
            self.fullmoves,
        )
    }

    fn fen_parsed(&mut self) {
        self.reset_hash();
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            // Side to move
            1 => match part {
                "x" => self.turn = Side::Player1,
                "o" => self.turn = Side::Player2,
                _ => panic!("Unknown side to move token: {}", part),
            },
            // Halfmoves
            2 => self.halfmoves = part.parse::<i32>().unwrap(),
            // Fullmoves
            3 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index: {}", idx),
        }
    }

    fn parse_move(&self, movestr: &str) -> Result<Self::MoveType, &'static str> {
        if movestr == "0000" {
            return Ok(AtaxxMove::Pass);
        }

        let bytes = movestr.as_bytes();
        let split = (1..bytes.len())
            .find(|&i| bytes[i].is_ascii_alphabetic() && bytes[i - 1].is_ascii_digit());

        match split {
            Some(i) => Ok(AtaxxMove::Double(
                Square::from_string(&movestr[..i])?,
                Square::from_string(&movestr[i..])?,
            )),
            None => Ok(AtaxxMove::Single(Square::from_string(movestr)?)),
        }
    }

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        if (self.get_black() & self.get_white()).is_occupied() {
            Err("black & white overlap")
        } else if (self.get_black() & self.get_blockers()).is_occupied() {
            Err("black & blockers overlap")
        } else if (self.get_white() & self.get_blockers()).is_occupied() {
            Err("white & blockers overlap")
        } else {
            Ok(())
        }
    }

    fn perft(&mut self, depth: i32) -> u64 {
        match depth {
            0 => 1,
            1 => self.count_moves(),
            _ => {
                if self.is_finished() {
                    return 0;
                }

                let us = self.get_us();
                let empty = self.get_empty();
                let saved_pieces = self.pieces;
                let saved_halfmoves = self.halfmoves;
                let mut nodes = 0;
                let mut num_moves = 0;

                for sq in us.adjacent() & empty {
                    let bb = Bitboard::from_square(sq);
                    let captured = bb.adjacent() & self.get_them();

                    self.pieces[self.turn] ^= bb | captured;
                    self.pieces[!self.turn] ^= captured;
                    self.halfmoves = 0;
                    self.turn = !self.turn;
                    self.fullmoves += (self.turn == Side::Player1) as i32;

                    nodes += self.perft(depth - 1);

                    self.turn = !self.turn;
                    self.fullmoves -= (self.turn == Side::Player2) as i32;
                    self.pieces = saved_pieces;
                    self.halfmoves = saved_halfmoves;

                    num_moves += 1;
                }

                for fr in us {
                    for to in Bitboard::from_square(fr).dist2() & empty {
                        let bb = Bitboard::from_square(to);
                        let captured = bb.adjacent() & self.get_them();

                        self.pieces[self.turn] ^= bb | Bitboard::from_square(fr) | captured;
                        self.pieces[!self.turn] ^= captured;
                        self.halfmoves += 1;
                        self.turn = !self.turn;
                        self.fullmoves += (self.turn == Side::Player1) as i32;

                        nodes += self.perft(depth - 1);

                        self.turn = !self.turn;
                        self.fullmoves -= (self.turn == Side::Player2) as i32;
                        self.pieces = saved_pieces;
                        self.halfmoves = saved_halfmoves;

                        num_moves += 1;
                    }
                }

                if num_moves == 0 && self.reachable_by(self.get_them()).is_occupied() {
                    self.halfmoves += 1;
                    self.turn = !self.turn;
                    self.fullmoves += (self.turn == Side::Player1) as i32;

                    nodes += self.perft(depth - 1);

                    self.turn = !self.turn;
                    self.fullmoves -= (self.turn == Side::Player2) as i32;
                    self.halfmoves = saved_halfmoves;
                }

                nodes
            }
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::fmt::Display for AtaxxPosition<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let keep = !self.get_blockers();
        let height = keep.last_occupied_rank();
        let width = keep.last_occupied_file();

        for y in (0..=height).rev() {
            for x in 0..=width {
                let sq = Square::from_coords(x as i32, y as i32);
                match self.get_square_char(sq) {
                    Some('-') => write!(f, "#")?,
                    Some(c) => write!(f, "{c}")?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: x")?,
            Side::Player2 => writeln!(f, "Turn: o")?,
        }

        Ok(())
    }
}
