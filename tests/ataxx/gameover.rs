#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules};

    #[test]
    fn no_moves_when_over() {
        let fens = [
            "xxxxxxx/xxxxxxx/xxxxxxx/xxxxooo/ooooooo/ooooooo/ooooooo x 0 1",
            "x5o/7/7/7/7/7/o5x x 100 1",
        ];

        for fen in fens {
            let pos = AtaxxPosition::<7, 7>::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }
    }

    #[test]
    fn moves_when_not_over() {
        let fens = [
            "startpos",
            "xxxxxxx/xxxxxxx/xxxxxxx/xxx1ooo/ooooooo/ooooooo/ooooooo x 0 1",
        ];

        for fen in fens {
            let pos = AtaxxPosition::<7, 7>::from_fen(fen);
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
