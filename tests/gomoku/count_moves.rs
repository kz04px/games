#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, gomoku::GomokuPosition};

    #[test]
    fn count_moves() {
        let tests = [
            "startpos",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x14 o 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x13o x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x4o3x4o o 1",
        ];

        for fen in tests {
            let pos = GomokuPosition::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }
}
