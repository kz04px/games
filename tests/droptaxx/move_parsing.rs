#[cfg(test)]
mod test {
    use games::{
        droptaxx::{DroptaxxMove, DroptaxxPosition},
        gamerules::GameRules,
        general::square::Square,
    };

    #[test]
    fn move_parsing() {
        let tests = [
            ("a1", DroptaxxMove(Square::<7, 7>::from_coords(0, 0))),
            ("a7", DroptaxxMove(Square::<7, 7>::from_coords(0, 6))),
            ("g1", DroptaxxMove(Square::<7, 7>::from_coords(6, 0))),
            ("g7", DroptaxxMove(Square::<7, 7>::from_coords(6, 6))),
        ];

        let pos = DroptaxxPosition::default();
        for (movestr, expected) in tests {
            if let Ok(mv) = pos.parse_move(movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }
}
