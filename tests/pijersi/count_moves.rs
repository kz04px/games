#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, pijersi::PijersiPosition};

    #[test]
    fn count_moves() {
        let tests = [
            "startpos",
            "s-p-r-s-p-r-/p-r-s-wwr-s-/6/6/6/P-S-R-WWS-R-/R-P-S-R-P-S- w 0 1",
            "s-p-r-s-p-r-/p-r-s-wwr-s-/6/6/P-5/1S-R-WWS-R-/R-P-S-R-P-S- b 1 1",
        ];

        for fen in tests {
            let pos = PijersiPosition::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }
}
