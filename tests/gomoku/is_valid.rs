#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, gomoku::GomokuPosition};

    #[test]
    fn gomoku_counts_must_match_the_side_to_move() {
        let empty = "15/15/15/15/15/15/15/15/15/15/15/15/15/15/15";
        assert_eq!(
            GomokuPosition::from_fen(&format!("{empty} x 1")).is_valid(),
            Ok(())
        );

        // One stone played, so it must be Player 2's turn.
        let one = "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x14";
        assert_eq!(
            GomokuPosition::from_fen(&format!("{one} o 1")).is_valid(),
            Ok(())
        );
        assert!(
            GomokuPosition::from_fen(&format!("{one} x 1"))
                .is_valid()
                .is_err(),
            "one stone on the board cannot be Player 1's turn again"
        );

        // Player 2 cannot be ahead, and cannot be two behind.
        let lopsided = "15/15/15/15/15/15/15/15/15/15/15/15/15/15/oo13";
        assert!(
            GomokuPosition::from_fen(&format!("{lopsided} x 1"))
                .is_valid()
                .is_err()
        );
        let two_ahead = "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xx13";
        assert!(
            GomokuPosition::from_fen(&format!("{two_ahead} o 1"))
                .is_valid()
                .is_err()
        );
    }

    #[test]
    fn gomoku_stays_valid_across_moves() {
        let mut pos = GomokuPosition::from_fen("startpos");
        for mv in pos.legal_moves().into_iter().take(20) {
            pos.makemove(&mv);
            assert_eq!(pos.is_valid(), Ok(()), "after {mv:?}");
            pos.undomove(&mv);
            assert_eq!(pos.is_valid(), Ok(()), "after undoing {mv:?}");
        }
    }
}
