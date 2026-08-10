#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, pijersi::PijersiPosition};

    fn check(pos: &mut PijersiPosition, depth: i32) {
        assert_eq!(pos.is_valid(), Ok(()), "{}", pos.get_fen());

        if depth == 0 {
            return;
        }

        for mv in pos.legal_moves() {
            let before = pos.clone();

            pos.makemove(&mv);
            check(pos, depth - 1);
            pos.undomove(&mv);

            assert_eq!(pos.sides, before.sides, "sides: {}", before.get_fen());
            assert_eq!(pos.pieces, before.pieces, "pieces: {}", before.get_fen());
            assert_eq!(pos.turn, before.turn, "turn: {}", before.get_fen());
            assert_eq!(
                pos.halfmoves,
                before.halfmoves,
                "halfmoves: {}",
                before.get_fen()
            );
            assert_eq!(
                pos.fullmoves,
                before.fullmoves,
                "fullmoves: {}",
                before.get_fen()
            );
        }
    }

    #[test]
    fn consistent() {
        let fens = [
            "startpos",
            "srp-r-s-2/p-1s-1r-1p-/4p-1/1RPw-3sr/2w-1RSP-/P-2WWS-2/R-S-S-1PR1 w 0 1",
            "1p-sr1p-r-/p-r-1w-w-s-p-/4r-1/3ss3/SPR-S-2R-/P-2WW2P-/R-2R-SPS- w 0 1",
            // Stacks, captures and the wise piece, which cannot be taken.
            "6/7/6/RRr-5/6/7/6 w 0 1",
            "6/7/6/W-r-5/6/7/6 w 0 1",
            "6/5r-r-/4r-W-/5r-r-/6/5w-w-/4w-R- w 0 1",
        ];

        for fen in fens {
            let mut pos = PijersiPosition::from_fen(fen);
            // Pijersi has ~180 moves per position, so depth 3 would be 6M nodes.
            check(&mut pos, 2);
        }
    }
}
