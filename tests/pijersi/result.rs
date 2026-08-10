#[cfg(test)]
mod tests {
    use games::{
        gamerules::{GameResult, GameRules},
        general::side::Side,
        pijersi::PijersiPosition,
    };

    #[test]
    fn results() {
        let tests = [
            ("startpos", None),
            ("6/7/6/R-3r-1/6/7/6 w 0 1", None),
            (
                "R-4r-/7/6/7/6/7/6 w 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "6/7/6/7/6/7/R-4r- w 0 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            (
                "r-4R-/7/6/7/6/7/6 w 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            ("W-4r-/7/6/7/6/7/6 w 0 1", None),
            ("6/7/6/R-6/6/7/w-5 w 0 1", None),
            // Losing every piece loses the game.
            (
                "6/7/6/R-6/6/7/6 w 0 1",
                Some(GameResult::Win(Side::Player1)),
            ),
            (
                "6/7/6/6r-5/6/7/6 w 0 1",
                Some(GameResult::Win(Side::Player2)),
            ),
            // The halfmove clock draws at 20, and not before.
            ("6/7/6/R-3r-1/6/7/6 w 19 1", None),
            ("6/7/6/R-3r-1/6/7/6 w 20 1", Some(GameResult::Draw)),
            ("6/7/6/R-3r-1/6/7/6 w 21 1", Some(GameResult::Draw)),
        ];

        for (fen, expected) in tests {
            let pos = PijersiPosition::from_fen(fen);
            assert_eq!(pos.get_result(), expected, "{fen}");
            assert_eq!(pos.is_gameover(), expected.is_some(), "{fen}");
        }
    }
}
