#[cfg(test)]
mod test {
    use games::{
        ataxx::{AtaxxMove, AtaxxPosition},
        gamerules::GameRules,
        general::square::Square,
    };

    #[test]
    fn move_parsing() {
        let tests = [
            ("0000", AtaxxMove::Pass),
            ("a1", AtaxxMove::Single(Square::<7, 7>::from_coords(0, 0))),
            ("a7", AtaxxMove::Single(Square::<7, 7>::from_coords(0, 6))),
            ("g1", AtaxxMove::Single(Square::<7, 7>::from_coords(6, 0))),
            ("g7", AtaxxMove::Single(Square::<7, 7>::from_coords(6, 6))),
            (
                "a1c3",
                AtaxxMove::Double(
                    Square::<7, 7>::from_coords(0, 0),
                    Square::<7, 7>::from_coords(2, 2),
                ),
            ),
            (
                "g7e5",
                AtaxxMove::Double(
                    Square::<7, 7>::from_coords(6, 6),
                    Square::<7, 7>::from_coords(4, 4),
                ),
            ),
        ];

        let pos = AtaxxPosition::<7, 7>::default();
        for (movestr, expected) in tests {
            if let Ok(mv) = pos.parse_move(movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }
}
