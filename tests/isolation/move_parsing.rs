#[cfg(test)]
mod test {
    use games::{
        gamerules::GameRules,
        general::square::Square,
        isolation::{IsolationMove, IsolationPosition},
    };

    #[test]
    fn move_parsing() {
        let tests = [
            (
                "a1a6",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(0, 0),
                    remove: Square::<8, 6>::from_coords(0, 5),
                },
            ),
            (
                "a6h6",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(0, 5),
                    remove: Square::<8, 6>::from_coords(7, 5),
                },
            ),
            (
                "h6h1",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(7, 5),
                    remove: Square::<8, 6>::from_coords(7, 0),
                },
            ),
            (
                "h1a1",
                IsolationMove {
                    to: Square::<8, 6>::from_coords(7, 0),
                    remove: Square::<8, 6>::from_coords(0, 0),
                },
            ),
        ];

        let pos = IsolationPosition::default();
        for (movestr, expected) in tests {
            if let Ok(mv) = pos.parse_move(movestr) {
                assert_eq!(mv, expected);
            } else {
                panic!("Failed to parse move");
            }
        }
    }

    #[test]
    fn move_parsing_fail() {
        let tests = [
            "", "a1", "a1a7", "a7a1", "i1a1", "a1a2a3", "aa11", "0000", "test",
        ];

        let pos = IsolationPosition::default();
        for movestr in tests {
            assert!(pos.parse_move(movestr).is_err());
        }
    }
}
