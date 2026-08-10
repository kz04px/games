#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules};

    fn generated_before_stopping(fen: &str, stop_after: usize) -> usize {
        let pos = AtaxxPosition::<7, 7>::from_fen(fen);
        let mut seen = 0;
        pos.move_generator(|_| {
            seen += 1;
            seen >= stop_after
        });
        seen
    }

    #[test]
    fn stops_when_asked() {
        for fen in [
            "startpos",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
            "7/7/7/2x1o2/7/7/7 x 0 1",
        ] {
            let total = AtaxxPosition::<7, 7>::from_fen(fen).legal_moves().len();
            assert!(total > 2, "need a few moves to stop partway: {fen}");

            assert_eq!(generated_before_stopping(fen, 1), 1, "{fen}");
            assert_eq!(generated_before_stopping(fen, 2), 2, "{fen}");
            assert_eq!(
                generated_before_stopping(fen, total - 1),
                total - 1,
                "{fen}"
            );
        }
    }

    #[test]
    fn runs_to_the_end_when_not_asked() {
        for fen in ["startpos", "7/7/7/2x1o2/7/7/7 x 0 1"] {
            let total = AtaxxPosition::<7, 7>::from_fen(fen).legal_moves().len();
            assert_eq!(generated_before_stopping(fen, total + 1), total, "{fen}");
        }
    }
}
