#[cfg(test)]
mod tests {
    use games::{
        chess::ChessPosition,
        gamerules::{GameResult, GameRules},
    };

    #[test]
    fn threefold_repetition_via_knight_shuffle() {
        let mut pos = ChessPosition::startpos();
        assert_eq!(pos.get_result(), None);

        // Second occurrence of the starting position.
        for word in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            let mv = pos.parse_movestr(word).unwrap();
            pos.makemove(&mv);
        }
        assert_eq!(pos.get_result(), None, "shouldn't be a draw yet");

        // Third occurrence of the starting position.
        for word in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            let mv = pos.parse_movestr(word).unwrap();
            pos.makemove(&mv);
        }
        assert_eq!(pos.get_result(), Some(GameResult::Draw));
        assert!(pos.is_gameover());
    }

    #[test]
    fn undomove_restores_the_repetition_count() {
        let mut pos = ChessPosition::startpos();
        let mut moves = vec![];

        for word in [
            "g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1", "f6g8",
        ] {
            let mv = pos.parse_movestr(word).unwrap();
            pos.makemove(&mv);
            moves.push(mv);
        }
        assert_eq!(pos.get_result(), Some(GameResult::Draw));

        pos.undomove(moves.last().unwrap());
        assert_eq!(pos.get_result(), None);
    }

    #[test]
    fn a_captured_pawn_never_repeats_the_position() {
        // Capturing changes the piece hash permanently, so the pre-capture
        // position can never recur no matter how the remaining pieces shuffle.
        let mut pos =
            ChessPosition::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let start_hash = pos.hash();

        for word in ["e2e4", "d7d5", "e4d5"] {
            let mv = pos.parse_movestr(word).unwrap();
            pos.makemove(&mv);
        }

        assert_ne!(pos.hash(), start_hash);
        assert_eq!(pos.get_result(), None);
    }
}
