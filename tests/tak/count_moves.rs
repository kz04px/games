#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn count_moves_3x3() {
        let tests = ["startpos", "x3/x3/x3 2 1"];

        for fen in tests {
            let pos = TakPosition::<3>::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }

    #[test]
    fn count_moves_6x6() {
        let tests = [
            "startpos",
            "x6/x6/x6/x6/x6/x6 2 1",
            "21,12,x4/x6/x6/x6/x6/x6 1 4",
            "211,122,x4/x6/x6/x6/x6/x6 1 6",
            "21C,12C,x4/x6/x6/x6/x6/x6 1 4",
            "21S,12S,x4/x6/x6/x6/x6/x6 1 4",
            "x,2,1,x3/x,111S,22,2S,222111,1/x2,2,1S,2,2S/x,121S,1,2S,2C,2/2,x,212S,2,2,1S/x,1221C,2,x,21,x 2 29",
        ];

        for fen in tests {
            let pos = TakPosition::<6>::from_fen(fen);
            assert_eq!(pos.legal_moves().len() as u64, pos.count_moves());
        }
    }
}
