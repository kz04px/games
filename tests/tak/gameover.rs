//! A finished game must have no continuations.
//!
//! Tak checks for this explicitly, unlike gomoku, which had to be taught to.

#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn tak() {
        let over = [
            "x6/x6/x6/x6/x6/1,1,1,1,1,1 1 1",
            "x6/x6/x6/x6/x6/2,2,2,2,2,2 2 1",
        ];
        for fen in over {
            let pos = TakPosition::<6>::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }

        let not_over = ["startpos", "x6/x6/x6/x6/x6/1,1,1,1,1,x 2 1"];
        for fen in not_over {
            let pos = TakPosition::<6>::from_fen(fen);
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
