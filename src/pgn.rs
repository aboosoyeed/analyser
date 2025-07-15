
#![allow(dead_code)]
use regex::Regex;

use crate::{pgn_header::PgnHeaders, board::Board, r#move::Move, utils::{index_to_file_rank, get_header_regex}};

pub struct PGN{
    pub headers:PgnHeaders,
    pub moves: Vec<Move>,
    pub _move_counter:u16
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
                //let (file,rank) =index_to_file_rank(source.unwrap());
                mov.source = index_to_file_rank(source.unwrap());
            }


            let fen = board.generate_fen(&mov);
            fens.push(fen);

        }
        //println!("{}",board);
        fens

    }

    pub fn extract_headers(&mut self, contents:String){
        let header_pattern = get_header_regex();
        let headers:Vec<&str> = header_pattern.find_iter(&contents).map(|m| m.as_str()).collect();
        for line in headers{
            self._extract_meta_from_line(line.get(1..line.len()-1).unwrap())
        }

    }

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
