#[cfg(test)]
mod tests {
    use games::{
        chess::{ChessPiece, ChessPosition},
        gamerules::GameRules,
    };

    #[test]
    fn get_piece_matches_the_named_getters() {
        let pos = ChessPosition::startpos();
        let board = &pos.board;
        assert_eq!(board.get_piece(ChessPiece::Pawn), board.get_pawns());
        assert_eq!(board.get_piece(ChessPiece::Knight), board.get_knights());
        assert_eq!(board.get_piece(ChessPiece::Bishop), board.get_bishops());
        assert_eq!(board.get_piece(ChessPiece::Rook), board.get_rooks());
        assert_eq!(board.get_piece(ChessPiece::Queen), board.get_queens());
        assert_eq!(board.get_piece(ChessPiece::King), board.get_kings());
    }
}
