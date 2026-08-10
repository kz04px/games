#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules, general::side::Side};

    #[test]
    fn ataxx_accepts_legal_positions() {
        for fen in [
            "startpos",
            "x5o/7/7/7/7/7/o5x x 0 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
        ] {
            assert_eq!(
                AtaxxPosition::<7, 7>::from_fen(fen).is_valid(),
                Ok(()),
                "{fen}"
            );
        }
    }

    #[test]
    fn ataxx_rejects_overlapping_masks() {
        let base = AtaxxPosition::<7, 7>::startpos();

        let mut black_white = base.clone();
        let white = base.get_white();
        black_white.pieces[Side::Player1] |= white;
        assert!(
            black_white.is_valid().is_err(),
            "black & white overlap accepted"
        );

        let mut black_blocker = base.clone();
        let black = base.get_black();
        black_blocker.blockers |= black;
        assert!(
            black_blocker.is_valid().is_err(),
            "black & blockers overlap accepted"
        );

        let mut white_blocker = base.clone();
        let white = base.get_white();
        white_blocker.blockers |= white;
        assert!(
            white_blocker.is_valid().is_err(),
            "white & blockers overlap accepted"
        );
    }
}
