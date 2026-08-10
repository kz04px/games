#[cfg(test)]
mod tests {
    use games::{
        gamerules::GameRules,
        general::{bitboard::Bitboard, side::Side, square::Square},
        tak::{PieceType, TakPosition},
    };

    #[test]
    fn tak_accepts_legal_positions() {
        for fen in [
            "x6/x6/x6/x6/x6/x6 1 1",
            "2,1,x4/1C,2C,x4/x6/x6/x6/x6 1 3",
            "2,1,x4/1S,2S,x4/x6/x6/x6/x6 1 3",
        ] {
            assert_eq!(TakPosition::<6>::from_fen(fen).is_valid(), Ok(()), "{fen}");
        }
    }

    #[test]
    fn tak_rejects_negative_counts() {
        let base = TakPosition::<5>::startpos();

        let mut neg_pieces = base.clone();
        neg_pieces.num_pieces[Side::Player1] = -1;
        assert!(neg_pieces.is_valid().is_err(), "negative piece count accepted");

        let mut neg_caps = base.clone();
        neg_caps.num_caps[Side::Player1] = -1;
        assert!(neg_caps.is_valid().is_err(), "negative cap count accepted");
    }

    #[test]
    fn tak_rejects_overlapping_side_masks() {
        let mut pos = TakPosition::<5>::startpos();
        let sq = Square::from_coords(0, 0);

        pos.side_masks[Side::Player1] |= Bitboard::from_square(sq);
        pos.side_masks[Side::Player2] |= Bitboard::from_square(sq);

        assert!(pos.is_valid().is_err(), "p1 & p2 mask overlap accepted");
    }

    #[test]
    fn tak_rejects_overlapping_piece_type_masks() {
        let mut pos = TakPosition::<5>::startpos();
        let sq = Square::from_coords(0, 0);

        pos.piece_masks[PieceType::Flat as usize] |= Bitboard::from_square(sq);
        pos.piece_masks[PieceType::Standing as usize] |= Bitboard::from_square(sq);

        assert!(
            pos.is_valid().is_err(),
            "flat & standing mask overlap accepted"
        );
    }

    #[test]
    fn tak_rejects_a_standing_mask_without_a_stack() {
        let mut pos = TakPosition::<5>::startpos();
        let sq = Square::from_coords(0, 0);

        pos.piece_masks[PieceType::Standing as usize] |= Bitboard::from_square(sq);

        assert!(
            pos.is_valid().is_err(),
            "standing mask on an empty square accepted"
        );
    }
}
