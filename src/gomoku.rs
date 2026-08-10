use crate::{
    gamerules::{GameResult, GameRules},
    general::{mailbox::Mailbox, side::Side, square::Square},
};

#[derive(Clone, Default)]
pub struct GomokuPosition {
    pub board: Mailbox<Option<Side>, 15, 15>,
    pub turn: Side,
    pub fullmoves: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GomokuMove(pub Square<15, 15>);

impl GomokuPosition {
    #[must_use]
    pub fn get_square_char(&self, x: i32, y: i32) -> Option<char> {
        match self.board.get_piece_coords(x, y) {
            Some(Side::Player1) => Some('x'),
            Some(Side::Player2) => Some('o'),
            None => None,
        }
    }
}

impl GameRules for GomokuPosition {
    type MoveType = GomokuMove;
    const WIDTH: i32 = 15;
    const HEIGHT: i32 = 15;

    fn startpos() -> Self {
        Self {
            board: Mailbox::<Option<Side>, 15, 15>::default(),
            turn: Side::Player1,
            fullmoves: 1,
        }
    }

    fn set_piece(&mut self, x: i32, y: i32, c: char, _: usize) -> bool {
        let piece = match c {
            'x' => Some(Side::Player1),
            'o' => Some(Side::Player2),
            _ => None,
        };
        self.board.set_piece(x, y, piece);
        true
    }

    fn get_square_string(&self, x: i32, y: i32) -> Option<String> {
        self.get_square_char(x, y).map(|c| c.to_string())
    }

    fn move_generator(&self, mut func: impl FnMut(Self::MoveType) -> bool) {
        if self.is_gameover() {
            return;
        }

        for sq in &self.board {
            if self.board.get_piece_square(sq).is_none() && func(GomokuMove(sq)) {
                return;
            }
        }
    }

    fn get_turn(&self) -> Side {
        self.turn
    }

    fn makemove(&mut self, mv: &Self::MoveType) {
        debug_assert!(self.board.get_piece_square(mv.0).is_none());
        self.board.set_piece_square(mv.0, Some(self.turn));
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undomove(&mut self, mv: &Self::MoveType) {
        debug_assert!(self.board.get_piece_square(mv.0).is_some());
        self.board.set_piece_square(mv.0, None);
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
    }

    fn makenull(&mut self) {
        self.turn = !self.turn;
        self.fullmoves += (self.turn == Side::Player1) as i32;
    }

    fn undonull(&mut self) {
        self.turn = !self.turn;
        self.fullmoves -= (self.turn == Side::Player2) as i32;
    }

    fn is_valid(&self) -> Result<(), &'static str> {
        let p1 = self.board.count_piece(Some(Side::Player1));
        let p2 = self.board.count_piece(Some(Side::Player2));

        let expected = match self.turn {
            Side::Player1 => p1 == p2,
            Side::Player2 => p1 == p2 + 1,
        };
        if !expected {
            return Err("stone counts do not match the side to move");
        }

        if self.fullmoves < 1 {
            return Err("fullmove counter below one");
        }

        Ok(())
    }

    fn get_result(&self) -> Option<GameResult> {
        let mut empties = 0;

        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let Some(piece) = self.board.get_piece_coords(x, y) else {
                    empties += 1;
                    continue;
                };

                if self.board.streak::<5, 0, 1>(x, y) >= 5
                    || self.board.streak::<5, 1, 0>(x, y) >= 5
                    || self.board.streak::<5, 1, 1>(x, y) >= 5
                    || self.board.streak::<5, -1, 1>(x, y) >= 5
                {
                    return Some(GameResult::Win(piece));
                }
            }
        }

        (empties == 0).then_some(GameResult::Draw)
    }

    fn get_fen(&self) -> String {
        format!(
            "{} {} {}",
            self.get_board_fen(),
            match self.turn {
                Side::Player1 => "x",
                Side::Player2 => "o",
            },
            self.fullmoves,
        )
    }

    fn parse_fen_part(&mut self, idx: usize, part: &str) {
        match idx {
            0 => {}
            // Side to move
            1 => match part {
                "x" => self.turn = Side::Player1,
                "o" => self.turn = Side::Player2,
                _ => panic!("Uh oh {}", part),
            },
            // Fullmoves
            2 => self.fullmoves = part.parse::<i32>().unwrap(),
            _ => panic!("Invalid fen part index"),
        }
    }

    fn parse_move(&self, movestr: &str) -> Result<Self::MoveType, &'static str> {
        let sq = Square::<15, 15>::from_string(movestr)?;
        Ok(GomokuMove(sq))
    }
}

impl std::fmt::Display for GomokuPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in (0..Self::HEIGHT).rev() {
            for x in 0..Self::WIDTH {
                match self.get_square_char(x, y) {
                    Some(c) => write!(f, "{c}")?,
                    None => write!(f, "-")?,
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
