use crate::{components::{Piece,Rank, File}, utils::{square_to_file_rank, file_rank_to_index}, color::Color};

#[derive(Debug)]
pub struct Move{
    pub san: String,
    pub index: u16,
    pub piece: Piece,
    pub is_capture: bool,
    pub castling: Option<Castling>,
    pub target: (Option<File>,Option<Rank>), // will be None if its a castling 
    pub source:(Option<File>,Option<Rank>)
}

impl Move {
    pub fn new(san:String, index:u16)->Move{
        let first_char = san.chars().next().unwrap();
        let last_char = san.chars().last().unwrap();
        let mut piece = Piece::from_char( first_char );
        let mut is_capture = false;
        let mut castling = None;
        let mut source = (None,None);
        let mut target = (None,None);
                        
        
        
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
        
        if san.len()>3 && castling.is_none() { // if its none of the above it surely deals with disambiguity
            let ch = san.chars().nth(1);
            if let Some(f) = File::from_char(ch.unwrap()){
                source.0 = Some(f);
            }else{
                source.1 = Rank::from_char(ch.unwrap())
            }
        }


        let target_square = if castling.is_some(){
            None
        }else if last_char.is_numeric(){
            Some(san[san.len() -2..].to_string())
        }else{
            Some(san[san.len() -3..san.len() -1].to_string())
        };

        if target_square.is_some(){
            let (f,r)=square_to_file_rank(target_square.unwrap().as_str());
            target = (Some(f),Some(r));
        }
        
        

        assert!( piece.is_some(), "piece could not be destructured {}", san);
        Move { san, index, piece: piece.unwrap(), is_capture , castling, target, source}
    }

    pub fn get_target_index(&self) -> Option<u8> {
        let (file,rank) = &self.target;
        if file.is_none() || rank.is_none(){
            return None;
        }
        return Some(file_rank_to_index(file.unwrap(),rank.unwrap()));
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
