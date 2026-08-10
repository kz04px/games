#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, general::side::Side, pijersi::PijersiPosition};

    #[test]
    fn counters() {
        let mut pos = PijersiPosition::startpos();
        assert_eq!(pos.halfmoves, 0);
        assert_eq!(pos.fullmoves, 1);

        let mut expected_halfmoves = 0;
        let mut expected_fullmoves = 1;

        for ply in 0..6 {
            let pieces_before = pos.get_white().count() + pos.get_black().count();

            let mv = pos.legal_moves()[0];
            pos.makemove(&mv);

            let pieces_after = pos.get_white().count() + pos.get_black().count();
            let captured = pieces_after < pieces_before;

            expected_halfmoves = if captured { 0 } else { expected_halfmoves + 1 };
            if pos.get_turn() == Side::Player1 {
                expected_fullmoves += 1;
            }

            assert_eq!(pos.halfmoves, expected_halfmoves, "ply {ply}");
            assert_eq!(pos.fullmoves, expected_fullmoves, "ply {ply}");
        }
    }
}
