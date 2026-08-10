#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, isolation::IsolationPosition};

    #[test]
    fn no_moves_when_over() {
        let fens = [
            "8/8/8/8/8/p.....1P b",
            "8/8/8/8/8/p1.....P r",
            "8/8/8/8/8/6Pp b",
        ];

        for fen in fens {
            let pos = IsolationPosition::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }
    }

    #[test]
    fn moves_when_not_over() {
        let fens = ["startpos", "8/8/8/8/8/p......P b"];

        for fen in fens {
            let pos = IsolationPosition::from_fen(fen);
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
