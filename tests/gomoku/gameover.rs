#[cfg(test)]
mod tests {
    use games::{
        gamerules::{GameResult, GameRules},
        gomoku::GomokuPosition,
    };

    #[test]
    fn no_moves_when_over() {
        let fens = [
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxxx10 o 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/ooooo10 x 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxxxx9 o 1",
        ];

        for fen in fens {
            let pos = GomokuPosition::from_fen(fen);
            assert!(pos.is_gameover(), "expected a finished game: {fen}");
            assert_eq!(pos.legal_moves().len(), 0, "moves after the end: {fen}");
            assert_eq!(pos.count_moves(), 0, "count_moves disagrees: {fen}");
        }
    }

    #[test]
    fn moves_when_not_over() {
        let fens = [
            "startpos",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/x1xxxx9 o 1",
            "15/15/15/15/15/15/15/15/15/15/15/15/15/15/xxxx11 o 1",
        ];

        for fen in fens {
            let pos = GomokuPosition::from_fen(fen);
            assert!(!pos.is_gameover(), "expected an unfinished game: {fen}");
            assert!(pos.count_moves() > 0, "no moves in a live game: {fen}");
            assert_eq!(
                pos.legal_moves().len() as u64,
                pos.count_moves(),
                "count_moves disagrees with legal_moves: {fen}"
            );
        }
    }

    #[test]
    fn gomoku_full_board_is_a_draw() {
        // Rows of alternating pairs, offset each rank, so no five ever line up.
        let row = |offset: usize| {
            let mut s = String::new();
            for i in 0..15 {
                s.push(if (i + offset) % 4 < 2 { 'x' } else { 'o' });
            }
            s
        };
        let board = (0..15).map(|y| row(y * 2)).collect::<Vec<_>>().join("/");

        let pos = GomokuPosition::from_fen(&format!("{board} x 1"));
        assert_eq!(pos.get_result(), Some(GameResult::Draw), "{board}");
        assert!(pos.is_gameover(), "{board}");
        assert_eq!(pos.legal_moves().len(), 0, "{board}");
    }
}
