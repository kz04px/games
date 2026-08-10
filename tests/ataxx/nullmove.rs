#[cfg(test)]
mod tests {
    use games::{ataxx::AtaxxPosition, gamerules::GameRules};

    #[test]
    fn round_trips() {
        let fens = ["7/7/7/7/7/7/7 x 0 1", "x5o/7/2-1-2/7/2-1-2/7/o5x o 3 7"];

        for fen in fens {
            let mut pos = AtaxxPosition::<7, 7>::from_fen(fen);
            let turn = pos.get_turn();

            pos.makenull();
            assert_ne!(
                pos.get_turn(),
                turn,
                "null move did not pass the turn: {fen}"
            );

            pos.undonull();
            assert_eq!(pos.is_valid(), Ok(()), "invalid after undonull: {fen}");
            assert_eq!(pos.get_turn(), turn, "turn not restored: {fen}");
            assert_eq!(pos.get_fen(), fen, "state not restored: {fen}");

            pos.makenull();
            pos.makenull();
            pos.undonull();
            pos.undonull();
            assert_eq!(pos.get_fen(), fen, "state not restored twice: {fen}");
        }
    }
}
