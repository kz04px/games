#[cfg(test)]
mod tests {
    use games::{
        gamerules::GameRules,
        general::{bitboard::Bitboard, side::Side, square::Square},
        isolation::IsolationPosition,
    };

    #[test]
    fn isolation_accepts_legal_positions() {
        for fen in ["startpos", "8/8/8/8/8/p......P b", "8/8/8/8/8/p......P r"] {
            assert_eq!(IsolationPosition::from_fen(fen).is_valid(), Ok(()), "{fen}");
        }
    }

    #[test]
    fn isolation_rejects_overlapping_pieces() {
        let base = IsolationPosition::from_fen("startpos");

        let mut overlap = base.clone();
        let red = base.get_red();
        overlap.pieces[Side::Player1] |= red;
        assert!(overlap.is_valid().is_err(), "black & red overlap accepted");
    }

    #[test]
    fn isolation_rejects_the_wrong_piece_count() {
        let base = IsolationPosition::from_fen("startpos");

        let mut no_black = base.clone();
        no_black.pieces[Side::Player1] = Bitboard::default();
        assert!(no_black.is_valid().is_err(), "zero black pieces accepted");

        let mut two_black = base.clone();
        two_black.pieces[Side::Player1] |= Bitboard::from_square(Square::from_coords(3, 3));
        assert!(two_black.is_valid().is_err(), "two black pieces accepted");
    }

    #[test]
    fn isolation_rejects_fullmoves_off_by_one_from_the_start_square() {
        let mut pos = IsolationPosition::from_fen("startpos");
        pos.fullmoves = 2;
        assert!(pos.is_valid().is_err());
    }

    #[test]
    fn isolation_rejects_a_piece_off_a_tile() {
        let mut pos = IsolationPosition::from_fen("startpos");
        let off_start = Square::from_coords(3, 3);

        pos.pieces[Side::Player1] = Bitboard::from_square(off_start);
        pos.tiles &= !Bitboard::from_square(off_start);

        assert!(pos.is_valid().is_err());
    }
}
