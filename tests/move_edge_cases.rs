use analyzer::r#move::Move;
use analyzer::components::Piece;

#[test]
fn test_move_parsing_basic_moves() {
    // Test basic pawn moves
    let pawn_move = Move::new("e4".to_string(), 0);
    assert_eq!(pawn_move.piece, Piece::Pawn);
    assert!(!pawn_move.is_capture);
    
    let pawn_move2 = Move::new("d5".to_string(), 1);
    assert_eq!(pawn_move2.piece, Piece::Pawn);
    assert!(!pawn_move2.is_capture);
}

#[test]
fn test_move_parsing_piece_moves() {
    // Test piece moves with explicit piece notation
    let knight_move = Move::new("Nf3".to_string(), 0);
    assert_eq!(knight_move.piece, Piece::Knight);
    assert!(!knight_move.is_capture);
    
    let bishop_move = Move::new("Bc4".to_string(), 2);
    assert_eq!(bishop_move.piece, Piece::Bishop);
    assert!(!bishop_move.is_capture);
    
    let queen_move = Move::new("Qd4".to_string(), 4);
    assert_eq!(queen_move.piece, Piece::Queen);
    assert!(!queen_move.is_capture);
}

#[test]
fn test_move_parsing_captures() {
    // Test capture notation
    let pawn_capture = Move::new("exd5".to_string(), 0);
    assert_eq!(pawn_capture.piece, Piece::Pawn);
    assert!(pawn_capture.is_capture);
    
    let piece_capture = Move::new("Nxe5".to_string(), 2);
    assert_eq!(piece_capture.piece, Piece::Knight);
    assert!(piece_capture.is_capture);
    
    let queen_capture = Move::new("Qxd8".to_string(), 4);
    assert_eq!(queen_capture.piece, Piece::Queen);
    assert!(queen_capture.is_capture);
}

#[test]
fn test_move_parsing_castling() {
    // Test castling notation
    let kingside_castle = Move::new("O-O".to_string(), 0);
    assert_eq!(kingside_castle.piece, Piece::King);
    assert!(kingside_castle.castling.is_some());
    
    let queenside_castle = Move::new("O-O-O".to_string(), 2);
    assert_eq!(queenside_castle.piece, Piece::King);
    assert!(queenside_castle.castling.is_some());
}

#[test]
fn test_move_parsing_with_check() {
    // Test moves with check notation
    let check_move = Move::new("Nf3+".to_string(), 0);
    assert_eq!(check_move.piece, Piece::Knight);
    // The + should be stripped from the move notation
    
    let checkmate_move = Move::new("Qd8#".to_string(), 2);
    assert_eq!(checkmate_move.piece, Piece::Queen);
    // The # should be stripped from the move notation
}

#[test]
fn test_move_parsing_promotion() {
    // Test pawn promotion notation
    let promotion = Move::new("e8=Q".to_string(), 0);
    assert_eq!(promotion.piece, Piece::Pawn);
    assert!(promotion.promotion.is_some());
    
    let promotion_with_capture = Move::new("dxe8=R".to_string(), 2);
    assert_eq!(promotion_with_capture.piece, Piece::Pawn);
    assert!(promotion_with_capture.is_capture);
    assert!(promotion_with_capture.promotion.is_some());
}

#[test]
fn test_move_parsing_disambiguation() {
    // Test disambiguating moves (when multiple pieces can move to same square)
    let file_disambig = Move::new("Nbd2".to_string(), 0);
    assert_eq!(file_disambig.piece, Piece::Knight);
    assert!(file_disambig.source.0.is_some()); // File disambiguation
    
    let rank_disambig = Move::new("R1a3".to_string(), 2);
    assert_eq!(rank_disambig.piece, Piece::Rook);
    assert!(rank_disambig.source.1.is_some()); // Rank disambiguation
    
    let full_disambig = Move::new("Qh4e1".to_string(), 4);
    assert_eq!(full_disambig.piece, Piece::Queen);
    // Both file and rank specified
}

#[test]
fn test_move_parsing_edge_cases() {
    // Test various edge cases that could break parsing
    
    // Very short notation
    let short_move = Move::new("e4".to_string(), 0);
    assert_eq!(short_move.piece, Piece::Pawn);
    
    // Capture with check
    let capture_check = Move::new("Rxd8+".to_string(), 0);
    assert_eq!(capture_check.piece, Piece::Rook);
    assert!(capture_check.is_capture);
    
    // Promotion with check
    let promotion_check = Move::new("e8=Q+".to_string(), 2);
    assert_eq!(promotion_check.piece, Piece::Pawn);
    assert!(promotion_check.promotion.is_some());
    
    // Complex capture with disambiguation
    let complex_move = Move::new("Ngxf6+".to_string(), 4);
    assert_eq!(complex_move.piece, Piece::Knight);
    assert!(complex_move.is_capture);
    assert!(complex_move.source.0.is_some()); // File disambiguation
}

#[test]
fn test_move_color_determination() {
    // Test that move color is determined correctly by move index
    let white_move = Move::new("e4".to_string(), 0);
    assert!(matches!(white_move.color(), analyzer::color::Color::White));
    
    let black_move = Move::new("e5".to_string(), 1);
    assert!(matches!(black_move.color(), analyzer::color::Color::Black));
    
    let white_move2 = Move::new("Nf3".to_string(), 2);
    assert!(matches!(white_move2.color(), analyzer::color::Color::White));
    
    let black_move2 = Move::new("Nc6".to_string(), 3);
    assert!(matches!(black_move2.color(), analyzer::color::Color::Black));
}