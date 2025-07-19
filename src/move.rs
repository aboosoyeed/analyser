use crate::{components::{Piece,Rank, File}, utils::{file_rank_to_index, is_piece}, color::Color};

/// Represents a chess move parsed from Standard Algebraic Notation (SAN).
/// 
/// This struct contains all the information needed to represent a chess move,
/// including the piece being moved, source and target squares, captures,
/// castling, and promotions. Moves are typically created by parsing SAN
/// strings like "e4", "Nf3", "O-O", "exd5", "e8=Q", etc.
/// 
/// # Examples
/// 
/// ```rust
/// use analyzer::r#move::Move;
/// 
/// // Parse a simple pawn move
/// let pawn_move = Move::new("e4".to_string(), 0);
/// 
/// // Parse a knight move with disambiguation
/// let knight_move = Move::new("Nbd7".to_string(), 1);
/// 
/// // Parse castling
/// let castle = Move::new("O-O".to_string(), 2);
/// 
/// // Parse a capture with promotion
/// let promotion = Move::new("exd8=Q+".to_string(), 3);
/// ```
#[derive(Debug, Clone)]
pub struct Move {
    /// The original Standard Algebraic Notation string
    pub san: String,
    /// Move index in the game (0 for first move, 1 for second, etc.)
    pub index: u16,
    /// The piece being moved
    pub piece: Piece,
    /// Whether this move captures an opponent piece
    pub is_capture: bool,
    /// Castling information if this is a castling move
    pub castling: Option<Castling>,
    /// Target square coordinates (None for castling moves)
    pub target: (Option<File>, Option<Rank>),
    /// Source square coordinates (determined during move application)
    pub source: (Option<File>, Option<Rank>),
    /// Promotion piece if this is a pawn promotion
    pub promotion: Option<Piece>
}

impl Move {
    /// Creates a new Move by parsing a Standard Algebraic Notation string.
    /// 
    /// This function parses various types of chess moves including:
    /// - Simple piece moves: "e4", "Nf3", "Bb5"
    /// - Captures: "exd5", "Nxe4", "Qxf7+"
    /// - Castling: "O-O" (kingside), "O-O-O" (queenside)
    /// - Pawn promotion: "e8=Q", "axb8=N+"
    /// - Disambiguated moves: "Nbd7", "R1e1", "Qh4e1"
    /// 
    /// # Arguments
    /// 
    /// * `san` - The Standard Algebraic Notation string to parse
    /// * `index` - The move index in the game sequence
    /// 
    /// # Returns
    /// 
    /// A new `Move` instance with all fields populated based on the SAN string.
    /// 
    /// # Panics
    /// 
    /// Panics if the SAN string cannot be parsed into a valid piece type.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use analyzer::r#move::Move;
    /// 
    /// let move1 = Move::new("e4".to_string(), 0);
    /// let move2 = Move::new("Nf3".to_string(), 1);
    /// let castle = Move::new("O-O".to_string(), 10);
    /// ```
    pub fn new(san: String, index: u16) -> Move {
        
        let mut piece = Some(Piece::Pawn);
        let mut is_capture = false;
        let mut castling = None;
        let mut source = (None,None);
        let mut target = (None,None);
        let san_chars = san.chars();
        let mut positions:Vec<char> = vec![];
        let mut promotion = None;
        let mut has_promotion= false;
        for ch in san_chars {
            if ch=='O'{
                castling = Some(Castling::parse(&san));
                piece = Some(Piece::King)
            }
            else if ch =='x' {
                is_capture = true;
            }
            else if (ch>='a' && ch<='h') || (ch>='1' && ch<='8') {
                positions.push(ch);
            }
            else if ch=='=' {
                has_promotion = true;
            }
            else if is_piece(ch){
                if has_promotion { // last charcter was =
                    promotion = Piece::from_char(ch);
                }else{
                    piece = Piece::from_char(ch);
                }
                
            }

            

        } 

        if positions.len()>0 {
            let target_start_index = if positions.len()==3{
                if let Some(f) = File::from_char(positions[0]){
                    source.0 = Some(f);
                }else{
                    source.1 = Rank::from_char(positions[0])
                }
                1
            }else{
                0
            };

            target.0 = File::from_char(positions[target_start_index]);
            target.1 = Rank::from_char(positions[target_start_index+1])
        }
        

        assert!( piece.is_some(), "piece could not be destructured {}", san);
        Move { san, index, piece: piece.unwrap(), is_capture , castling, target, source, promotion}
    }

    pub fn get_target_index(&self) -> Option<u8> {
        let (file, rank) = &self.target;
        if file.is_none() || rank.is_none() {
            return None;
        }
        // Use the safe file_rank_to_index function, but fall back to None on error
        // This maintains the existing API while adding safety
        file_rank_to_index(file.unwrap(), rank.unwrap()).ok()
    }
    
    

    pub fn color(&self)->Color{
        if &self.index%2==0{
            Color::White
        }else{
            Color::Black
        }
    }
    

}

/// Represents the type of castling move.
/// 
/// Chess allows two types of castling:
/// - Kingside castling (O-O): King moves toward the h-file rook
/// - Queenside castling (O-O-O): King moves toward the a-file rook
/// 
/// # Examples
/// 
/// ```rust
/// use analyzer::r#move::Castling;
/// 
/// // Kingside castling is represented as "O-O" in algebraic notation
/// let kingside = Castling::King;
/// 
/// // Queenside castling is represented as "O-O-O" in algebraic notation  
/// let queenside = Castling::Queen;
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Castling {
    /// Queenside castling (O-O-O) - King moves toward the a-file
    Queen,
    /// Kingside castling (O-O) - King moves toward the h-file  
    King
}
impl Castling {
    fn parse(san:&str) ->Castling{
        if san=="O-O"{
            Castling::King
        }else {
            Castling::Queen
        }
    }

    pub fn compute_squares(&self,color:Color) -> ((u8,u8),(u8,u8)){
        if self==&Castling::King && color==Color::White {
            ((4,6),(7,5))
        }else if self==&Castling::Queen && color==Color::White {
            ((4,2),(0,3))
        }else if self==&Castling::King && color==Color::Black{
            ((60,62),(63,61))
        }else{
            ((60,58),(56,59))
        }
    }

    

}
