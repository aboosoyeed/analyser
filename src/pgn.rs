
#![allow(dead_code)]
use regex::Regex;

use crate::{pgn_header::PgnHeaders, board::Board, r#move::Move, utils::{index_to_file_rank, get_header_regex}};

/// Represents a chess game in Portable Game Notation (PGN) format.
/// 
/// PGN is a standard format for recording chess games. This struct provides
/// two distinct usage patterns for different application modes:
/// 
/// 1. **Analysis Mode**: Use `PGN::parse()` to get FEN strings for engine analysis
/// 2. **Navigation Mode**: Use `extract_headers()` + `extract_moves()` for interactive replay
/// 
/// This dual-purpose design supports both automated chess engine analysis and 
/// interactive step-by-step game navigation.
/// 
/// # Examples
/// 
/// ## Analysis Mode (for chess engine processing)
/// ```rust
/// use analyzer::pgn::PGN;
/// 
/// let pgn_content = "1. e4 e5 2. Nf3 Nc6";
/// let fens = PGN::parse(pgn_content.to_string());
/// // Returns FEN strings for each position after e4, e5, Nf3, Nc6
/// ```
/// 
/// ## Navigation Mode (for interactive game replay)
/// ```rust
/// # use analyzer::pgn::PGN;
/// # use analyzer::pgn_header::PgnHeaders;
/// let mut pgn = PGN {
///     headers: PgnHeaders::new(),
///     moves: Vec::new(),
///     _move_counter: 0,
/// };
/// # let pgn_content = "1. e4 e5".to_string();
/// pgn.extract_headers(pgn_content.clone());
/// pgn.extract_moves(pgn_content);
/// // Now pgn.moves contains Move objects for step-by-step navigation
/// ```
pub struct PGN{
    /// Game metadata like event, players, date, etc.
    pub headers: PgnHeaders,
    /// Sequence of moves in the game
    pub moves: Vec<Move>,
    /// Internal move counter for processing
    pub _move_counter: u16
}

impl PGN{

    pub fn new(contents: String) -> Self {
        let mut pgn = PGN {
            headers: PgnHeaders::new(),
            moves: Vec::new(),
            _move_counter: 0,
        };
        pgn.extract_headers(contents.clone());
        pgn.extract_moves(contents);
        pgn
    }

    /// **Analysis Mode**: Parses a PGN string and returns FEN representations for engine analysis.
    /// 
    /// This method is designed for chess engine workflows. It takes a complete PGN
    /// file content, extracts headers and moves, then simulates the game to
    /// generate FEN strings for each position after each move.
    /// 
    /// Use this for automated analysis where you need FEN strings to feed to a chess engine.
    /// 
    /// # Arguments
    /// 
    /// * `contents` - Complete PGN file content as a string
    /// 
    /// # Returns
    /// 
    /// A vector of FEN strings, where each string represents the board position
    /// after the corresponding move in the game.
    /// 
    /// # Examples
    /// 
    /// ```rust
    /// use analyzer::pgn::PGN;
    /// 
    /// let pgn = "1. e4 e5 2. Nf3 Nc6";
    /// let positions = PGN::parse(pgn.to_string());
    /// // Returns FEN strings for each position after e4, e5, Nf3, Nc6
    /// ```
    pub fn parse(contents: String) -> Vec<String> {
        let mut pgn = Self::new(contents);
        pgn.parse_moves()
    }
    
    pub fn parse_moves(&mut self) -> Vec<String>{
        let mut board = Board::init();
        let mut fens:Vec<String> =Vec::new();
        for mov in &mut self.moves{
            let source = board.apply_move(&mov);

            if source.is_some() {
                // Use the safe index_to_file_rank function
                match index_to_file_rank(source.unwrap()) {
                    Ok((file, rank)) => {
                        mov.source = (Some(file), Some(rank));
                    }
                    Err(_) => {
                        // Log error but continue processing - keeps existing behavior
                        eprintln!("[Chess Analyzer] Warning: Invalid source square index {}", source.unwrap());
                        mov.source = (None, None);
                    }
                }
            }


            let fen = board.generate_fen(&mov);
            fens.push(fen);

        }
        //println!("{}",board);
        fens

    }

    /// **Navigation Mode**: Extracts and parses PGN headers for interactive game replay.
    /// 
    /// This method is part of the navigation workflow. It parses PGN header tags
    /// like [Event "..."], [White "..."], etc. and populates the headers field.
    /// 
    /// Use this in combination with `extract_moves()` when you need structured
    /// access to game metadata and moves for step-by-step navigation.
    pub fn extract_headers(&mut self, contents:String){
        let header_pattern = get_header_regex();
        let headers:Vec<&str> = header_pattern.find_iter(&contents).map(|m| m.as_str()).collect();
        for line in headers{
            self._extract_meta_from_line(line.get(1..line.len()-1).unwrap())
        }

    }

    /// **Navigation Mode**: Extracts and parses moves for interactive game replay.
    /// 
    /// This method is part of the navigation workflow. It parses the move sequence
    /// from PGN content and populates the moves field with Move objects.
    /// 
    /// Use this in combination with `extract_headers()` when you need to navigate
    /// through a game move by move in interactive mode.
    pub fn extract_moves(&mut self, contents:String){
        let header_pattern = get_header_regex();
        let move_list = header_pattern.replace_all(&contents, "");
        let move_list = move_list.trim();
        let move_list = move_list.replace("\n", "");
        
        let alt_moves_pattern = Regex::new(r"(\(.*\))").unwrap();
        let move_list = alt_moves_pattern.replace_all(&move_list, "").to_string();    
        for ( _,token) in move_list.split(" ").enumerate() {
            if token.len()==0 { // has length
                continue;
            }
            if token.get(token.len()-1..) == Some("."){ //is a counter
                continue;
            }
            if token == "1-0" || token == "0-1" || token =="0-0" || token=="*"{ //end
                continue;        
            }
            self.moves.push(
                Move::new(String::from(token), self._move_counter)
            );
            self._move_counter+=1;
        }
    }

    
    
    fn _extract_meta_from_line(&mut self, line:&str){
        let mut key  = String::from("");
        let mut val  = String::from("");
        
        let mut is_key_found = false;
        for (_, c) in line.chars().enumerate() {
            
            if c!=' '{
                if is_key_found {
                    val.push(c) 
                } else{
                    key.push(c);    
                } 
                    
            }else{
                is_key_found = true;
            }
        }
        
        self.set_header(key, val.trim_matches('"').to_owned());
    }

    fn set_header(&mut self, key:String, val:String){
        
        match key.to_lowercase().as_str() {
            "event" => self.headers.set_event(val),
            "site" => self.headers.set_site(val),
            "date" => self.headers.set_date(val),
            "white" => self.headers.set_white(val),
            "black" => self.headers.set_black(val),
            "result" => self.headers.set_result(val),
            &_ => ()
        }
    }

    

}
