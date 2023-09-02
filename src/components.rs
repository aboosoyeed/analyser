use crate::{bitboard::Bitboard, board::Board, move_::Move, utils::compute_attack_squares, color::Color};

macro_rules!  define_piece{
    ($($name:ident),*) => {
        #[derive(Copy,Clone,Debug)]
        pub enum Piece{
            $(
                $name($name),
            )*

        }

        impl Piece{
            pub fn compute_source(&self,board:&Board,mov:Move)->u8{
                return match self {
                    $(
                        Piece::$name(_)=> $name::compute_source(board,mov),
                    )*
                
                };
            }
        }

        $(
            #[derive(Copy,Clone,Debug)]
            pub struct $name;
        )*
    };
}

define_piece!(Pawn, Knight, Bishop, Rook, Queen, King);


impl Piece {
    pub const fn from_char(ch: char) -> Option<Piece> {
        match ch {
            'P' | 'p' => Some(Piece::Pawn(Pawn)),
            'N' | 'n' => Some(Piece::Knight(Knight)),
            'B' | 'b' => Some(Piece::Bishop(Bishop)),
            'R' | 'r' => Some(Piece::Rook(Rook)),
            'Q' | 'q' => Some(Piece::Queen(Queen)),
            'K' | 'k' => Some(Piece::King(King)),
            _ => None,
        }
    }

    pub fn to_char(&self,color:Color) -> char {
        match &self {
            Piece::Pawn(_) => Self::_pick(color, 'P', 'p'),
            Piece::Knight(_) => Self::_pick(color, 'N', 'n'),
            Piece::Bishop(_) => Self::_pick(color, 'B', 'b'),
            Piece::Rook(_) => Self::_pick(color, 'R', 'r'),
            Piece::Queen(_) => Self::_pick(color, 'Q', 'q'),
            Piece::King(_) => Self::_pick(color, 'K', 'k'),
        }
        
    }

    pub fn to_unicode(&self) -> &str {
        match &self {
            Piece::Pawn(_) => "♙",
            Piece::Knight(_) => "♘",
            Piece::Bishop(_) => "♗",
            Piece::Rook(_) => "♖",
            Piece::Queen(_) => "♕",
            Piece::King(_) => "♔",
        }
        
    }

    fn _pick(color:Color,upper:char,lower:char)->char{
        if color==Color::white{
            upper
        }else{
            lower
        }
    }

    pub fn get_all() ->Vec<Piece>{
        vec![
            Piece::Pawn(Pawn),Piece::Knight(Knight),Piece::Bishop(Bishop),Piece::Rook(Rook),Piece::Queen(Queen),Piece::King(King),
             
        ]
    }
    
}


pub trait PieceCompute {
    fn compute_source(_:&Board,_:Move) -> u8{
        todo!()
    }

    fn _compute_source(board:&Board,mov:Move, deltas:&[i8], step_only:bool) -> u8{
        let piece = mov.piece;
        let piece_bitboard = board.by_piece.get(piece);
        let color_bitboard = board.by_color.get(mov.color());
        let attack_bitboard = compute_attack_squares(mov.get_target_index().unwrap() as i8, deltas, step_only);
        let source = piece_bitboard.get() & color_bitboard.get() & attack_bitboard;
        //println!("{}",Bitboard(attack_bitboard).printable());
        //println!("{}",piece_bitboard.printable());
        //println!("{}",color_bitboard.printable());
        
        assert_eq!(source.count_ones(),1,"Bitboard \n{}", Bitboard(attack_bitboard).printable());
        return source.trailing_zeros() as u8

    }
    
    
    
}
impl PieceCompute for Pawn{
    fn compute_source(board:&Board,mov:Move) -> u8{
        let piece_bitboard = board.by_piece.get(Piece::Pawn(Pawn));
        let (file,_) = mov.get_target_file_rank();
        let color_bitboard = board.by_color.get(mov.color());
        
        let file_bitboard = file.get_bit_board();

        let attack_bitboard = if !mov.is_capture {
            file_bitboard.get()
        }else{
            let delta = if mov.color()==Color::white{
                [-7, -9]
            }else{
                [7, 9]
            };

            compute_attack_squares(mov.get_target_index().unwrap() as i8, &delta, true)
        };

        let source = piece_bitboard.get() & color_bitboard.get() & attack_bitboard;    
        
        assert_eq!(source.count_ones(),1,"Bitboard \n{}",Bitboard(attack_bitboard).printable());

        return source.trailing_zeros() as u8
    }
}
impl PieceCompute for Knight{
    fn compute_source(board:&Board,mov:Move) -> u8{
        Self::_compute_source(board, mov, &[17, 15, 10, 6, -17, -15, -10, -6], true)
    }
}

impl PieceCompute for Bishop{
    fn compute_source(board:&Board,mov:Move) -> u8{
        Self::_compute_source(board, mov, &[9, 7, -9, -7], false)
    }
}

impl PieceCompute for Rook{
    fn compute_source(board:&Board,mov:Move) -> u8{
        Self::_compute_source(board, mov, &[1,8,-1,-8], false)
    }
}
impl PieceCompute for Queen{
    fn compute_source(board:&Board,mov:Move) -> u8{
        Self::_compute_source(board, mov, &[1,8,7,9,-1,-8, -7, -9], false)
    }
}
impl PieceCompute for King{
    fn compute_source(board:&Board,mov:Move) -> u8{
        Self::_compute_source(board, mov, &[9, 8, 7, 1, -9, -8, -7, -1], true)
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

}

#[derive(Copy,Clone,Debug)]
pub enum Rank{
    FIRST=0,SECOND,THIRD,FOURTH,FIFTH,SIXTH,SEVENTH,EIGHT,
}
impl Rank {
    pub fn get_bit_board(&self)->Bitboard{
        match  self {
            Rank::FIRST =>  Bitboard(0x0000_0000_0000_00FF),
            Rank::SECOND => Bitboard(0x0000_0000_0000_FF00),
            Rank::THIRD =>  Bitboard(0x0000_0000_00FF_0000),
            Rank::FOURTH => Bitboard(0x0000_0000_FF00_0000),
            Rank::FIFTH =>  Bitboard(0x0000_00FF_0000_0000),
            Rank::SIXTH =>  Bitboard(0x0000_FF00_0000_0000),
            Rank::SEVENTH => Bitboard(0x00FF_0000_0000_0000),
            Rank::EIGHT => Bitboard(0xFF00_0000_0000_0000),
        }
    }

    pub const fn from_char(ch: char) -> Option<Rank> {
        match ch {
            '1' => Some(Rank::FIRST),
            '2' => Some(Rank::SECOND),
            '3' => Some(Rank::THIRD),
            '4' => Some(Rank::FOURTH),
            '5' => Some(Rank::FIFTH),
            '6' => Some(Rank::SIXTH),
            '7' => Some(Rank::SEVENTH),
            '8' => Some(Rank::EIGHT),
            _ => None,
        }
    }   

}