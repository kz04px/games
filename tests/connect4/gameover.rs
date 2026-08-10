#[cfg(test)]
mod tests {
    use games::{connect4::Connect4Position, gamerules::GameRules};

    #[test]
    fn no_moves_when_over() {
        let fens = ["7/7/7/7/7/rrrr3 y 1", "7/7/7/7/7/yyyy3 r 1"];

        for fen in fens {
            let pos = Connect4Position::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }
    }

    #[test]
    fn moves_when_not_over() {
        let fens = ["startpos", "7/7/7/7/7/rrr4 y 1"];

        for fen in fens {
            let pos = Connect4Position::from_fen(fen);
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
