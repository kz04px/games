#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, pijersi::PijersiPosition};

    #[test]
    fn no_moves_when_over() {
        let fens = [
            "R-4r-/7/6/7/6/7/6 w 0 1",
            "6/7/6/7/6/7/R-4r- w 0 1",
            "6/7/6/R-3r-1/6/7/6 w 20 1",
        ];

        for fen in fens {
            let pos = PijersiPosition::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }
    }

    #[test]
    fn moves_when_not_over() {
        let fens = ["startpos", "6/7/6/R-3r-1/6/7/6 w 0 1"];

        for fen in fens {
            let pos = PijersiPosition::from_fen(fen);
            assert!(!pos.is_gameover(), "expected an unfinished game: {fen}");
            assert!(pos.count_moves() > 0, "no moves in a live game: {fen}");
            assert_eq!(
                pos.legal_moves().len() as u64,
                pos.count_moves(),
                "count_moves disagrees with legal_moves: {fen}"
            );
        }
    }
}
