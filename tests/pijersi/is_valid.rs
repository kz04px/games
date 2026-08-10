#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, general::side::Side, pijersi::PijersiPosition};

    #[test]
    fn pijersi_accepts_legal_positions() {
        for fen in [
            "startpos",
            "srp-r-s-2/p-1s-1r-1p-/4p-1/1RPw-3sr/2w-1RSP-/P-2WWS-2/R-S-S-1PR1 w 0 1",
            "6/7/6/RRr-5/6/7/6 w 0 1",
        ] {
            assert_eq!(PijersiPosition::from_fen(fen).is_valid(), Ok(()), "{fen}");
        }
    }

    #[test]
    fn pijersi_rejects_disagreeing_masks() {
        let base = PijersiPosition::from_fen("startpos");

        // A hex owned by both players.
        let mut both = base.clone();
        let ours = both.sides[Side::Player1];
        both.sides[Side::Player2] |= ours;
        assert!(both.is_valid().is_err(), "overlapping sides accepted");

        // A side mask claiming a hex that holds no piece.
        let mut ghost = base.clone();
        ghost.sides[Side::Player1] = !ghost.sides[Side::Player1];
        assert!(
            ghost.is_valid().is_err(),
            "side mask without pieces accepted"
        );

        // An upper-layer piece floating above an empty hex.
        let mut floating = base.clone();
        floating.pieces[0][1] = !floating.get_lower();
        assert!(floating.is_valid().is_err(), "floating stack accepted");

        // Two piece kinds on the same hex in the same layer.
        let mut doubled = base.clone();
        let rocks = doubled.pieces[0][0];
        doubled.pieces[1][0] |= rocks;
        assert!(doubled.is_valid().is_err(), "two kinds on one hex accepted");
    }
}
