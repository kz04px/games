#[cfg(test)]
mod tests {
    use games::{chess::ChessPosition, gamerules::GameRules};

    #[test]
    fn makemove_undomove_restores_hash() {
        let fens = [
            "startpos",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 4 9",
            "1R2k3/8/4K3/8/8/8/8/8 b - - 0 1",
        ];

        for fen in fens {
            let mut pos = ChessPosition::from_fen(fen);
            let hash = pos.hash();

            for mv in pos.legal_moves() {
                pos.makemove(&mv);
                pos.undomove(&mv);
                assert_eq!(pos.hash(), hash, "hash not restored after {mv:?}: {fen}");
            }
        }
    }

    #[test]
    fn transposition() {
        let mut pos = ChessPosition::startpos();
        let start_hash = pos.hash();

        for word in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            let mv = pos.parse_movestr(word).unwrap();
            pos.makemove(&mv);
        }

        assert_eq!(
            pos.get_board_fen(),
            ChessPosition::startpos().get_board_fen()
        );
        assert_eq!(pos.hash(), start_hash);
    }

    #[test]
    fn different_positions_differ() {
        let startpos = ChessPosition::startpos().hash();

        let mut after_e4 = ChessPosition::startpos();
        let mv = after_e4.parse_movestr("e2e4").unwrap();
        after_e4.makemove(&mv);

        assert_ne!(startpos, after_e4.hash());
    }

    #[test]
    fn castling_rights_affect_hash() {
        let with_rights = ChessPosition::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").hash();
        let without_rights = ChessPosition::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w - - 0 1").hash();

        assert_ne!(with_rights, without_rights);
    }

    #[test]
    fn en_passant_square_affects_hash() {
        let with_ep = ChessPosition::from_fen(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
        )
        .hash();
        let without_ep =
            ChessPosition::from_fen("rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2")
                .hash();

        assert_ne!(with_ep, without_ep);
    }
}
