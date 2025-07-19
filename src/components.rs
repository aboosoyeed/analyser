use std::ops::Sub;

use crate::{bitboard::Bitboard, board::Board, r#move::Move, utils::compute_attack_squares, color::Color};

macro_rules!  define_piece{
    ($($name:ident
        {
            delta:[$($delta:expr),*],
            chr:$chr:expr,
            unicode:$unicode:expr,
            step_only:$step_only:expr
        }
    ),*) => {
        
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum Piece{
            $(
                $name,
            )*

        }
        
        impl Piece{
            
            fn delta(&self) -> &[i8] {
                match self {
                    $(
                        Piece::$name => &[$($delta),*],
                    )*
                }
            }
            
            
            pub fn to_char(&self, color:Color) -> char {
                match self {
                    $(
                        Piece::$name => if color==Color::White{
                            $chr.to_ascii_uppercase()
                        }else{
                            $chr
                        },
                    )*
                }
            }

            pub fn to_unicode(&self) -> char {
                match self {
                    $(
                        Piece::$name => $unicode,
                    )*
                }
            }
             
             
            pub fn compute_source(&self,board:&Board,mov:&Move)->Result<u8, String>{
                match self {
                    $(
                        Piece::$name=> Self::_compute_source(board,mov,self.delta(),$step_only),
                    )*
                
                }
            }
            
        }
        
    };
}

define_piece!(
    Pawn{
        delta:[7,9],
        chr:'p',
        unicode:'♙',
        step_only:true
    }, 
    Knight{
        delta:[17, 15, 10, 6, -17, -15, -10, -6],
        chr:'n',
        unicode:'♘',
        step_only:true
    },
    Bishop{
        delta:[9, 7, -9, -7],
        chr:'b',
        unicode:'♗',
        step_only:false
    },
    Rook{
        delta:[1,8,-1,-8],
        chr:'r',
        unicode:'♖',
        step_only:false
    },
    Queen{
        delta:[1,8,7,9,-1,-8, -7, -9],
        chr:'q',
        unicode:'♕',
        step_only:false
    },
    King{
        delta:[9, 8, 7, 1, -9, -8, -7, -1],
        chr:'k',
        unicode:'♔',
        step_only:true
    }
);


impl Piece {
    pub const fn from_char(ch: char) -> Option<Piece> {
        match ch {
            'P' | 'p' => Some(Piece::Pawn),
            'N' | 'n' => Some(Piece::Knight),
            'B' | 'b' => Some(Piece::Bishop),
            'R' | 'r' => Some(Piece::Rook),
            'Q' | 'q' => Some(Piece::Queen),
            'K' | 'k' => Some(Piece::King),
            _ => None,
        }
    }

    pub fn get_all() ->Vec<Piece>{
        vec![
            Piece::Pawn,Piece::Knight,Piece::Bishop,Piece::Rook,Piece::Queen,Piece::King,
             
        ]
    }

    fn _compute_source(board:&Board,mov:&Move, deltas:&[i8], step_only:bool) -> Result<u8, String>{
        let piece = mov.piece;
        let piece_bitboard = board.by_piece.get(piece);
        let color_bitboard = board.by_color.get(mov.color());
        let occupancy = board.occupied;
        let mut deltas = deltas;
        
        if piece==Piece::Pawn {
            deltas = if mov.color()==Color::White{
                &[-7, -9]
            }else{
                &[7, 9]
            };
        }
        
        let mut attack_bitboard = if !mov.is_capture && mov.piece==Piece::Pawn {
            let (file,_) = mov.target;
            let file_bitboard = file.unwrap().get_bit_board();
            file_bitboard.get()
        }else{
            compute_attack_squares(occupancy,mov.get_target_index().unwrap() as i8, deltas, step_only)
        };
        
        if mov.source.0.is_some(){
            attack_bitboard = attack_bitboard & mov.source.0.unwrap().get_bit_board().get()
        }else if mov.source.1.is_some(){
            attack_bitboard = attack_bitboard & mov.source.1.unwrap().get_bit_board().get()    
        }

        let source = piece_bitboard.get() & color_bitboard.get() & attack_bitboard;
        
        
        
        if source.count_ones() != 1 {
            return Err(format!("Move validation failed for {}.{}: expected exactly one source square, found {}. Attack pattern:\n{}", 
                             mov.index, mov.san, source.count_ones(), Bitboard(attack_bitboard).printable()));
        }
        Ok(source.trailing_zeros() as u8)

    }    
}



#[derive(Copy,Clone,Debug)]
pub enum File{
    A=0,B,C,D,E,F,G,H
}

impl File {

    pub fn get_bit_board(&self)->Bitboard{
        match  self {
            File::A => Bitboard(0x0101_0101_0101_0101),
            File::B => Bitboard(0x0202_0202_0202_0202),
            File::C => Bitboard(0x0404_0404_0404_0404),
            File::D => Bitboard(0x0808_0808_0808_0808),
            File::E => Bitboard(0x1010_1010_1010_1010),
            File::F => Bitboard(0x2020_2020_2020_2020),
            File::G => Bitboard(0x4040_4040_4040_4040),
            File::H => Bitboard(0x8080_8080_8080_8080),
        }
    }

    pub const fn from_char(ch: char) -> Option<File> {
        match ch {
            'A' | 'a' => Some(File::A),
            'B' | 'b' => Some(File::B),
            'C' | 'c' => Some(File::C),
            'D' | 'd' => Some(File::D),
            'E' | 'e' => Some(File::E),
            'F' | 'f' => Some(File::F),
            'G' | 'g' => Some(File::G),
            'H' | 'h' => Some(File::H),
            _ => None,
        }
    }

    pub const fn to_char(&self) -> char {
        match self {
            File::A =>'a',
            File::B =>'b',
            File::C =>'c',
            File::D =>'d',
            File::E =>'e',
            File::F =>'f',
            File::G =>'g',
            File::H =>'h'
        }
    }   

}

impl Sub for File {
    type Output = usize;

    fn sub(self, other: File) -> usize {
        let self_value = self as usize;
        let other_value = other as usize;
        self_value.abs_diff(other_value)
        
    }
}  

#[derive(Copy,Clone,Debug)]
pub enum Rank{
    First=0,Second,Third,Fourth,Fifth,Sixth,Seventh,Eighth,
}
impl Rank {
    pub fn get_bit_board(&self)->Bitboard{
        match  self {
            Rank::First =>  Bitboard(0x0000_0000_0000_00FF),
            Rank::Second => Bitboard(0x0000_0000_0000_FF00),
            Rank::Third =>  Bitboard(0x0000_0000_00FF_0000),
            Rank::Fourth => Bitboard(0x0000_0000_FF00_0000),
            Rank::Fifth =>  Bitboard(0x0000_00FF_0000_0000),
            Rank::Sixth =>  Bitboard(0x0000_FF00_0000_0000),
            Rank::Seventh => Bitboard(0x00FF_0000_0000_0000),
            Rank::Eighth => Bitboard(0xFF00_0000_0000_0000),
        }
    }

    pub const fn from_char(ch: char) -> Option<Rank> {
        match ch {
            '1' => Some(Rank::First),
            '2' => Some(Rank::Second),
            '3' => Some(Rank::Third),
            '4' => Some(Rank::Fourth),
            '5' => Some(Rank::Fifth),
            '6' => Some(Rank::Sixth),
            '7' => Some(Rank::Seventh),
            '8' => Some(Rank::Eighth),
            _ => None,
        }
    }   

}

impl Sub for Rank {
    type Output = usize;

    fn sub(self, other: Rank) -> usize {
        let self_value = self as usize;
        let other_value = other as usize;
        self_value.abs_diff(other_value)
        
    }
}