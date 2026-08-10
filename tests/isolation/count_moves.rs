#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, isolation::IsolationPosition};

    #[test]
    fn count_moves() {
        let tests = [
            "startpos",
            "8/8/8/8/8/pP6 b 1",
            "8/8/8/8/8/p.P5 b 1",
            "8/8/8/8/8/p..P4 b 1",
            "8/8/8/8/8/p...P3 b 1",
            "8/8/8/8/8/p....P2 b 1",
        ];

        for fen in tests {
            let pos = IsolationPosition::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }
}
