#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules};

    const FENS: [&str; 7] = [
        "startpos",
        "x5o/7/2-1-2/7/2-1-2/7/o5x x 0 1",
        "x5o/7/2-1-2/3-3/2-1-2/7/o5x o 0 1",
        "x5o/7/3-3/2-1-2/3-3/7/o5x x 0 1",
        "7/7/7/7/ooooooo/ooooooo/xxxxxxx x 0 1",
        "7/7/7/2x1o2/7/7/7 x 0 1",
        "xxxxxxx/xxxxxxx/xxxxxxx/xxx1ooo/ooooooo/ooooooo/ooooooo o 0 1",
    ];

    fn check(pos: &mut AtaxxPosition<7, 7>, depth: i32) {
        assert_eq!(pos.get_hash(), pos.calculate_hash());

        if depth == 0 {
            return;
        }

        for mv in pos.legal_moves() {
            let before = pos.get_hash();
            let predicted = pos.predict_hash(&mv);

            pos.makemove(&mv);
            assert_eq!(pos.get_hash(), predicted);
            check(pos, depth - 1);
            pos.undomove(&mv);

            assert_eq!(pos.get_hash(), before);
        }
    }

    #[test]
    fn incremental_matches_from_scratch() {
        for fen in FENS {
            let mut pos = AtaxxPosition::<7, 7>::from_fen(fen);
            check(&mut pos, 4);
        }
    }

    #[test]
    fn set_when_fen_is_parsed() {
        for fen in FENS {
            let pos = AtaxxPosition::<7, 7>::from_fen(fen);
            assert_eq!(pos.get_hash(), pos.calculate_hash());
        }
    }

    #[test]
    fn side_to_move_changes_hash() {
        let x = AtaxxPosition::<7, 7>::from_fen("7/7/7/2x1o2/7/7/7 x 0 1");
        let o = AtaxxPosition::<7, 7>::from_fen("7/7/7/2x1o2/7/7/7 o 0 1");
        assert_ne!(x.get_hash(), o.get_hash());
    }

    #[test]
    fn transpositions_share_a_hash() {
        let mut a = AtaxxPosition::<7, 7>::startpos();
        let mut b = AtaxxPosition::<7, 7>::startpos();

        // Player1 plays the same two moves in a different order
        for mv in ["a6", "a2", "g2", "g6"] {
            let mv = a.parse_move(mv).unwrap();
            a.makemove(&mv);
        }
        for mv in ["g2", "a2", "a6", "g6"] {
            let mv = b.parse_move(mv).unwrap();
            b.makemove(&mv);
        }

        assert_eq!(a.pieces, b.pieces);
        assert_eq!(a.get_hash(), b.get_hash());
    }

    #[test]
    fn nullmove_toggles_side_only() {
        let mut pos = AtaxxPosition::<7, 7>::startpos();
        let before = pos.get_hash();
        pos.makenull();
        assert_ne!(pos.get_hash(), before);
        assert_eq!(pos.get_hash(), pos.calculate_hash());
        pos.undonull();
        assert_eq!(pos.get_hash(), before);
    }

    #[test]
    fn makemove_without_hash_restores_it_on_undo() {
        for fen in FENS {
            let mut pos = AtaxxPosition::<7, 7>::from_fen(fen);
            let before = pos.get_hash();
            for mv in pos.legal_moves() {
                pos.makemove_with::<false>(&mv);
                pos.undomove(&mv);
                assert_eq!(pos.get_hash(), before);
            }
        }
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "stale")]
    fn stale_hash_is_caught() {
        let mut pos = AtaxxPosition::<7, 7>::startpos();
        let mv = pos.legal_moves()[0];
        pos.makemove_with::<false>(&mv);
        let _ = pos.get_hash();
    }

    #[test]
    fn reset_hash_after_editing_fields() {
        let mut pos = AtaxxPosition::<7, 7>::startpos();
        pos.turn = !pos.turn;
        pos.reset_hash();
        assert_eq!(pos.get_hash(), pos.calculate_hash());
    }
}
