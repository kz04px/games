#[cfg(test)]
mod tests {
    use games::{droptaxx::DroptaxxPosition, gamerules::GameRules};

    #[test]
    fn round_trips() {
        let fens = ["7/7/7/7/7/7/7 x 1"];

        for fen in fens {
            let mut pos = DroptaxxPosition::from_fen(fen);
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
