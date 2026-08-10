use games::{
    gamerules::GameRules,
    pijersi::{PijersiMove, PijersiPosition},
};

fn mv_to_string(mv: &PijersiMove) -> String {
    match mv {
        PijersiMove::SoloMove(fr, to) => format!("{fr}-{to}"),
        PijersiMove::SoloStack(fr, to) => format!("{fr}={to}"),
        PijersiMove::SoloStackMove(fr, hex1, hex2) => format!("{fr}={hex1}-{hex2}"),
        PijersiMove::StackMove(fr, to) => format!("{fr}-{to}"),
        PijersiMove::StackMoveDestack(fr, hex1, hex2) => format!("{fr}-{hex1}-{hex2}"),
        PijersiMove::StackDestack(fr, to) => format!("{fr}={to}"),
        PijersiMove::StackStack(fr, to) => format!("{fr}={to}"),
        PijersiMove::StackStackMove(fr, hex1, hex2) => format!("{fr}={hex1}-{hex2}"),
    }
}

fn check_roundtrip(pos: &PijersiPosition, depth: i32) {
    if depth == 0 {
        return;
    }

    for mv in pos.legal_moves() {
        let movestr = mv_to_string(&mv);
        let parsed = pos
            .parse_movestr(&movestr)
            .unwrap_or_else(|e| panic!("failed to parse '{movestr}': {e}"));

        let mut expected = pos.clone();
        expected.makemove(&mv);

        let mut got = pos.clone();
        got.makemove(&parsed);

        assert_eq!(
            expected.get_fen(),
            got.get_fen(),
            "move string '{movestr}' produced a different position"
        );

        check_roundtrip(&got, depth - 1);
    }
}

#[test]
fn roundtrip_startpos() {
    check_roundtrip(&PijersiPosition::startpos(), 2);
}

#[test]
fn roundtrip_midgame() {
    let fens = [
        "s-p-r-s-p-r-/p-r-s-wwr-s-/6/6/6/P-S-R-WWS-R-/R-P-S-R-P-S- w 0 1",
        "s-p-r-s-p-r-/p-r-s-wwr-s-/6/6/P-5/1S-R-WWS-R-/R-P-S-R-P-S- b 1 1",
    ];

    for fen in fens {
        let pos = PijersiPosition::from_fen(fen);
        check_roundtrip(&pos, 1);
    }
}
