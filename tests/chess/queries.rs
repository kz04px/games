#[cfg(test)]
mod tests {
    use games::{
        chess::{ChessMove, ChessPosition},
        gamerules::{GameResult, GameRules},
    };

    #[test]
    fn insufficient_material() {
        let cases = [
            ("8/8/8/4k3/8/8/8/4K3 w - - 0 1", true, "king vs king"),
            (
                "8/8/8/4k3/8/8/8/3NK3 w - - 0 1",
                true,
                "king and knight vs king",
            ),
            (
                "8/8/8/4k3/8/8/8/3BK3 w - - 0 1",
                true,
                "king and bishop vs king",
            ),
            (
                "6b1/8/8/4k3/8/8/8/3BK3 w - - 0 1",
                true,
                "bishops on the same colour",
            ),
            (
                "5b2/8/8/4k3/8/8/8/3BK3 w - - 0 1",
                false,
                "bishops on opposite colours",
            ),
            (
                "8/8/8/4k3/8/B7/8/2B1K3 w - - 0 1",
                true,
                "two bishops on the same colour vs king",
            ),
            (
                "8/8/8/4k3/8/8/8/N2NK3 w - - 0 1",
                false,
                "two knights vs king",
            ),
            ("8/8/8/4k3/8/8/4P3/4K3 w - - 0 1", false, "a pawn"),
            ("8/8/8/4k3/8/8/8/3RK3 w - - 0 1", false, "a rook"),
            ("8/8/8/4k3/8/8/8/3QK3 w - - 0 1", false, "a queen"),
            ("startpos", false, "the start position"),
        ];

        for (fen, expected, name) in cases {
            let pos = ChessPosition::from_fen(fen);
            assert_eq!(pos.is_insufficient_material(), expected, "{name}: {fen}");
        }
    }

    #[test]
    fn insufficient_material_is_not_part_of_get_result() {
        let pos = ChessPosition::from_fen("8/8/8/4k3/8/8/8/4K3 w - - 0 1");
        assert!(pos.is_insufficient_material());
        assert_eq!(pos.get_result(), None);
    }

    #[test]
    fn insufficient_material_is_the_same_from_either_side() {
        let white = ChessPosition::from_fen("6b1/8/8/4k3/8/8/8/3BK3 w - - 0 1");
        let black = ChessPosition::from_fen("6b1/8/8/4k3/8/8/8/3BK3 b - - 0 1");
        assert_eq!(
            white.is_insufficient_material(),
            black.is_insufficient_material()
        );
    }

    #[test]
    fn fifty_move() {
        let below = ChessPosition::from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 99 60");
        let at = ChessPosition::from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 100 60");

        assert!(!below.is_fifty_move());
        assert!(at.is_fifty_move());
        assert_eq!(at.get_result(), Some(GameResult::Draw));
    }

    #[test]
    fn legal_moves_into_matches_legal_moves() {
        let fens = [
            "startpos",
            "r3k2r/pppp1ppp/8/8/8/8/PPPP1PPP/R3K2R w KQkq - 0 1",
            "r3k2r/1P4P1/8/3pP3/8/8/1p4p1/R3K2R w KQkq d6 0 1",
            "R6R/3Q4/1Q4Q1/4Q3/2Q4Q/Q4Q2/pp1Q4/kBNN1KB1 w - - 0 1",
            "4k3/8/8/8/8/8/4q3/4K3 w - - 0 1",
        ];

        for fen in fens {
            let pos = ChessPosition::from_fen(fen);
            let mut buffer = [ChessMove::default(); ChessPosition::MAX_LEGAL_MOVES];
            let count = pos.legal_moves_into(&mut buffer);
            assert_eq!(&buffer[..count], pos.legal_moves().as_slice(), "{fen}");
        }
    }

    #[test]
    fn default_move_is_not_a_legal_move() {
        let pos = ChessPosition::startpos();
        let placeholder = ChessMove::default();
        assert_eq!(placeholder.from, placeholder.to);
        assert!(!pos.legal_moves().contains(&placeholder));
    }
}
