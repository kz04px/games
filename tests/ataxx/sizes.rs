#[cfg(test)]
mod tests {
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::{GameResult, GameRules},
        general::side::Side,
    };

    const EMPTY: char = '.';

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum RefMove {
        Pass,
        Single(usize, usize),
        Double(usize, usize, usize, usize),
    }

    #[derive(Clone)]
    struct Reference {
        board: Vec<Vec<char>>,
        width: usize,
        height: usize,
        turn: usize,
        halfmoves: i32,
    }

    impl Reference {
        fn from_fen(width: usize, height: usize, fen: &str) -> Self {
            let parts: Vec<&str> = fen.split(' ').collect();
            let mut board = vec![vec![EMPTY; width]; height];
            let mut y = height - 1;
            let mut x = 0;
            let mut run = 0;

            for c in parts[0].chars() {
                match c {
                    '0'..='9' => run = 10 * run + c.to_digit(10).unwrap() as usize,
                    '/' => {
                        y = y.saturating_sub(1);
                        x = 0;
                        run = 0;
                    }
                    _ => {
                        x += run;
                        run = 0;
                        board[y][x] = c;
                        x += 1;
                    }
                }
            }

            Self {
                board,
                width,
                height,
                turn: usize::from(parts[1] == "o"),
                halfmoves: parts[2].parse().unwrap(),
            }
        }

        fn us(&self) -> char {
            ['x', 'o'][self.turn]
        }

        fn them(&self) -> char {
            ['o', 'x'][self.turn]
        }

        fn count(&self, side: char) -> usize {
            self.board.iter().flatten().filter(|c| **c == side).count()
        }

        fn is_finished(&self) -> bool {
            self.halfmoves >= 100 || self.count('x') == 0 || self.count('o') == 0
        }

        fn piece_moves(&self, side: char) -> Vec<RefMove> {
            let (w, h) = (self.width as i32, self.height as i32);
            let mut moves = vec![];

            for ty in 0..h {
                for tx in 0..w {
                    if self.board[ty as usize][tx as usize] != EMPTY {
                        continue;
                    }

                    let mut can_grow = false;
                    for fy in 0..h {
                        for fx in 0..w {
                            if self.board[fy as usize][fx as usize] != side {
                                continue;
                            }
                            match (fx - tx).abs().max((fy - ty).abs()) {
                                1 => can_grow = true,
                                2 => moves.push(RefMove::Double(
                                    fx as usize,
                                    fy as usize,
                                    tx as usize,
                                    ty as usize,
                                )),
                                _ => {}
                            }
                        }
                    }

                    if can_grow {
                        moves.push(RefMove::Single(tx as usize, ty as usize));
                    }
                }
            }

            moves
        }

        fn moves(&self) -> Vec<RefMove> {
            if self.is_finished() {
                return vec![];
            }

            let mine = self.piece_moves(self.us());
            if !mine.is_empty() {
                mine
            } else if self.piece_moves(self.them()).is_empty() {
                vec![]
            } else {
                vec![RefMove::Pass]
            }
        }

        fn place(&mut self, x: usize, y: usize) {
            let (us, them) = (self.us(), self.them());
            self.board[y][x] = us;

            for dy in -1i32..=1 {
                for dx in -1i32..=1 {
                    let (nx, ny) = (x as i32 + dx, y as i32 + dy);
                    if nx < 0 || ny < 0 || nx >= self.width as i32 || ny >= self.height as i32 {
                        continue;
                    }
                    if self.board[ny as usize][nx as usize] == them {
                        self.board[ny as usize][nx as usize] = us;
                    }
                }
            }
        }

        fn makemove(&mut self, mv: RefMove) {
            match mv {
                RefMove::Pass => self.halfmoves += 1,
                RefMove::Single(x, y) => {
                    self.place(x, y);
                    self.halfmoves = 0;
                }
                RefMove::Double(fx, fy, x, y) => {
                    self.board[fy][fx] = EMPTY;
                    self.place(x, y);
                    self.halfmoves += 1;
                }
            }
            self.turn = 1 - self.turn;
        }

        fn perft(&self, depth: i32) -> u64 {
            if depth == 0 {
                return 1;
            }

            let mut nodes = 0;
            for mv in self.moves() {
                let mut next = self.clone();
                next.makemove(mv);
                nodes += next.perft(depth - 1);
            }
            nodes
        }
    }

    fn compare_moves<const WIDTH: usize, const HEIGHT: usize>(
        pos: &mut AtaxxPosition<WIDTH, HEIGHT>,
        depth: i32,
    ) {
        let fen = pos.get_fen();
        let expected = Reference::from_fen(WIDTH, HEIGHT, &fen).moves().len() as u64;

        assert_eq!(
            pos.legal_moves().len() as u64,
            expected,
            "{WIDTH}x{HEIGHT}: {fen}"
        );
        assert_eq!(pos.count_moves(), expected, "{WIDTH}x{HEIGHT}: {fen}");

        if depth == 0 {
            return;
        }

        for mv in pos.legal_moves() {
            pos.makemove(&mv);
            compare_moves(pos, depth - 1);
            pos.undomove(&mv);
        }
    }

    fn cross_check<const WIDTH: usize, const HEIGHT: usize>(fens: &[&str], depth: i32) {
        for fen in fens {
            let mut pos = AtaxxPosition::<WIDTH, HEIGHT>::from_fen(fen);
            let reference = Reference::from_fen(WIDTH, HEIGHT, &pos.get_fen());

            for d in 0..=depth {
                assert_eq!(
                    pos.perft(d),
                    reference.perft(d),
                    "{WIDTH}x{HEIGHT} depth {d}: {fen}"
                );
            }

            compare_moves(&mut pos, depth - 2);
        }
    }

    #[test]
    fn perft_square() {
        cross_check::<4, 4>(
            &[
                "startpos",
                "x2o/1--1/1--1/o2x x 0 1",
                "x2o/4/4/o2x o 0 1",
                "oxxo/----/----/4 x 0 1",
            ],
            4,
        );
        cross_check::<5, 5>(
            &["startpos", "x3o/5/2-2/5/o3x x 0 1", "x3o/5/5/5/o3x o 0 1"],
            4,
        );
        cross_check::<6, 6>(&["startpos", "x4o/6/2--2/2--2/6/o4x x 0 1"], 4);
        cross_check::<7, 7>(&["startpos", "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1"], 4);
    }

    #[test]
    fn perft_8x8() {
        cross_check::<8, 8>(
            &[
                "startpos",
                "x6o/8/2----2/8/8/2----2/8/o6x x 0 1",
                "8/8/8/8/8/--------/--------/1oooxxxx x 0 1",
                "8/8/8/8/8/--------/--------/ooooxxxx x 0 1",
                "xxxxxxxx/xxxxxxxx/xxxxxxxx/xxxxxxxx/oooooooo/oooooooo/oooooooo/oooooooo x 0 1",
            ],
            4,
        );
    }

    #[test]
    fn perft_oblong() {
        cross_check::<8, 4>(&["startpos", "x6o/2----2/8/o6x x 0 1"], 4);
        cross_check::<4, 8>(&["startpos", "x2o/4/1--1/4/4/1--1/4/o2x x 0 1"], 4);
        cross_check::<3, 7>(&["startpos", "x1o/3/1-1/3/1-1/3/o1x x 0 1"], 4);
        cross_check::<16, 4>(&["startpos", "x14o/16/4--------4/o14x x 0 1"], 3);
        cross_check::<32, 2>(&["startpos"], 3);
    }

    #[test]
    fn perft_lines() {
        cross_check::<1, 8>(&["startpos", "x/1/1/-/1/1/1/o x 0 1"], 5);
        cross_check::<8, 1>(&["startpos", "o1-3x x 0 1"], 5);
        cross_check::<1, 16>(&["startpos"], 4);
        cross_check::<64, 1>(&["startpos"], 4);
    }

    #[test]
    fn startpos_per_shape() {
        assert_eq!(
            AtaxxPosition::<4, 4>::startpos().get_fen(),
            "x2o/4/4/o2x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<7, 7>::startpos().get_fen(),
            "x5o/7/7/7/7/7/o5x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<8, 8>::startpos().get_fen(),
            "x6o/8/8/8/8/8/8/o6x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<8, 4>::startpos().get_fen(),
            "x6o/8/8/o6x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<4, 8>::startpos().get_fen(),
            "x2o/4/4/4/4/4/4/o2x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<16, 4>::startpos().get_fen(),
            "x14o/16/16/o14x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<1, 8>::startpos().get_fen(),
            "x/1/1/1/1/1/1/o x 0 1"
        );
        assert_eq!(AtaxxPosition::<8, 1>::startpos().get_fen(), "o6x x 0 1");
    }

    #[test]
    fn startpos_is_playable() {
        fn check<const WIDTH: usize, const HEIGHT: usize>(moves: usize) {
            let pos = AtaxxPosition::<WIDTH, HEIGHT>::startpos();
            assert_eq!(pos.get_black().count(), pos.get_white().count(), "lopsided");
            assert_eq!(pos.get_result(), None, "{WIDTH}x{HEIGHT} starts finished");
            assert_eq!(pos.legal_moves().len(), moves, "{WIDTH}x{HEIGHT}");
        }

        check::<7, 7>(16);
        check::<8, 8>(16);
        check::<16, 4>(16);
        check::<32, 2>(8);
        check::<1, 8>(2);
        check::<8, 1>(2);
        check::<1, 64>(2);
        check::<64, 1>(2);
    }

    #[test]
    fn the_smallest_boards_start_finished() {
        assert_eq!(AtaxxPosition::<1, 2>::startpos().get_fen(), "x/o x 0 1");
        assert_eq!(AtaxxPosition::<2, 1>::startpos().get_fen(), "ox x 0 1");

        for result in [
            AtaxxPosition::<1, 2>::startpos().get_result(),
            AtaxxPosition::<2, 1>::startpos().get_result(),
        ] {
            assert_eq!(result, Some(GameResult::Draw));
        }
        assert_eq!(AtaxxPosition::<1, 2>::startpos().legal_moves().len(), 0);
        assert_eq!(AtaxxPosition::<2, 1>::startpos().legal_moves().len(), 0);
    }

    fn as_string<const WIDTH: usize, const HEIGHT: usize>(mv: &AtaxxMove<WIDTH, HEIGHT>) -> String {
        match mv {
            AtaxxMove::Pass => "0000".to_owned(),
            AtaxxMove::Single(to) => to.to_string(),
            AtaxxMove::Double(fr, to) => format!("{fr}{to}"),
        }
    }

    fn move_strings<const WIDTH: usize, const HEIGHT: usize>(fen: &str) -> Vec<String> {
        let mut moves: Vec<String> = AtaxxPosition::<WIDTH, HEIGHT>::from_fen(fen)
            .legal_moves()
            .iter()
            .map(as_string)
            .collect();
        moves.sort();
        moves
    }

    #[test]
    fn the_last_file_does_not_touch_the_first() {
        let mut expected = vec!["g1", "g2", "h2", "h1f1", "h1f2", "h1f3", "h1g3", "h1h3"];
        expected.sort();
        assert_eq!(move_strings::<8, 8>("o7/8/8/8/8/8/8/7x x 0 1"), expected);

        let mut expected = vec!["o1", "o2", "p2", "p1n1", "p1n2", "p1n3", "p1o3", "p1p3"];
        expected.sort();
        assert_eq!(move_strings::<16, 4>("o15/16/16/15x x 0 1"), expected);
    }

    #[test]
    fn move_strings_on_a_wide_board() {
        assert_eq!(
            move_strings::<64, 1>("startpos"),
            vec!["ak1", "al1aj1"],
            "64 wide"
        );
        assert_eq!(
            move_strings::<32, 2>("32/o30x x 0 1"),
            vec!["E1", "E2", "F1D1", "F1D2", "F2"],
            "32 wide"
        );
    }

    #[test]
    fn move_strings_round_trip() {
        fn check<const WIDTH: usize, const HEIGHT: usize>() {
            let pos = AtaxxPosition::<WIDTH, HEIGHT>::startpos();
            let moves = pos.legal_moves();
            assert!(!moves.is_empty(), "{WIDTH}x{HEIGHT}");

            for mv in moves {
                let text = as_string(&mv);
                assert_eq!(pos.parse_move(&text), Ok(mv), "{WIDTH}x{HEIGHT}: {text}");
            }
        }

        check::<7, 7>();
        check::<8, 8>();
        check::<16, 4>();
        check::<32, 2>();
        check::<64, 1>();
        check::<1, 64>();
        check::<1, 16>();
    }

    #[test]
    fn a_capture_does_not_reach_around_an_edge() {
        let mut pos = AtaxxPosition::<8, 8>::from_fen("8/8/8/8/8/8/o7/6x1 x 0 1");
        let mv = pos.parse_move("h1").unwrap();
        pos.makemove(&mv);

        assert_eq!(pos.get_fen(), "8/8/8/8/8/8/o7/6xx o 0 1");
    }

    #[test]
    fn move_parsing_respects_the_shape() {
        assert!(AtaxxPosition::<8, 8>::default().parse_move("h8").is_ok());
        assert!(AtaxxPosition::<7, 7>::default().parse_move("h1").is_err());
        assert!(AtaxxPosition::<7, 7>::default().parse_move("a8").is_err());
        assert!(AtaxxPosition::<8, 4>::default().parse_move("h4").is_ok());
        assert!(AtaxxPosition::<8, 4>::default().parse_move("h5").is_err());
        assert!(AtaxxPosition::<4, 8>::default().parse_move("d8").is_ok());
        assert!(AtaxxPosition::<4, 8>::default().parse_move("e8").is_err());
        assert!(AtaxxPosition::<1, 8>::default().parse_move("a8").is_ok());
        assert!(AtaxxPosition::<1, 8>::default().parse_move("b1").is_err());
        assert!(AtaxxPosition::<8, 1>::default().parse_move("h1").is_ok());
        assert!(AtaxxPosition::<8, 1>::default().parse_move("a2").is_err());
        assert_eq!(
            AtaxxPosition::<4, 4>::default().parse_move("0000"),
            Ok(AtaxxMove::Pass)
        );
    }

    #[test]
    fn results_per_shape() {
        assert_eq!(
            AtaxxPosition::<4, 4>::from_fen("x3/4/4/4 o 0 1").get_result(),
            Some(GameResult::Win(Side::Player1))
        );
        assert_eq!(
            AtaxxPosition::<8, 8>::from_fen("8/8/8/8/8/8/8/o7 x 0 1").get_result(),
            Some(GameResult::Win(Side::Player2))
        );
        assert_eq!(
            AtaxxPosition::<1, 8>::from_fen("x/1/1/1/1/1/1/1 o 0 1").get_result(),
            Some(GameResult::Win(Side::Player1))
        );

        let full = "xxxxxxxx/xxxxxxxx/xxxxxxxx/xxxxxxxx/oooooooo/oooooooo/oooooooo/oooooooo";
        let pos = AtaxxPosition::<8, 8>::from_fen(&format!("{full} x 0 1"));
        assert_eq!(pos.get_result(), Some(GameResult::Draw));
        assert!(pos.get_empty().is_empty());

        let x_ahead = "xxxxxxxx/xxxxxxxx/xxxxxxxx/xxxxxxxx/xooooooo/oooooooo/oooooooo/oooooooo";
        assert_eq!(
            AtaxxPosition::<8, 8>::from_fen(&format!("{x_ahead} o 0 1")).get_result(),
            Some(GameResult::Win(Side::Player1))
        );

        assert_eq!(
            AtaxxPosition::<8, 1>::from_fen("ooooxxxx x 0 1").get_result(),
            Some(GameResult::Draw)
        );
        assert_eq!(
            AtaxxPosition::<8, 1>::from_fen("oooxxxxx x 0 1").get_result(),
            Some(GameResult::Win(Side::Player1))
        );
    }

    #[test]
    fn fens_round_trip_per_shape() {
        assert_eq!(
            AtaxxPosition::<4, 4>::from_fen("x2o/1--1/1--1/o2x o 3 7").get_fen(),
            "x2o/1--1/1--1/o2x o 3 7"
        );
        assert_eq!(
            AtaxxPosition::<8, 4>::from_fen("x6o/2----2/8/o6x x 0 1").get_fen(),
            "x6o/2----2/8/o6x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<4, 8>::from_fen("x2o/4/1--1/4/4/1--1/4/o2x o 12 30").get_fen(),
            "x2o/4/1--1/4/4/1--1/4/o2x o 12 30"
        );
        assert_eq!(
            AtaxxPosition::<16, 4>::from_fen("x14o/16/4--------4/o14x x 0 1").get_fen(),
            "x14o/16/4--------4/o14x x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<32, 2>::from_fen("x30o/o30x o 99 100").get_fen(),
            "x30o/o30x o 99 100"
        );
        assert_eq!(
            AtaxxPosition::<1, 8>::from_fen("x/1/1/-/1/1/1/o x 0 1").get_fen(),
            "x/1/1/-/1/1/1/o x 0 1"
        );
        assert_eq!(
            AtaxxPosition::<64, 1>::from_fen("o62x x 0 1").get_fen(),
            "o62x x 0 1"
        );
    }
}
