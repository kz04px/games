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
    fn makemove_hash_matches_fen_hash() {
        let fens = [
            "startpos",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 4 9",
            "r3k2r/8/8/8/8/8/8/R3K2R b KQkq - 4 9",
            "1R2k3/8/4K3/8/8/8/8/8 b - - 0 1",
            "8/P6k/8/8/8/8/p6K/8 w - - 0 1",
        ];

        for fen in fens {
            let mut pos = ChessPosition::from_fen(fen);
            assert_eq!(pos.hash(), ChessPosition::from_fen(&pos.get_fen()).hash());

            for mv in pos.legal_moves() {
                pos.makemove(&mv);
                assert_eq!(
                    pos.hash(),
                    ChessPosition::from_fen(&pos.get_fen()).hash(),
                    "hash differs after {mv:?}: {fen}"
                );
                pos.undomove(&mv);
            }
        }
    }

    #[test]
    fn random_playouts_hash_matches_fen_hash() {
        let fens = [
            "startpos",
            "r3k2r/pppp1ppp/8/8/8/8/PPPP1PPP/R3K2R w KQkq - 0 1",
            "r3k2r/1P4P1/8/3pP3/8/8/1p4p1/R3K2R w KQkq d6 0 1",
        ];

        let mut state = 0x9E3779B97F4A7C15u64;
        for fen in fens {
            for _ in 0..20 {
                let mut pos = ChessPosition::from_fen(fen);
                for _ in 0..150 {
                    let moves = pos.legal_moves();
                    if moves.is_empty() {
                        break;
                    }

                    state ^= state << 13;
                    state ^= state >> 7;
                    state ^= state << 17;
                    let mv = moves[state as usize % moves.len()];

                    pos.makemove(&mv);
                    assert_eq!(
                        pos.hash(),
                        ChessPosition::from_fen(&pos.get_fen()).hash(),
                        "hash differs after {mv:?}: {}",
                        pos.get_fen()
                    );
                }
            }
        }
    }

    #[test]
    fn makenull_undonull_hash() {
        let mut pos = ChessPosition::from_fen("r3k2r/8/8/3pP3/8/8/8/R3K2R w KQkq d6 0 1");
        let hash = pos.hash();

        pos.makenull();
        assert_eq!(pos.hash(), ChessPosition::from_fen(&pos.get_fen()).hash());
        pos.undonull();
        assert_eq!(pos.hash(), hash);
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
