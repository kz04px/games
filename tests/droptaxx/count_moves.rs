#[cfg(test)]
mod tests {
    use games::{droptaxx::DroptaxxPosition, gamerules::GameRules};

    #[test]
    fn count_moves() {
        let tests = [
            "startpos",
            "x5o/7/7/7/7/7/o5x x 1",
            "x5o/7/7/7/7/7/o5x o 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x x 1",
            "x5o/7/2-1-2/7/2-1-2/7/o5x o 1",
        ];

        for fen in tests {
            let pos = DroptaxxPosition::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }
}
