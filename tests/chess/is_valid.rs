#[cfg(test)]
mod tests {
    use games::{chess::ChessPosition, gamerules::GameRules};

    #[test]
    fn valid() {
        let fens = [
            "startpos",
            "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 4 9",
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            "8/8/8/4k3/8/8/8/4K3 b - - 0 1",
        ];

        for fen in fens {
            assert_eq!(ChessPosition::from_fen(fen).is_valid(), Ok(()), "{fen}");
        }
    }

    #[test]
    fn invalid() {
        let fens = [
            // One king each
            "8/8/8/8/8/8/8/4K3 w - - 0 1",
            "4k3/8/8/8/8/8/8/8 w - - 0 1",
            "4k3/8/8/8/8/8/8/3KK3 w - - 0 1",
            // Pawn on the back rank
            "4k3/8/8/8/8/8/8/P3K3 w - - 0 1",
            "P3k3/8/8/8/8/8/8/4K3 w - - 0 1",
            // Can capture king
            "4r2k/8/8/8/8/8/8/4K3 b - - 0 1",
            "4R2K/8/8/8/8/8/8/4k3 w - - 0 1",
            // Invalid castling rights
            "4k3/8/8/8/8/8/8/4K3 w K - 0 1",
            "4k3/8/8/8/8/8/8/4K2R w Q - 0 1",
            "r3k3/8/8/8/8/8/8/4K3 w k - 0 1",
            // En passant square
            "4k3/8/8/8/8/8/8/4K3 w - e6 0 1",
        ];

        for fen in fens {
            assert!(
                ChessPosition::from_fen(fen).is_valid().is_err(),
                "accepted an invalid position: {fen}"
            );
        }
    }
}
