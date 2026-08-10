use games::{
    gamerules::GameRules,
    tak::{Dir, PieceType, TakMove, TakPosition},
};

fn dir_char(dir: Dir) -> char {
    match dir {
        Dir::Up => '+',
        Dir::Down => '-',
        Dir::Left => '<',
        Dir::Right => '>',
    }
}

fn decode_drops(height: u8, mut spread: u8) -> Vec<u8> {
    while spread & 1 == 0 {
        spread >>= 1;
    }

    let mut drops = vec![];
    let mut cur = 0u8;
    for i in 0..height {
        if i > 0 && (spread >> i) & 1 == 1 {
            drops.push(cur);
            cur = 0;
        }
        cur += 1;
    }
    drops.push(cur);
    drops
}

fn mv_to_string<const S: usize>(mv: &TakMove<S>) -> String {
    match mv {
        TakMove::Drop(sq, PieceType::Flat) => sq.to_string(),
        TakMove::Drop(sq, PieceType::Standing) => format!("S{sq}"),
        TakMove::Drop(sq, PieceType::Cap) => format!("C{sq}"),
        &TakMove::Spread(sq, dir, height, spread, is_crush) => {
            let drops = decode_drops(height, spread);
            let mut out = format!("{height}{sq}{}", dir_char(dir));
            for d in drops {
                out += &d.to_string();
            }
            if is_crush {
                out += "*";
            }
            out
        }
    }
}

fn check_roundtrip<const SIZE: usize>(pos: &TakPosition<SIZE>, depth: i32) {
    if depth == 0 {
        return;
    }

    for mv in pos.legal_moves() {
        let movestr = mv_to_string(&mv);
        let parsed = pos
            .parse_move(&movestr)
            .unwrap_or_else(|e| panic!("failed to parse '{movestr}': {e}"));

        assert_eq!(
            parsed, mv,
            "move string '{movestr}' parsed to a different move"
        );

        let unmarked = movestr.trim_end_matches('*');
        assert_eq!(
            pos.parse_move(unmarked),
            Ok(mv),
            "move string '{unmarked}' parsed to a different move"
        );

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
    check_roundtrip(&TakPosition::<3>::startpos(), 2);
    check_roundtrip(&TakPosition::<4>::startpos(), 2);
    check_roundtrip(&TakPosition::<5>::startpos(), 2);
    check_roundtrip(&TakPosition::<6>::startpos(), 2);
}

#[test]
fn roundtrip_midgame_stacks() {
    let fens = [
        "21,12,x4/x6/x6/x6/x6/x6 1 4",
        "211,122,x4/x6/x6/x6/x6/x6 1 6",
        "21C,12C,x4/x6/x6/x6/x6/x6 1 4",
        "21S,12S,x4/x6/x6/x6/x6/x6 1 4",
        "x,2,1,x3/x,111S,22,2S,222111,1/x2,2,1S,2,2S/x,121S,1,2S,2C,2/2,x,212S,2,2,1S/x,1221C,2,x,21,x 2 29",
    ];

    for fen in fens {
        let pos = TakPosition::<6>::from_fen(fen);
        check_roundtrip(&pos, 1);
    }
}

#[test]
fn placements() {
    let pos = TakPosition::<5>::default();

    assert!(matches!(
        pos.parse_move("c3").unwrap(),
        TakMove::Drop(_, PieceType::Flat)
    ));
    assert!(matches!(
        pos.parse_move("Fc3").unwrap(),
        TakMove::Drop(_, PieceType::Flat)
    ));
    assert!(matches!(
        pos.parse_move("Sc3").unwrap(),
        TakMove::Drop(_, PieceType::Standing)
    ));
    assert!(matches!(
        pos.parse_move("Cc3").unwrap(),
        TakMove::Drop(_, PieceType::Cap)
    ));
}

#[test]
fn spreads() {
    let pos = TakPosition::<5>::default();

    match pos.parse_move("3c3>21").unwrap() {
        TakMove::Spread(sq, Dir::Right, height, spread, is_crush) => {
            assert_eq!(sq.to_string(), "c3");
            assert_eq!(height, 3);
            assert_eq!(decode_drops(height, spread), vec![2, 1]);
            assert!(!is_crush);
        }
        other => panic!("expected a spread, got {other:?}"),
    }

    match pos.parse_move("c3>").unwrap() {
        TakMove::Spread(_, Dir::Right, height, spread, is_crush) => {
            assert_eq!(height, 1);
            assert_eq!(decode_drops(height, spread), vec![1]);
            assert!(!is_crush);
        }
        other => panic!("expected a spread, got {other:?}"),
    }

    match pos.parse_move("2c3+11*").unwrap() {
        TakMove::Spread(_, Dir::Up, height, spread, is_crush) => {
            assert_eq!(height, 2);
            assert_eq!(decode_drops(height, spread), vec![1, 1]);
            assert!(is_crush);
        }
        other => panic!("expected a spread, got {other:?}"),
    }
}

#[test]
fn failures() {
    let tests = [
        "",
        "z9",
        "3c3>2",
        "3c3>211",
        "c3>0",
        "9c3>",
        "c3?",
        "Xc3",
    ];

    let pos = TakPosition::<5>::default();
    for movestr in tests {
        assert!(
            pos.parse_move(movestr).is_err(),
            "expected '{movestr}' to fail"
        );
    }
}

#[test]
fn unmarked_crush() {
    let pos = TakPosition::<6>::from_fen("x6/x6/x6/x6/x6/1C,2S,x4 1 3");
    let mv = pos.parse_move("a1>").unwrap();
    assert!(matches!(mv, TakMove::Spread(_, Dir::Right, 1, _, true)));
    assert!(pos.legal_moves().contains(&mv));
    assert_eq!(pos.parse_move("a1>*"), Ok(mv));
}
