#[cfg(test)]
mod tests {
    use games::{
        connect4::Connect4Position,
        gamerules::GameRules,
        general::{bitboard::Bitboard, side::Side, square::Square},
    };

    #[test]
    fn connect4_accepts_legal_positions() {
        for fen in [
            "startpos",
            "7/7/7/7/7/rrrr3 r 1",
            "7/yyrryyr/rryyrry/yyrryyr/rryyrry/yyrryyr r 1",
        ] {
            assert_eq!(Connect4Position::from_fen(fen).is_valid(), Ok(()), "{fen}");
        }
    }

    #[test]
    fn connect4_rejects_overlapping_masks() {
        let base = Connect4Position::default();
        let sq = Square::<7, 6>::from_coords(3, 0);

        let mut overlap = base.clone();
        overlap.pieces[Side::Player1] |= Bitboard::from_square(sq);
        overlap.pieces[Side::Player2] |= Bitboard::from_square(sq);
        assert!(overlap.is_valid().is_err(), "red & yellow overlap accepted");
    }

    #[test]
    fn connect4_rejects_floating_pieces() {
        let base = Connect4Position::default();
        let sq = Square::<7, 6>::from_coords(3, 3);

        let mut floating_red = base.clone();
        floating_red.pieces[Side::Player1] |= Bitboard::from_square(sq);
        assert!(floating_red.is_valid().is_err(), "floating red accepted");

        let mut floating_yellow = base.clone();
        floating_yellow.pieces[Side::Player2] |= Bitboard::from_square(sq);
        assert!(
            floating_yellow.is_valid().is_err(),
            "floating yellow accepted"
        );
    }
}
