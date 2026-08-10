#[cfg(test)]
mod tests {
    use games::{
        chess::ChessPosition,
        gamerules::{GameResult, GameRules},
        general::side::Side,
    };

    #[test]
    fn chess() {
        for fen in [
            "1R2k3/8/4K3/8/8/8/8/8 b - - 0 1",
            "8/8/8/8/8/4k3/8/1r2K3 w - - 0 1",
            "7k/5Q2/6K1/8/8/8/8/8 b - - 0 1",
        ] {
            let pos = ChessPosition::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }

        for fen in [
            "startpos",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 99 1",
        ] {
            let pos = ChessPosition::from_fen(fen);
            assert!(!pos.is_gameover(), "expected an unfinished game: {fen}");
            assert!(pos.count_moves() > 0, "no moves in a live game: {fen}");
            assert_eq!(
                pos.legal_moves().len() as u64,
                pos.count_moves(),
                "count_moves disagrees with legal_moves: {fen}"
            );
        }
    }

    #[test]
    fn fifty_move_rule_is_over_but_not_stuck() {
        let pos =
            ChessPosition::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 100 1");
        assert!(pos.is_gameover());
        assert!(!pos.legal_moves().is_empty());
    }

    #[test]
    fn checkmate_takes_priority_over_fifty_move_rule() {
        let pos = ChessPosition::from_fen("1R2k3/8/4K3/8/8/8/8/8 b - - 100 1");
        assert_eq!(pos.get_result(), Some(GameResult::Win(Side::Player1)));
    }
}
