
use std::fs;

use crate::{pgn_header::PgnHeaders, board::Board, r#move::Move};


pub struct PGN{
    headers:PgnHeaders,
    pub moves: Vec<Move>,
    _move_counter:u16
}

impl PGN{
    
    pub fn parse(path:&str) -> Vec<String>{
        let mut board = Board::init();
    
        let mut pgn = PGN{ headers: PgnHeaders::new() , moves: Vec::new(), _move_counter:0};
        let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");
        
        for line in contents.lines(){
            pgn.process_line(line);   
        }
        let moves = pgn.moves;
        let mut fens:Vec<String> =Vec::new();
        for mov in moves{
            let cloned_move = mov.clone();
            board.apply_move(mov);
            let fen = board.generate_fen(cloned_move);
            fens.push(fen);
        }
        fens
    }

    fn process_line(&mut self, line:&str){
        let mut move_string = String::from("");
        match line.get(0..1) {
            None => (),
            Some("[") => self.extract_meta_from_line(line.get(1..line.len()-1).unwrap()),
            Some(&_) => move_string += line
    
        };
        
        self.extract_moves(&move_string);
    }
    
    fn extract_moves(&mut self, move_str:&str){
        
        for ( _,token) in move_str.split(" ").enumerate() {
            if token.len()>0 && token.get(token.len()-1..) != Some(".") {
                if token == "1-0" || token == "0-1" || token =="0-0"{
                    continue;        
                }

                self.moves.push(
                    Move::new(String::from(token), self._move_counter)
                );
                self._move_counter+=1;
            }
        }
        
    }

    fn extract_meta_from_line(&mut self, line:&str){
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

    pub fn print_headers(&self){
        self.headers.print_headers();
    }

}
