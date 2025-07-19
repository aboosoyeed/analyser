/// Chess-specific constants and square indices.
///
/// This module contains all hard-coded values related to chess board geometry,
/// piece positions, and game rules. Using constants improves maintainability
/// and makes the code more readable.

/// Square indices for important board positions.
pub mod squares {
    /// White king starting position (e1)
    pub const WHITE_KING_START: u8 = 4;
    
    /// Black king starting position (e8)
    pub const BLACK_KING_START: u8 = 60;
    
    /// White rook starting positions [a1, h1]
    pub const WHITE_ROOKS: [u8; 2] = [0, 7];
    
    /// Black rook starting positions [a8, h8]
    pub const BLACK_ROOKS: [u8; 2] = [56, 63];
    
    /// All initial rook positions for castling rights tracking
    pub const ALL_ROOK_SQUARES: [u8; 4] = [0, 7, 56, 63];
}

/// Castling-related constants.
pub mod castling {
    /// White kingside castling: (king_from, king_to), (rook_from, rook_to)
    pub const WHITE_KINGSIDE: ((u8, u8), (u8, u8)) = ((4, 6), (7, 5));
    
    /// White queenside castling: (king_from, king_to), (rook_from, rook_to)
    pub const WHITE_QUEENSIDE: ((u8, u8), (u8, u8)) = ((4, 2), (0, 3));
    
    /// Black kingside castling: (king_from, king_to), (rook_from, rook_to)
    pub const BLACK_KINGSIDE: ((u8, u8), (u8, u8)) = ((60, 62), (63, 61));
    
    /// Black queenside castling: (king_from, king_to), (rook_from, rook_to)
    pub const BLACK_QUEENSIDE: ((u8, u8), (u8, u8)) = ((60, 58), (56, 59));
}

/// Board geometry constants.
pub mod board {
    /// Number of ranks on a chess board
    pub const RANKS: u8 = 8;
    
    /// Number of files on a chess board
    pub const FILES: u8 = 8;
    
    /// Total number of squares on a chess board
    pub const TOTAL_SQUARES: u8 = 64;
    
    /// Maximum valid square index (h8)
    pub const MAX_SQUARE_INDEX: u8 = 63;
    
    /// Minimum valid square index (a1)
    pub const MIN_SQUARE_INDEX: u8 = 0;
}

/// Initial game state constants.
pub mod game_state {
    /// Starting position occupied squares bitboard
    pub const STARTING_OCCUPIED: u64 = 0xffff_0000_0000_ffff;
    
    /// All castling rights available (KQkq)
    pub const ALL_CASTLING_RIGHTS: u8 = 0b_1111;
    
    /// Starting half-move count (for 50-move rule)
    pub const STARTING_HALF_MOVES: u8 = 0;
    
    /// Starting full-move count
    pub const STARTING_FULL_MOVES: u16 = 1;
}

/// Default file paths and names.
pub mod defaults {
    /// Default PGN file for testing and examples
    pub const DEFAULT_PGN_PATH: &str = "./tests/pgn/3.pgn";
    
    /// Default engine search depth
    pub const DEFAULT_ENGINE_DEPTH: u8 = 16;
}