#[cfg(test)]
mod test {
    use games::{
        gamerules::GameRules,
        general::square::Square,
        gomoku::{GomokuMove, GomokuPosition},
    };

    #[test]
    fn move_parsing() {
        let tests = [
            ("a1", GomokuMove(Square::<15, 15>::from_coords(0, 0))),
            ("a15", GomokuMove(Square::<15, 15>::from_coords(0, 14))),
            ("o1", GomokuMove(Square::<15, 15>::from_coords(14, 0))),
            ("o15", GomokuMove(Square::<15, 15>::from_coords(14, 14))),
        ];

        let pos = GomokuPosition::default();
        for (movestr, expected) in tests {
            if let Ok(mv) = pos.parse_move(movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move {}", movestr);
            }
        }
    }
}
