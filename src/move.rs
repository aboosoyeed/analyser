use crate::{components::{Piece,Rank, File}, utils::{file_rank_to_index, is_piece}, color::Color};

#[derive(Debug, Clone)]
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
        
        let mut piece = Some(Piece::Pawn);
        let mut is_capture = false;
        let mut castling = None;
        let mut source = (None,None);
        let mut target = (None,None);
                        
        let san_chars = san.chars();
        let mut positions:Vec<char> = vec![];
        
        for ch in san_chars{
            if ch=='O'{
                castling = Some(Castling::parse(&san));
                piece = Some(Piece::King)
            }
            if ch =='x' {
                is_capture = true;
            }
            if (ch>='a' && ch<='h') || (ch>='1' && ch<='8') {
                positions.push(ch);
            }

            if is_piece(ch){
                piece = Piece::from_char(ch);
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
            Color::White
        }else{
            Color::Black
        }
    }
    

}

#[derive(Debug, PartialEq, Clone,Copy)]
pub enum Castling{
    Queen,
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
