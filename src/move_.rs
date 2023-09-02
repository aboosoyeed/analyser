use crate::{components::{Piece,Rank, File}, utils::{square_to_index, square_to_file_rank}, color::Color};

#[derive(Debug)]
pub struct Move{
    san: String,
    pub index: u16,
    pub piece: Piece,
    pub is_capture: bool,
    pub castling: Option<Castling>,
    target_square: Option<String> // will be None if its a castling 

}

impl Move {
    pub fn new(san:String, index:u16)->Move{
        let first_char = san.chars().next().unwrap();
        let last_char = san.chars().last().unwrap();
        let mut piece = Piece::from_char( first_char );
        let mut is_capture = false;
        let mut castling = None;

            
        
        
        if san.len()==2{
            piece = Some(Piece::Pawn)
        }else if san.contains("O-"){
            castling = Some(Castling::parse(&san));
            piece = Some(Piece::King)
        }else if san.contains("x"){
            is_capture = true;
            if first_char.is_ascii_lowercase(){
                piece = Some(Piece::Pawn)
            }
        }

        let target_square = if castling.is_some(){
            None
        }else if last_char.is_numeric(){
            Some(san[san.len() -2..].to_string())
        }else{
            Some(san[san.len() -3..san.len() -1].to_string())
        };
        

        assert!( piece.is_some(), "piece could not be destructured {}", san);
        Move { san, index, piece: piece.unwrap(), is_capture , castling, target_square}
    }

    pub fn get_target_index(&self) -> Option<u8> {
        let square = &self.target_square;
        if square.is_none(){
            return None;
        }
        return Some(square_to_index(&square.as_ref().unwrap().as_str()));
    }
    
    pub fn get_target_file_rank(&self) -> (File,Rank){
        return square_to_file_rank(self.target_square.as_ref().unwrap());
    }

    pub fn color(&self)->Color{
        if &self.index%2==0{
            Color::white
        }else{
            Color::black
        }
    }
    

}

#[derive(Debug, PartialEq)]
pub enum Castling{
    queen,
    king
}
impl Castling {
    fn parse(san:&str) ->Castling{
        if san=="O-O"{
            Castling::king
        }else {
            Castling::queen
        }
    }

    pub fn compute_squares(&self,color:Color) -> ((u8,u8),(u8,u8)){
        if self==&Castling::king && color==Color::white {
            ((4,6),(7,5))
        }else if self==&Castling::queen && color==Color::white {
            ((4,2),(0,3))
        }else if self==&Castling::king && color==Color::black{
            ((60,62),(63,61))
        }else{
            ((60,58),(56,59))
        }
    }

}