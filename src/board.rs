use core::fmt;

use crate::{
    bitboard::Bitboard,
    color::{ByColor, Color},
    components::Piece,
    constants::{board, game_state, squares},
    error::{ChessError, Square},
    fen::generate,
    r#move::Move,
    role::ByPiece,
};

/// Represents a chess board position using bitboards for efficient operations.
///
/// The board uses separate bitboards for each piece type and color, allowing
/// for fast position queries and move generation. It maintains all necessary
/// game state including castling rights, move counters, and piece positions.
///
/// # Examples
///
/// ```rust
/// use analyzer::board::Board;
/// use analyzer::r#move::Move;
///
/// // Create a new board in starting position
/// let mut board = Board::init();
///
/// // Apply a move
/// let move_e4 = Move::new("e4".to_string(), 0);
/// board.apply_move(&move_e4);
///
/// // Display the board
/// println!("{}", board);
/// ```
#[derive(Clone)]
pub struct Board {
    /// Bitboards for each piece type (pawn, knight, bishop, rook, queen, king)
    pub by_piece: ByPiece,
    /// Bitboards for each color (white, black)
    pub by_color: ByColor,
    /// Combined bitboard of all occupied squares
    pub occupied: Bitboard,
    /// Castling rights encoded as 4 bits: KQkq (white king/queen, black king/queen)
    pub castling_rights: u8,
    /// Number of half-moves since last pawn move or capture (for 50-move rule)
    pub half_move_count: u8,
    /// Full move counter (incremented after Black's move)
    pub full_move_count: u16,
    /// Fast lookup table for piece at each square (None if empty)
    piece_lookup: [Option<Piece>; 64],
}

impl Board {
    /// Creates a new chess board in the standard starting position.
    ///
    /// The board is initialized with all pieces in their starting squares:
    /// - White pieces on ranks 1-2
    /// - Black pieces on ranks 7-8
    /// - All castling rights available
    /// - Move counters reset to starting values
    ///
    /// # Returns
    ///
    /// A new `Board` instance ready for gameplay.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use analyzer::board::Board;
    ///
    /// let board = Board::init();
    /// assert_eq!(board.full_move_count, 1);
    /// assert_eq!(board.half_move_count, 0);
    /// ```
    pub fn init() -> Board {
        let by_piece = ByPiece::init();
        let by_color = ByColor::init();
        let occupied = Bitboard(game_state::STARTING_OCCUPIED);
        
        let mut board = Board {
            by_piece,
            by_color,
            occupied,
            castling_rights: game_state::ALL_CASTLING_RIGHTS,
            half_move_count: game_state::STARTING_HALF_MOVES,
            full_move_count: game_state::STARTING_FULL_MOVES,
            piece_lookup: [None; 64],
        };
        
        board.rebuild_piece_lookup();
        board
    }

    /// Rebuilds the piece lookup table from current bitboard state.
    ///
    /// This method scans all piece bitboards and populates the lookup table
    /// for O(1) piece queries. Called after board initialization and whenever
    /// the bitboards are modified.
    fn rebuild_piece_lookup(&mut self) {
        // Clear the lookup table
        self.piece_lookup = [None; 64];
        
        // Populate with current piece positions
        for piece in Piece::get_all() {
            let piece_board = self.by_piece.get(piece);
            for index in 0..64 {
                if piece_board.get_bit(index) {
                    self.piece_lookup[index as usize] = Some(piece);
                }
            }
        }
    }

    /// Applies a move to the board, updating all relevant state.
    ///
    /// This handles all types of moves including:
    /// - Normal piece moves
    /// - Captures (including en passant)
    /// - Castling
    /// - Pawn promotion
    ///
    /// The method automatically updates:
    /// - Piece positions on the bitboards
    /// - Castling rights when kings or rooks move
    /// - Half-move clock for the 50-move rule
    /// - Full move counter
    ///
    /// # Arguments
    ///
    /// * `mov` - The move to apply in parsed algebraic notation
    ///
    /// # Returns
    ///
    /// Returns the source square index for normal moves, or `None` for castling moves.
    /// This is used by PGN processing to determine the source square for ambiguous notation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use analyzer::board::Board;
    /// use analyzer::r#move::Move;
    ///
    /// let mut board = Board::init();
    /// let move_e4 = Move::new("e4".to_string(), 0);
    /// let source = board.apply_move(&move_e4);
    /// // source will be Some(12) indicating the pawn moved from e2
    /// ```
    pub fn apply_move(&mut self, mov: &Move) -> Option<u8> {
        let mut source: Option<u8> = None;

        if mov.is_capture || mov.piece == { Piece::Pawn } {
            self.half_move_count = game_state::STARTING_HALF_MOVES
        } else {
            self.half_move_count += 1;
        }

        if mov.color() == Color::Black {
            self.full_move_count += 1;
        }

        if mov.castling.is_some() {
            self.apply_castling(mov)
        } else {
            source = Some(self.apply_normal_move(mov));
        }
        
        // Update piece lookup table after any move
        self.rebuild_piece_lookup();
        
        source
    }

    fn apply_castling(&mut self, mov: &Move) {
        let color = mov.color();
        let castling = mov.castling;
        let ((ks, kt), (rs, rt)) = castling.unwrap().compute_squares(color);
        self.move_piece(ks, kt, color, Piece::King);
        self.move_piece(rs, rt, color, Piece::Rook);

        // remove all castling rights for the side
        self.remove_castling_rights(color, true);
        self.remove_castling_rights(color, false);
    }

    fn remove_castling_rights(&mut self, color: Color, is_king_side: bool) {
        let mask = match (color, is_king_side) {
            (Color::White, true) => 0b_0111,
            (Color::White, false) => 0b_1011,
            (Color::Black, true) => 0b_1101,
            (Color::Black, false) => 0b_1110,
        };

        self.castling_rights = self.castling_rights & mask;
    }

    fn apply_normal_move(&mut self, mov: &Move) -> u8 {
        let target = mov.get_target_index();
        let color = mov.color();
        let piece = mov.piece;
        let is_capture = mov.is_capture;
        let promotion = mov.promotion;
        let source = self.get_source_index(mov);

        if is_capture {
            let mut opp_piece = self.get_piece_at_index(target.unwrap());
            let mut opponent_target = target.unwrap();

            if opp_piece.is_err() && piece == Piece::Pawn {
                // potentially enpassant
                opponent_target = if color == Color::White {
                    target.unwrap() - 8
                } else {
                    target.unwrap() + 8
                };
                opp_piece = self.get_piece_at_index(opponent_target);
                self.occupied.clear_bit(opponent_target);
            }

            let opp_color_board = &mut self.by_color.get_mut(color.get_opposite());
            opp_color_board.clear_bit(opponent_target);

            let opp_piece_board = &mut self.by_piece.get_mut(opp_piece.unwrap());
            opp_piece_board.clear_bit(opponent_target);
        }

        if piece == Piece::Rook && squares::ALL_ROOK_SQUARES.contains(&source) {
            self.remove_castling_rights(
                color,
                squares::WHITE_ROOKS[1] == source || squares::BLACK_ROOKS[1] == source,
            );
        }

        if piece == Piece::King
            && (source == squares::WHITE_KING_START || source == squares::BLACK_KING_START)
        {
            // remove all castling rights for the side
            self.remove_castling_rights(color, true);
            self.remove_castling_rights(color, false);
        }

        self.move_piece(source, target.unwrap(), color, piece);

        // once we have dealt with capture and moving piece. we deal with promotion

        if promotion.is_some() {
            let current_piece_board = &mut self.by_piece.get_mut(piece);
            current_piece_board.clear_bit(target.unwrap());

            let promotion_piece_board = &mut self.by_piece.get_mut(promotion.unwrap());
            promotion_piece_board.set_bit(target.unwrap());
        }

        source
    }

    fn move_piece(&mut self, source: u8, target: u8, color: Color, piece: Piece) {
        self.occupied.toggle(source, target);

        let color_board = &mut self.by_color.get_mut(color);
        color_board.toggle(source, target);

        let piece_board = &mut self.by_piece.get_mut(piece);
        piece_board.toggle(source, target);
    }

    /// Generates a FEN (Forsyth-Edwards Notation) string representing the current position.
    ///
    /// FEN is a standard notation for describing chess positions. The generated string
    /// includes piece placement, active color, castling availability, en passant target,
    /// half-move clock, and full-move number.
    ///
    /// # Arguments
    ///
    /// * `last_move` - The last move played, used to determine en passant possibilities
    ///
    /// # Returns
    ///
    /// A FEN string representing the current board position.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use analyzer::board::Board;
    /// use analyzer::r#move::Move;
    ///
    /// let mut board = Board::init();
    /// let move_e4 = Move::new("e4".to_string(), 0);
    /// board.apply_move(&move_e4);
    /// let fen = board.generate_fen(&move_e4);
    /// // Returns FEN representation of the position
    /// ```
    pub fn generate_fen(&self, last_move: &Move) -> String {
        generate(self, last_move)
    }

    pub fn get_source_index(&self, mov: &Move) -> u8 {
        let piece = mov.piece;
        piece.compute_source(self, mov)
    }

    pub fn get_piece_at_index(&self, index: u8) -> Result<Piece, ChessError> {
        // Validate square index
        let square = Square::new(index)?;
        
        // Use O(1) lookup table instead of O(6) bitboard search
        match self.piece_lookup[square.index() as usize] {
            Some(piece) => Ok(piece),
            None => Err(ChessError::PieceNotFound { square: index })
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..board::RANKS).rev() {
            write!(f, "{} ", rank + 1)?; // Rank labels
            for file in 0..board::FILES {
                let index = rank * board::FILES + file;
                if self.occupied.get_bit(index) {
                    let color_code = if self.by_color.white.get_bit(index) {
                        "33" // yellow
                    } else {
                        "34" // blue
                    };
                    match self.get_piece_at_index(index) {
                        Ok(piece) => {
                            write!(f, "\x1b[{}m{}\x1b[0m ", color_code, piece.to_unicode())?;
                        },
                        Err(_) => {
                            write!(f, "\x1b[37m◻\x1b[0m ")?; // gray fallback
                        }
                    }
                } else {
                    write!(f, "\x1b[37m◻\x1b[0m ")?; // gray empty square
                }
            }
            writeln!(f)?;
        }
        // Add file labels
        write!(f, "  ")?;
        for file in 0..board::FILES {
            write!(f, "{} ", (b'a' + file) as char)?;
        }
        writeln!(f)?;
        Ok(())
    }
}
