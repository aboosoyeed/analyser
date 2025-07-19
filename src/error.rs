/// Custom error types for type-safe error handling in the chess analyzer.
///
/// This module provides structured error types to replace string literals
/// and improve compile-time safety.
#[derive(Debug, Clone, PartialEq)]
pub enum ChessError {
    /// No piece found at the specified square index
    PieceNotFound { square: u8 },
    
    /// Invalid square index (must be 0-63)
    InvalidSquare { index: u8 },
    
    /// Invalid move string format
    InvalidMove { move_str: String },
    
    /// File or rank index out of bounds
    InvalidCoordinate { coordinate: String },
}

impl std::fmt::Display for ChessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChessError::PieceNotFound { square } => {
                write!(f, "[Chess Analyzer] Board error: No piece found at square {}", square)
            }
            ChessError::InvalidSquare { index } => {
                write!(f, "[Chess Analyzer] Coordinate error: Invalid square index {} (must be 0-63)", index)
            }
            ChessError::InvalidMove { move_str } => {
                write!(f, "[Chess Analyzer] Move error: Invalid move '{}'", move_str)
            }
            ChessError::InvalidCoordinate { coordinate } => {
                write!(f, "[Chess Analyzer] Coordinate error: Invalid coordinate '{}'", coordinate)
            }
        }
    }
}

impl std::error::Error for ChessError {}

/// Safe wrapper for chess board square indices.
///
/// Ensures square indices are always valid (0-63) and provides
/// type-safe conversion between index and file/rank coordinates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Square(u8);

impl Square {
    /// Create a new Square from a raw index with bounds checking.
    ///
    /// # Arguments
    /// * `index` - The square index (must be 0-63)
    ///
    /// # Returns
    /// * `Ok(Square)` if index is valid
    /// * `Err(ChessError::InvalidSquare)` if index is out of bounds
    pub fn new(index: u8) -> Result<Self, ChessError> {
        if index > 63 {
            return Err(ChessError::InvalidSquare { index });
        }
        Ok(Square(index))
    }

    /// Create a Square from file and rank with bounds checking.
    ///
    /// # Arguments
    /// * `file` - The file (a-h, represented as 0-7)
    /// * `rank` - The rank (1-8, represented as 0-7)
    ///
    /// # Returns
    /// * `Ok(Square)` if coordinates are valid
    /// * `Err(ChessError::InvalidSquare)` if coordinates are out of bounds
    pub fn from_file_rank(file: u8, rank: u8) -> Result<Self, ChessError> {
        if file > 7 || rank > 7 {
            return Err(ChessError::InvalidSquare { index: rank * 8 + file });
        }
        Ok(Square(rank * 8 + file))
    }

    /// Get the raw square index (0-63).
    pub fn index(self) -> u8 {
        self.0
    }

    /// Convert to file and rank coordinates.
    ///
    /// # Returns
    /// * `(file, rank)` where both are 0-7
    pub fn to_file_rank(self) -> (u8, u8) {
        (self.0 % 8, self.0 / 8)
    }

    /// Convert to chess notation coordinates.
    ///
    /// # Returns
    /// * `(file_char, rank_char)` like ('e', '4') for e4
    pub fn to_notation(self) -> (char, char) {
        let (file, rank) = self.to_file_rank();
        ((b'a' + file) as char, (b'1' + rank) as char)
    }
}

impl std::fmt::Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (file, rank) = self.to_notation();
        write!(f, "{}{}", file, rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_creation() {
        assert!(Square::new(0).is_ok());
        assert!(Square::new(63).is_ok());
        assert!(Square::new(64).is_err());
    }

    #[test]
    fn test_square_from_file_rank() {
        assert!(Square::from_file_rank(0, 0).is_ok());
        assert!(Square::from_file_rank(7, 7).is_ok());
        assert!(Square::from_file_rank(8, 0).is_err());
        assert!(Square::from_file_rank(0, 8).is_err());
    }

    #[test]
    fn test_square_conversion() {
        let square = Square::new(28).unwrap(); // e4
        assert_eq!(square.to_file_rank(), (4, 3));
        assert_eq!(square.to_notation(), ('e', '4'));
        assert_eq!(square.to_string(), "e4");
    }
}