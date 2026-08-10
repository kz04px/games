#[cfg(test)]
mod tests {
    use games::{gamerules::GameRules, tak::TakPosition};

    #[test]
    fn round_trips() {
        let fens = ["x6/x6/x6/x6/x6/1,2,x4 1 2"];

        for fen in fens {
            let mut pos = TakPosition::<6>::from_fen(fen);
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

            // Two passes are a whole move, so the position differs only in its clock.
            pos.makenull();
            pos.makenull();
            pos.undonull();
            pos.undonull();
            assert_eq!(pos.get_fen(), fen, "state not restored twice: {fen}");
        }
    }
}
