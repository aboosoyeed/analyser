use analyzer::board::Board;
use analyzer::components::Piece;
use analyzer::r#move::Move;
use analyzer::error::ChessError;
use analyzer::color::Color;

#[test]
fn test_initial_piece_lookup() {
    let board = Board::init();
    
    // Test white pieces on starting squares
    assert_eq!(board.get_piece_at_index(0).unwrap(), Piece::Rook);   // a1
    assert_eq!(board.get_piece_at_index(1).unwrap(), Piece::Knight); // b1
    assert_eq!(board.get_piece_at_index(2).unwrap(), Piece::Bishop); // c1
    assert_eq!(board.get_piece_at_index(3).unwrap(), Piece::Queen);  // d1
    assert_eq!(board.get_piece_at_index(4).unwrap(), Piece::King);   // e1
    assert_eq!(board.get_piece_at_index(5).unwrap(), Piece::Bishop); // f1
    assert_eq!(board.get_piece_at_index(6).unwrap(), Piece::Knight); // g1
    assert_eq!(board.get_piece_at_index(7).unwrap(), Piece::Rook);   // h1
    
    // Test white pawns
    for i in 8..16 {
        assert_eq!(board.get_piece_at_index(i).unwrap(), Piece::Pawn);
    }
    
    // Test black pieces on starting squares
    assert_eq!(board.get_piece_at_index(56).unwrap(), Piece::Rook);   // a8
    assert_eq!(board.get_piece_at_index(57).unwrap(), Piece::Knight); // b8
    assert_eq!(board.get_piece_at_index(58).unwrap(), Piece::Bishop); // c8
    assert_eq!(board.get_piece_at_index(59).unwrap(), Piece::Queen);  // d8
    assert_eq!(board.get_piece_at_index(60).unwrap(), Piece::King);   // e8
    assert_eq!(board.get_piece_at_index(61).unwrap(), Piece::Bishop); // f8
    assert_eq!(board.get_piece_at_index(62).unwrap(), Piece::Knight); // g8
    assert_eq!(board.get_piece_at_index(63).unwrap(), Piece::Rook);   // h8
    
    // Test black pawns
    for i in 48..56 {
        assert_eq!(board.get_piece_at_index(i).unwrap(), Piece::Pawn);
    }
}

#[test]
fn test_empty_square_lookup() {
    let board = Board::init();
    
    // Test empty squares in the middle of the board
    for i in 16..48 {
        match board.get_piece_at_index(i) {
            Err(ChessError::PieceNotFound { square }) => {
                assert_eq!(square, i);
            },
            _ => panic!("Expected PieceNotFound error for empty square {}", i),
        }
    }
}

#[test]
fn test_invalid_square_lookup() {
    let board = Board::init();
    
    // Test invalid square indices
    match board.get_piece_at_index(64) {
        Err(ChessError::InvalidSquare { index }) => {
            assert_eq!(index, 64);
        },
        _ => panic!("Expected InvalidSquare error for index 64"),
    }
    
    match board.get_piece_at_index(255) {
        Err(ChessError::InvalidSquare { index }) => {
            assert_eq!(index, 255);
        },
        _ => panic!("Expected InvalidSquare error for index 255"),
    }
}

#[test]
fn test_lookup_after_simple_move() {
    let mut board = Board::init();
    
    // Apply e4 (pawn from e2 to e4)
    let move_e4 = Move::new("e4".to_string(), 0);
    board.apply_move(&move_e4);
    
    // e2 should now be empty
    match board.get_piece_at_index(12) { // e2
        Err(ChessError::PieceNotFound { square }) => {
            assert_eq!(square, 12);
        },
        _ => panic!("Expected e2 to be empty after e4"),
    }
    
    // e4 should now have a pawn
    assert_eq!(board.get_piece_at_index(28).unwrap(), Piece::Pawn); // e4
}

#[test]
fn test_lookup_after_multiple_moves() {
    let mut board = Board::init();
    
    // Apply several moves: e4, d6, Nf3
    let moves = vec![
        Move::new("e4".to_string(), 0),
        Move::new("d6".to_string(), 1), 
        Move::new("Nf3".to_string(), 2),
    ];
    
    for mov in moves {
        board.apply_move(&mov);
    }
    
    // Check final positions
    assert_eq!(board.get_piece_at_index(28).unwrap(), Piece::Pawn);   // e4 - white pawn
    assert_eq!(board.get_piece_at_index(43).unwrap(), Piece::Pawn);   // d6 - black pawn
    assert_eq!(board.get_piece_at_index(21).unwrap(), Piece::Knight); // f3 - white knight
    
    // Check vacated squares
    match board.get_piece_at_index(12) { // e2
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e2 should be empty"),
    }
    match board.get_piece_at_index(51) { // d7
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("d7 should be empty"),
    }
    match board.get_piece_at_index(6) { // g1
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("g1 should be empty"),
    }
}

#[test]
fn test_lookup_after_capture() {
    let mut board = Board::init();
    
    // Set up a position where a capture can happen
    // e4, d5, exd5 (pawn captures pawn)
    let moves = vec![
        Move::new("e4".to_string(), 0),
        Move::new("d5".to_string(), 1),
        Move::new("exd5".to_string(), 2),
    ];
    
    for mov in moves {
        board.apply_move(&mov);
    }
    
    // d5 should have white pawn (captured black pawn)
    assert_eq!(board.get_piece_at_index(35).unwrap(), Piece::Pawn); // d5
    
    // e4 should be empty (pawn moved from there)
    match board.get_piece_at_index(28) { // e4
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e4 should be empty after pawn moved"),
    }
}

#[test]
fn test_lookup_after_castling() {
    let mut board = Board::init();
    
    // Set up kingside castling: move pieces out of the way first
    let prep_moves = vec![
        Move::new("e4".to_string(), 0),
        Move::new("e5".to_string(), 1),
        Move::new("Nf3".to_string(), 2),
        Move::new("Nf6".to_string(), 3),
        Move::new("Bc4".to_string(), 4),
        Move::new("Bc5".to_string(), 5),
    ];
    
    for mov in prep_moves {
        board.apply_move(&mov);
    }
    
    // Now castle kingside
    let castle = Move::new("O-O".to_string(), 6);
    board.apply_move(&castle);
    
    // King should be on g1
    assert_eq!(board.get_piece_at_index(6).unwrap(), Piece::King); // g1
    
    // Rook should be on f1
    assert_eq!(board.get_piece_at_index(5).unwrap(), Piece::Rook); // f1
    
    // e1 and h1 should be empty
    match board.get_piece_at_index(4) { // e1
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e1 should be empty after castling"),
    }
    match board.get_piece_at_index(7) { // h1
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("h1 should be empty after castling"),
    }
}

#[test]
fn test_lookup_consistency_with_bitboards() {
    let mut board = Board::init();
    
    // Apply several random moves
    let moves = vec![
        Move::new("e4".to_string(), 0),
        Move::new("c5".to_string(), 1),
        Move::new("d3".to_string(), 2),
        Move::new("Nc6".to_string(), 3),
        Move::new("Nf3".to_string(), 4),
    ];
    
    for mov in moves {
        board.apply_move(&mov);
    }
    
    // Verify lookup table matches bitboard state for all squares
    for square in 0..64 {
        let lookup_result = board.get_piece_at_index(square);
        
        // Check if square is occupied according to bitboards
        let is_occupied = board.occupied.get_bit(square);
        
        match (is_occupied, &lookup_result) {
            (true, Ok(piece)) => {
                // Verify the piece type matches what's in the bitboards
                let piece_board = board.by_piece.get(*piece);
                assert!(piece_board.get_bit(square), 
                    "Lookup table says {} is on square {} but bitboard disagrees", 
                    piece.to_char(analyzer::color::Color::White), square);
            },
            (false, Err(ChessError::PieceNotFound { .. })) => {
                // This is correct - empty square
            },
            (true, Err(_)) => {
                panic!("Square {} is occupied in bitboard but lookup table says it's empty", square);
            },
            (false, Ok(piece)) => {
                panic!("Square {} is empty in bitboard but lookup table says it has {}", 
                    square, piece.to_char(analyzer::color::Color::White));
            },
            (false, Err(_)) => {
                // Any other error on empty square should not occur for valid squares
                panic!("Unexpected error on empty square {}: {:?}", square, lookup_result);
            },
        }
    }
}

#[test]
fn test_lookup_table_rebuilds_correctly() {
    let mut board = Board::init();
    
    // Test that multiple consecutive moves maintain lookup table consistency
    let moves = vec![
        Move::new("e4".to_string(), 0),
        Move::new("e5".to_string(), 1),
        Move::new("Nf3".to_string(), 2),
        Move::new("Nc6".to_string(), 3),
        Move::new("Bc4".to_string(), 4),
        Move::new("f6".to_string(), 5),
    ];
    
    for (i, mov) in moves.iter().enumerate() {
        board.apply_move(mov);
        
        // Verify lookup table consistency after each move
        for square in 0..64 {
            let lookup_result = board.get_piece_at_index(square);
            let is_occupied = board.occupied.get_bit(square);
            
            match (is_occupied, lookup_result) {
                (true, Ok(_)) => {}, // Correct
                (false, Err(ChessError::PieceNotFound { .. })) => {}, // Correct
                _ => panic!("Lookup table inconsistency after move {}: {:?}", i, mov),
            }
        }
    }
    
    // Verify specific piece positions after all moves
    assert_eq!(board.get_piece_at_index(28).unwrap(), Piece::Pawn);   // e4 - white pawn
    assert_eq!(board.get_piece_at_index(36).unwrap(), Piece::Pawn);   // e5 - black pawn  
    assert_eq!(board.get_piece_at_index(21).unwrap(), Piece::Knight); // f3 - white knight
    assert_eq!(board.get_piece_at_index(42).unwrap(), Piece::Knight); // c6 - black knight
    assert_eq!(board.get_piece_at_index(26).unwrap(), Piece::Bishop); // c4 - white bishop
    assert_eq!(board.get_piece_at_index(45).unwrap(), Piece::Pawn);   // f6 - black pawn
}

#[test]
fn test_lookup_performance_vs_linear_search() {
    use std::time::Instant;
    
    let board = Board::init();
    
    // Time the optimized lookup (should be very fast)
    let start = Instant::now();
    for _ in 0..10000 {
        for square in 0..64 {
            let _ = board.get_piece_at_index(square);
        }
    }
    let optimized_duration = start.elapsed();
    
    // The test mainly verifies that the optimized version doesn't crash
    // and completes in reasonable time (< 1 second for 640,000 lookups)
    assert!(optimized_duration.as_millis() < 1000, 
        "Piece lookup taking too long: {:?}", optimized_duration);
}

// ================================
// CORE BOARD::APPLY_MOVE() TESTS
// ================================

#[test]
fn test_apply_move_pawn_advance() {
    let mut board = Board::init();
    let initial_half_moves = board.half_move_count;
    let initial_full_moves = board.full_move_count;
    
    // Test e4 (white pawn double move)
    let move_e4 = Move::new("e4".to_string(), 0);
    let source = board.apply_move(&move_e4);
    
    // Verify source square returned
    assert_eq!(source, Some(12)); // e2 square index
    
    // Verify piece moved correctly
    assert_eq!(board.get_piece_at_index(28).unwrap(), Piece::Pawn); // e4
    match board.get_piece_at_index(12) { // e2 should be empty
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e2 should be empty after pawn move"),
    }
    
    // Verify move counters (pawn move resets half-move clock)
    assert_eq!(board.half_move_count, 0);
    assert_eq!(board.full_move_count, initial_full_moves);
    
    // Test black pawn response e5
    let move_e5 = Move::new("e5".to_string(), 1);
    board.apply_move(&move_e5);
    
    // Verify full move increments after black move
    assert_eq!(board.full_move_count, initial_full_moves + 1);
}

#[test]
fn test_apply_move_capture() {
    let mut board = Board::init();
    
    // Set up position for capture: e4, d5, exd5
    board.apply_move(&Move::new("e4".to_string(), 0));
    board.apply_move(&Move::new("d5".to_string(), 1));
    
    let initial_half_moves = board.half_move_count;
    
    // Capture move
    let capture_move = Move::new("exd5".to_string(), 2);
    let source = board.apply_move(&capture_move);
    
    // Verify capture completed
    assert!(source.is_some());
    assert_eq!(board.get_piece_at_index(35).unwrap(), Piece::Pawn); // d5 has white pawn
    
    // Verify source square is empty
    match board.get_piece_at_index(28) { // e4 should be empty
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e4 should be empty after pawn capture"),
    }
    
    // Capture should reset half-move clock
    assert_eq!(board.half_move_count, 0);
}

#[test]
fn test_apply_move_knight() {
    let mut board = Board::init();
    let initial_half_moves = board.half_move_count;
    
    // Knight move Nf3
    let knight_move = Move::new("Nf3".to_string(), 0);
    let source = board.apply_move(&knight_move);
    
    // Verify knight moved
    assert_eq!(source, Some(6)); // g1 source
    assert_eq!(board.get_piece_at_index(21).unwrap(), Piece::Knight); // f3
    match board.get_piece_at_index(6) { // g1 should be empty
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("g1 should be empty after knight move"),
    }
    
    // Non-pawn, non-capture move should increment half-move clock
    assert_eq!(board.half_move_count, initial_half_moves + 1);
}

#[test]
fn test_apply_move_castling_kingside() {
    let mut board = Board::init();
    
    // Clear path for castling
    board.apply_move(&Move::new("e4".to_string(), 0));
    board.apply_move(&Move::new("e5".to_string(), 1));
    board.apply_move(&Move::new("Nf3".to_string(), 2));
    board.apply_move(&Move::new("Nf6".to_string(), 3));
    board.apply_move(&Move::new("Bc4".to_string(), 4));
    board.apply_move(&Move::new("Bc5".to_string(), 5));
    
    let initial_castling_rights = board.castling_rights;
    
    // Castle kingside
    let castle_move = Move::new("O-O".to_string(), 6);
    let source = board.apply_move(&castle_move);
    
    // Verify king and rook positions
    assert_eq!(board.get_piece_at_index(6).unwrap(), Piece::King); // g1
    assert_eq!(board.get_piece_at_index(5).unwrap(), Piece::Rook); // f1
    
    // Verify old squares are empty
    match board.get_piece_at_index(4) { // e1
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e1 should be empty after castling"),
    }
    match board.get_piece_at_index(7) { // h1
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("h1 should be empty after castling"),
    }
    
    // Verify castling rights removed for white
    assert_ne!(board.castling_rights, initial_castling_rights);
    // Check that some castling rights were removed (exact bits may vary by implementation)
    assert!(board.castling_rights < initial_castling_rights);
}

#[test]
fn test_apply_move_basic_functionality() {
    let mut board = Board::init();
    
    // Test that apply_move returns source square and updates board state
    let move_e4 = Move::new("e4".to_string(), 0);
    let source = board.apply_move(&move_e4);
    
    // Should return source square
    assert!(source.is_some());
    
    // Should update piece positions
    assert_eq!(board.get_piece_at_index(28).unwrap(), Piece::Pawn); // e4
    
    // Should maintain move counters properly
    assert_eq!(board.half_move_count, 0); // Pawn move resets
    
    // Test move sequence maintains board integrity
    board.apply_move(&Move::new("d5".to_string(), 1));
    board.apply_move(&Move::new("exd5".to_string(), 2));
    
    // Capture should work correctly
    assert_eq!(board.get_piece_at_index(35).unwrap(), Piece::Pawn); // d5 white pawn
    match board.get_piece_at_index(28) { // e4 should be empty
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e4 should be empty after capture"),
    }
}

#[test]
fn test_apply_move_en_passant() {
    let mut board = Board::init();
    
    // Set up en passant scenario: white pawn on 5th rank, black pawn double-moves adjacent
    board.apply_move(&Move::new("e4".to_string(), 0));
    board.apply_move(&Move::new("a6".to_string(), 1)); // Random black move
    board.apply_move(&Move::new("e5".to_string(), 2)); // White pawn to 5th rank
    board.apply_move(&Move::new("d5".to_string(), 3)); // Black pawn double move, enabling en passant
    
    // Verify setup
    assert_eq!(board.get_piece_at_index(36).unwrap(), Piece::Pawn); // e5 white pawn
    assert_eq!(board.get_piece_at_index(35).unwrap(), Piece::Pawn); // d5 black pawn
    
    // En passant capture
    let en_passant_move = Move::new("exd6".to_string(), 4);
    let source = board.apply_move(&en_passant_move);
    
    // Verify en passant worked
    assert!(source.is_some());
    assert_eq!(board.get_piece_at_index(43).unwrap(), Piece::Pawn); // d6 has white pawn
    
    // Verify captured pawn is gone
    match board.get_piece_at_index(35) { // d5 should be empty
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("d5 should be empty after en passant capture"),
    }
    
    // Verify source square is empty
    match board.get_piece_at_index(36) { // e5 should be empty
        Err(ChessError::PieceNotFound { .. }) => {},
        _ => panic!("e5 should be empty after en passant capture"),
    }
}

#[test]
fn test_apply_move_castling_rights_removal() {
    let mut board = Board::init();
    let initial_rights = board.castling_rights;
    
    // Moving king should remove castling rights
    board.apply_move(&Move::new("Ke2".to_string(), 0));
    
    // White castling rights should be removed
    assert_ne!(board.castling_rights, initial_rights);
    // Check that castling rights were reduced (king move removes castling ability)
    assert!(board.castling_rights < initial_rights);
}

#[test]
fn test_apply_move_half_move_clock() {
    let mut board = Board::init();
    
    // Non-pawn, non-capture moves should increment half-move clock
    board.apply_move(&Move::new("Nf3".to_string(), 0));
    assert_eq!(board.half_move_count, 1);
    
    board.apply_move(&Move::new("Nc6".to_string(), 1));
    assert_eq!(board.half_move_count, 2);
    
    // Pawn move should reset half-move clock
    board.apply_move(&Move::new("e4".to_string(), 2));
    assert_eq!(board.half_move_count, 0);
}

#[test]
fn test_apply_move_full_move_counter() {
    let mut board = Board::init();
    let initial_full_moves = board.full_move_count;
    
    // White move shouldn't increment full move counter
    board.apply_move(&Move::new("e4".to_string(), 0));
    assert_eq!(board.full_move_count, initial_full_moves);
    
    // Black move should increment full move counter
    board.apply_move(&Move::new("e5".to_string(), 1));
    assert_eq!(board.full_move_count, initial_full_moves + 1);
    
    // Another white move shouldn't increment
    board.apply_move(&Move::new("Nf3".to_string(), 2));
    assert_eq!(board.full_move_count, initial_full_moves + 1);
    
    // Another black move should increment again
    board.apply_move(&Move::new("Nc6".to_string(), 3));
    assert_eq!(board.full_move_count, initial_full_moves + 2);
}