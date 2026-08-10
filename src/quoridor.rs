use crate::{
    gamerules::{GameResult, GameRules},
    general::{
        bitboard::Bitboard,
        rank::Rank,
        side::Side,
        square::{Direction, Square},
    },
};

#[derive(Copy, Clone, Default, PartialEq)]
pub struct Yee {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

pub enum Wall {
    Vertical,
    Horizontal,
}

type QSquare = Square<9, 9>;

#[derive(Clone, Default)]
pub struct QuoridorPosition {
    pub pieces: [Option<QSquare>; 2],
    pub grid: [Bitboard<8, 8>; 2],
    pub turn: Side,
    pub walls: [usize; 2],
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum QuoridorMove {
    Move(Direction),
    Jump(Direction, Direction),
    VWall(Square<8, 8>),
    HWall(Square<8, 8>),
}

fn opposite(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

fn sideways(dir: Direction) -> [Direction; 2] {
    match dir {
        Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
        Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
    }
}

fn shift(sq: Option<QSquare>, dir: Direction) -> Option<QSquare> {
    sq.and_then(|sq| sq.push(dir))
}

impl QuoridorPosition {
    fn add_wall(&mut self, sq: Square<8, 8>, wall: Wall) {
        self.grid[wall as usize] ^= Bitboard::from_square(sq);
    }

    #[must_use]
    fn wall_is_legal(&self, sq: Square<8, 8>, wall: Wall) -> bool {
        let mut next = self.clone();
        next.add_wall(sq, wall);
        next.has_path(Side::Player1) && next.has_path(Side::Player2)
    }

    #[must_use]
    fn has_path(&self, side: Side) -> bool {
        let start = self.pieces[side].expect("Have piece");
        let mut reachable = [[false; 9]; 9];
        let mut stack = vec![];

        // Insert starting location
        stack.push(start);

        while let Some(sq) = stack.pop() {
            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if !self.can_move(sq, dir) {
                    continue;
                }

                let Some(nsq) = sq.push(dir) else {
                    continue;
                };

                if reachable[nsq.get_file()][nsq.get_rank()] {
                    continue;
                }

                let done = match side {
                    Side::Player1 => nsq.get_rank() == Rank::<9>(8),
                    Side::Player2 => nsq.get_rank() == Rank::<9>(0),
                };

                if done {
                    return true;
                }

                reachable[nsq.get_file()][nsq.get_rank()] = true;
                stack.push(nsq);
            }
        }

        false
    }

    /// The fewest steps `side`'s pawn needs to reach its back rank, going
    /// around walls and ignoring the other pawn. None if walls block every
    /// route.
    #[must_use]
    pub fn path_distance(&self, side: Side) -> Option<u32> {
        let start = self.pieces[side].expect("Have piece");
        let goal = match side {
            Side::Player1 => Rank::<9>(8),
            Side::Player2 => Rank::<9>(0),
        };

        let mut seen = [[false; 9]; 9];
        let mut queue = std::collections::VecDeque::new();
        seen[start.get_file()][start.get_rank()] = true;
        queue.push_back((start, 0));

        while let Some((sq, dist)) = queue.pop_front() {
            if sq.get_rank() == goal {
                return Some(dist);
            }

            for dir in [
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ] {
                if !self.can_move(sq, dir) {
                    continue;
                }
                let Some(next) = sq.push(dir) else {
                    continue;
                };
                if !seen[next.get_file()][next.get_rank()] {
                    seen[next.get_file()][next.get_rank()] = true;
                    queue.push_back((next, dist + 1));
                }
            }
        }

        None
    }

    #[must_use]
    fn can_move(&self, sq: QSquare, dir: Direction) -> bool {
        let x = sq.get_file().0 as i32;
        let y = sq.get_rank().0 as i32;

        let hor =
            self.grid[Wall::Horizontal as usize] | self.grid[Wall::Horizontal as usize].east();
        let ver = self.grid[Wall::Vertical as usize] | self.grid[Wall::Vertical as usize].north();

        let hx = x.min(7);
        let vy = y.min(7);

        let blocked = match dir {
            Direction::Up if y < Self::HEIGHT - 1 => {
                if x > 7 {
                    self.grid[Wall::Horizontal as usize].is_coord_set(hx, y)
                } else {
                    hor.is_coord_set(x, y)
                }
            }
            Direction::Down if y > 0 => {
                if x > 7 {
                    self.grid[Wall::Horizontal as usize].is_coord_set(hx, y - 1)
                } else {
                    hor.is_coord_set(x, y - 1)
                }
            }
            Direction::Left if x > 0 => {
                if y > 7 {
                    self.grid[Wall::Vertical as usize].is_coord_set(x - 1, vy)
                } else {
                    ver.is_coord_set(x - 1, y)
                }
            }
            Direction::Right if x < Self::WIDTH - 1 => {
                if y > 7 {
                    self.grid[Wall::Vertical as usize].is_coord_set(x, vy)
                } else {
                    ver.is_coord_set(x, y)
                }
            }
            _ => true,
        };

        !blocked
    }
}

impl GameRules for QuoridorPosition {
    type MoveType = QuoridorMove;
    const WIDTH: i32 = 9;
    const HEIGHT: i32 = 9;

    fn startpos() -> Self {
        Self {
            pieces: [
                Some(QSquare::from_coords(4, 0)),
                Some(QSquare::from_coords(4, 8)),
            ],
            // grid: Mailbox::default(),
            grid: [Bitboard::empty(); 2],
            turn: Side::Player1,
            walls: [10, 10],
        }
    }

    fn set_piece(&mut self, _x: i32, _y: i32, _c: char, _: usize) -> bool {
        todo!()
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        let sq = self.pieces[self.turn].expect("Must have a piece");

        // Piece moves
        let opponent = self.pieces[!self.turn];
        for dir in [
            Direction::Up,
            Direction::Down,
            Direction::Right,
            Direction::Left,
        ] {
            if !self.can_move(sq, dir) {
                continue;
            }

            let next = sq
                .push(dir)
                .expect("A pawn that can move has a square ahead");
            if Some(next) != opponent {
                func(QuoridorMove::Move(dir));
            } else if self.can_move(next, dir) {
                func(QuoridorMove::Move(dir));
            } else {
                for side in sideways(dir) {
                    if self.can_move(next, side) {
                        func(QuoridorMove::Jump(dir, side));
                    }
                }
            }
        }

        // Walls
        let num_walls = self.walls[self.turn];
        if num_walls > 0 {
            let v = self.grid[Wall::Vertical as usize];
            let h = self.grid[Wall::Horizontal as usize];

            // Vertical
            for sq in !(v | v.north() | v.south() | h) {
                if self.wall_is_legal(sq, Wall::Vertical) {
                    func(QuoridorMove::VWall(sq));
                }
            }

            // Horizontal
            for sq in !(h | h.east() | h.west() | v) {
                if self.wall_is_legal(sq, Wall::Horizontal) {
                    func(QuoridorMove::HWall(sq));
                }
            }
        }
    }

    fn is_gameover(&self) -> bool {
        self.pieces[Side::Player1]
            .expect("Must have player 1 piece")
            .get_rank()
            == Rank(8)
            || self.pieces[Side::Player2]
                .expect("Must have player 2 piece")
                .get_rank()
                == Rank(0)
    }

    fn get_result(&self) -> Option<GameResult> {
        if self.pieces[Side::Player1]
            .expect("Must have player 1 piece")
            .get_rank()
            == Rank(8)
        {
            Some(GameResult::Win(Side::Player1))
        } else if self.pieces[Side::Player2]
            .expect("Must have player 2 piece")
            .get_rank()
            == Rank(0)
        {
            Some(GameResult::Win(Side::Player2))
        } else {
            None
        }
    }

    // fn count_moves(&self) -> u64 {
    //     todo!("count moves")
    // }

    fn makemove(&mut self, mv: &Self::MoveType) {
        match mv {
            QuoridorMove::Move(dir) => {
                let mut sq = shift(self.pieces[self.turn], *dir);
                if sq == self.pieces[!self.turn] {
                    sq = shift(sq, *dir);
                }
                self.pieces[self.turn] = sq;
            }
            QuoridorMove::Jump(dir, side) => {
                self.pieces[self.turn] = shift(shift(self.pieces[self.turn], *dir), *side);
            }
            QuoridorMove::VWall(sq) => {
                self.grid[Wall::Vertical as usize] ^= Bitboard::from_square(*sq);
                self.walls[self.turn] -= 1;
            }
            QuoridorMove::HWall(sq) => {
                self.grid[Wall::Horizontal as usize] ^= Bitboard::from_square(*sq);
                self.walls[self.turn] -= 1;
            }
        }

        self.turn = !self.turn;
    }

    fn undomove(&mut self, mv: &Self::MoveType) {
        self.turn = !self.turn;

        match mv {
            QuoridorMove::Move(dir) => {
                let mut sq = shift(self.pieces[self.turn], opposite(*dir));
                if sq == self.pieces[!self.turn] {
                    sq = shift(sq, opposite(*dir));
                }
                self.pieces[self.turn] = sq;
            }
            QuoridorMove::Jump(dir, side) => {
                self.pieces[self.turn] = shift(
                    shift(self.pieces[self.turn], opposite(*side)),
                    opposite(*dir),
                );
            }
            QuoridorMove::VWall(sq) => {
                self.grid[Wall::Vertical as usize] ^= Bitboard::from_square(*sq);
                self.walls[self.turn] += 1;
            }
            QuoridorMove::HWall(sq) => {
                self.grid[Wall::Horizontal as usize] ^= Bitboard::from_square(*sq);
                self.walls[self.turn] += 1;
            }
        }
    }

    fn makenull(&mut self) {
        todo!("makenull")
    }

    fn undonull(&mut self) {
        todo!("undonull")
    }

    fn get_square_string(&self, _x: i32, _y: i32) -> Option<String> {
        todo!("get square string")
    }

    fn get_fen(&self) -> String {
        let list = |wall: Wall| {
            let squares: String = self.grid[wall as usize]
                .into_iter()
                .map(|sq| sq.to_string())
                .collect();
            if squares.is_empty() {
                "-".to_owned()
            } else {
                squares
            }
        };

        format!(
            "{} {} {} {} {} {}",
            list(Wall::Horizontal),
            list(Wall::Vertical),
            self.pieces[Side::Player1].expect("Must have player 1 piece"),
            self.pieces[Side::Player2].expect("Must have player 2 piece"),
            self.walls[Side::Player1],
            self.walls[Side::Player2],
        )
    }

    fn set_fen(&mut self, fen: &str) {
        if fen == "startpos" {
            *self = Self::startpos();
            return;
        }

        *self = Self::default();
        let parts: Vec<&str> = fen.split(' ').collect();
        assert_eq!(parts.len(), 6, "Quoridor fen needs 6 parts");

        for (wall, list) in [
            (Wall::Horizontal as usize, parts[0]),
            (Wall::Vertical as usize, parts[1]),
        ] {
            let list = list.trim_start_matches('-');
            let mut start = 0;
            for (i, c) in list.char_indices() {
                if c.is_ascii_alphabetic() && i > start {
                    let sq = Square::<8, 8>::from_string(&list[start..i]).expect("Bad wall square");
                    self.grid[wall] |= Bitboard::from_square(sq);
                    start = i;
                }
            }
            if start < list.len() {
                let sq = Square::<8, 8>::from_string(&list[start..]).expect("Bad wall square");
                self.grid[wall] |= Bitboard::from_square(sq);
            }
        }

        self.pieces = [
            Some(QSquare::from_string(parts[2]).expect("Bad player 1 square")),
            Some(QSquare::from_string(parts[3]).expect("Bad player 2 square")),
        ];
        self.walls = [
            parts[4].parse().expect("Bad player 1 wall count"),
            parts[5].parse().expect("Bad player 2 wall count"),
        ];
        self.turn = Side::Player1;
    }

    fn parse_fen_part(&mut self, _idx: usize, _part: &str) {
        todo!("parse fen part")
    }

    fn parse_move(&self, _movestr: &str) -> Result<Self::MoveType, &'static str> {
        todo!("parse move")
    }

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        todo!("is valid")
    }
}

impl std::fmt::Display for QuoridorPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = [[' '; 17]; 17];

        // Empty grid
        for x in 0..9 {
            for y in 0..9 {
                out[2 * x][2 * y] = '·';
            }
        }

        // Add pieces
        if let Some(sq) = self.pieces[Side::Player1] {
            let x = sq.get_file().0 * 2;
            let y = sq.get_rank().0 * 2;
            out[x as usize][y as usize] = 'P';
        }
        if let Some(sq) = self.pieces[Side::Player2] {
            let x = sq.get_file().0 * 2;
            let y = sq.get_rank().0 * 2;
            out[x as usize][y as usize] = 'p';
        }

        // Vertical
        for sq in self.grid[Wall::Vertical as usize] {
            out[2 * sq.get_file().0 as usize + 1][2 * sq.get_rank().0 as usize + 0] = '│';
            out[2 * sq.get_file().0 as usize + 1][2 * sq.get_rank().0 as usize + 2] = '│';
        }

        // Horizontal
        for sq in self.grid[Wall::Horizontal as usize] {
            out[2 * sq.get_file().0 as usize + 0][2 * sq.get_rank().0 as usize + 1] = '─';
            out[2 * sq.get_file().0 as usize + 2][2 * sq.get_rank().0 as usize + 1] = '─';
        }

        // Add walls - Intersections
        for x in 0..8 {
            for y in 0..8 {
                let has_down = self.grid[Wall::Vertical as usize].is_coord_set(x, y);
                let has_up = self.grid[Wall::Vertical as usize].is_coord_set(x, y)
                    | self.grid[Wall::Vertical as usize].is_coord_set(x, y + 1);

                let has_left = self.grid[Wall::Horizontal as usize].is_coord_set(x, y);
                let has_right = self.grid[Wall::Horizontal as usize].is_coord_set(x, y)
                    | self.grid[Wall::Horizontal as usize].is_coord_set(x + 1, y);

                out[(2 * x + 1) as usize][(2 * y + 1) as usize] =
                    match (has_up, has_down, has_left, has_right) {
                        (true, true, true, true) => '┼',
                        (true, true, true, false) => '┤',
                        (true, true, false, true) => '├',
                        (true, true, false, false) => '│',
                        (true, false, true, true) => '┴',
                        (true, false, true, false) => '┘',
                        (true, false, false, true) => '└',
                        (true, false, false, false) => ' ',
                        (false, true, true, true) => '┬',
                        (false, true, true, false) => '┐',
                        (false, true, false, true) => '┌',
                        (false, true, false, false) => ' ',
                        (false, false, true, true) => '─',
                        (false, false, true, false) => ' ',
                        (false, false, false, true) => ' ',
                        (false, false, false, false) => ' ',
                    };
            }
        }

        // Write
        for y in (0..17).rev() {
            for x in 0..17 {
                write!(f, "{}", out[x][y])?;
            }
            writeln!(f)?;
        }

        match self.turn {
            Side::Player1 => writeln!(f, "Turn: x")?,
            Side::Player2 => writeln!(f, "Turn: o")?,
        }

        writeln!(f, "Player 1 path: {}", self.has_path(Side::Player1))?;
        writeln!(f, "Player 2 path: {}", self.has_path(Side::Player2))?;

        Ok(())
    }
}

#[cfg(test)]
mod fen_tests {
    use super::*;

    #[test]
    fn fen_round_trips() {
        let fen = "c2a4d4 b2c3 c1 c5 7 8";
        let pos = QuoridorPosition::from_fen(fen);
        assert_eq!(pos.get_fen(), fen);
        assert_eq!(pos.walls, [7, 8]);
        assert_eq!(pos.pieces[0], Some(QSquare::from_coords(2, 0)));
        assert_eq!(pos.pieces[1], Some(QSquare::from_coords(2, 4)));

        let none = "- - e1 e9 10 10";
        assert_eq!(QuoridorPosition::from_fen(none).get_fen(), none);
    }
}

#[cfg(test)]
mod jump_tests {
    use super::*;

    fn moves(fen: &str) -> Vec<QuoridorMove> {
        QuoridorPosition::from_fen(fen)
            .legal_moves()
            .into_iter()
            .filter(|mv| !matches!(mv, QuoridorMove::VWall(_) | QuoridorMove::HWall(_)))
            .collect()
    }

    fn has(moves: &[QuoridorMove], mv: QuoridorMove) -> bool {
        moves.contains(&mv)
    }

    #[test]
    fn path_distance_goes_around_walls() {
        let pos = QuoridorPosition::from_fen("- - e1 e9 10 10");
        assert_eq!(pos.path_distance(Side::Player1), Some(8));
        assert_eq!(pos.path_distance(Side::Player2), Some(8));

        // A wall above e1 and f1 costs each pawn one sideways step: player 1
        // leaves rank 1 and player 2 arrives at it
        let pos = QuoridorPosition::from_fen("e1 - e1 e9 10 10");
        assert_eq!(pos.path_distance(Side::Player1), Some(9));
        assert_eq!(pos.path_distance(Side::Player2), Some(9));

        // A wall that's out of player 2's way only affects player 1's route
        let pos = QuoridorPosition::from_fen("e1 - e1 a9 10 10");
        assert_eq!(pos.path_distance(Side::Player1), Some(9));
        assert_eq!(pos.path_distance(Side::Player2), Some(8));
    }

    #[test]
    fn straight_jump() {
        let m = moves("- - e4 e5 5 5");
        assert!(has(&m, QuoridorMove::Move(Direction::Up)));
        assert!(!m.iter().any(|mv| matches!(mv, QuoridorMove::Jump(..))));
    }

    #[test]
    fn edge_behind_opponent() {
        let m = moves("- - e8 e9 5 5");
        assert!(!has(&m, QuoridorMove::Move(Direction::Up)));
        assert!(has(&m, QuoridorMove::Jump(Direction::Up, Direction::Left)));
        assert!(has(&m, QuoridorMove::Jump(Direction::Up, Direction::Right)));
    }

    #[test]
    fn wall_behind_opponent() {
        let m = moves("e5 - e4 e5 5 5");
        assert!(!has(&m, QuoridorMove::Move(Direction::Up)));
        assert!(has(&m, QuoridorMove::Jump(Direction::Up, Direction::Left)));
        assert!(has(&m, QuoridorMove::Jump(Direction::Up, Direction::Right)));
    }

    #[test]
    fn wall_between_blocks_everything() {
        let m = moves("e4 - e4 e5 5 5");
        assert!(!has(&m, QuoridorMove::Move(Direction::Up)));
        assert!(!m.iter().any(|mv| matches!(mv, QuoridorMove::Jump(..))));
    }

    #[test]
    fn diagonals_can_be_blocked() {
        let m = moves("- d8 e8 e9 5 5");
        assert!(!has(&m, QuoridorMove::Jump(Direction::Up, Direction::Left)));
        assert!(has(&m, QuoridorMove::Jump(Direction::Up, Direction::Right)));
    }

    #[test]
    fn every_move_round_trips_and_stays_on_the_board() {
        for fen in [
            "- - e4 e5 5 5",
            "- - e8 e9 5 5",
            "e5 - e4 e5 5 5",
            "- d8 e8 e9 5 5",
            "- - a8 a9 5 5",
            "- - i1 i2 5 5",
        ] {
            let mut pos = QuoridorPosition::from_fen(fen);
            for mv in pos.legal_moves() {
                let before = pos.get_fen();
                pos.makemove(&mv);
                assert!(
                    pos.pieces[0].is_some() && pos.pieces[1].is_some(),
                    "{fen} {mv:?}"
                );
                pos.undomove(&mv);
                assert_eq!(pos.get_fen(), before, "{fen} {mv:?}");
            }
        }
    }
}
