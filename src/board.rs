use core::fmt;

use crate::{bitboard::Bitboard, role::ByPiece, color::{ByColor, Color}, r#move::Move, fen::generate, components::Piece,};

pub struct Board{
    pub by_piece: ByPiece,
    pub by_color: ByColor,
    pub occupied: Bitboard,
    pub castling_rights:u8,
    pub half_move_count:u8,
    pub full_move_count:u16
}

impl Board {
    pub fn init()->Board{
        Board { 
            by_piece: ByPiece::init(), 
            by_color: ByColor::init(), 
            occupied: Bitboard(0xffff_0000_0000_ffff),
            castling_rights: 0b_1111 ,
            half_move_count:0,
            full_move_count:1,
        }
    }

    pub fn apply_move(&mut self, mov : Move)->Option<u8>{
        let mut source:Option<u8> = None;
        
        if mov.is_capture || mov.piece=={Piece::Pawn} {
            self.half_move_count = 0
        }else{
            self.half_move_count += 1;
        }

        if mov.color()==Color::Black{
            self.full_move_count += 1;
        }


        if mov.castling.is_some() {
            self.apply_castling(mov)
        }else{
            source = Some(self.apply_normal_move(mov));
        }
        source
    }

    fn apply_castling(&mut self, mov:Move){
        let color = mov.color();
        let castling = mov.castling ;
        let ((ks,kt),(rs,rt)) = castling.unwrap().compute_squares(color);
        self.move_piece(ks, kt, color, Piece::King);
        self.move_piece(rs, rt, color, Piece::Rook);
        
        // remove all castling rights for the side
        self.remove_castling_rights(color, true);
        self.remove_castling_rights(color, false);
    }

    fn remove_castling_rights(&mut self, color:Color,is_king_side:bool){
        let mask = match (color, is_king_side) {
            (Color::White, true) => 0b_0111,
            (Color::White, false) => 0b_1011,
            (Color::Black, true) => 0b_1101,
            (Color::Black, false) => 0b_1110,
        };
    
        self.castling_rights = self.castling_rights & mask;
    }

    fn apply_normal_move(&mut self, mov:Move) ->u8{
        let target = mov.get_target_index();
        let color = mov.color();
        let piece = mov.piece;
        let is_capture = mov.is_capture;
        let source = self.get_source_index(mov);
            
        if is_capture{
            let opp_color_board = &mut self.by_color.get_mut(color.get_opposite());
            opp_color_board.clear_bit(target.unwrap());
            let mut opp_piece = self.get_piece_at_index(target.unwrap());
            
            // potentially enpassant
            if opp_piece==Err("piece_not_found") && piece==Piece::Pawn{
                let revised_target = if color==Color::White{
                    target.unwrap()-8
                }else{
                    target.unwrap()+8
                };
                opp_piece = self.get_piece_at_index(revised_target);
            }
            
            let opp_piece_board = &mut self.by_piece.get_mut(opp_piece.unwrap());
            opp_piece_board.clear_bit(target.unwrap())
        }

        if piece==Piece::Rook && (source==0 || source==7 || source==56 || source==63  ){

            self.remove_castling_rights(color, source==7 || source==63);
                
        }

        if piece==Piece::King && (source==4 || source==60){
            // remove all castling rights for the side
            self.remove_castling_rights(color, true);
            self.remove_castling_rights(color, false);
        }

        self.move_piece(source, target.unwrap(), color, piece);
        source
        
    }
    
    fn move_piece(&mut self, source:u8, target:u8, color:Color, piece:Piece){
        self.occupied.toggle(source,target);
            
        let color_board = &mut self.by_color.get_mut(color);
        color_board.toggle(source,target);
        
        let piece_board = &mut self.by_piece.get_mut(piece);
        piece_board.toggle(source,target);
    }

    pub fn generate_fen(&self, last_move:Move)->String{
        generate(self, last_move)
    }

    

    pub fn get_source_index(&self, mov : Move) ->u8{
        let piece = mov.piece;
        piece.compute_source(self,mov)
    }

    fn get_piece_at_index(&self,index:u8) ->Result<Piece, &'static str>{
        let mask = 1<< index;
        for piece in Piece::get_all(){
            let piece_board = self.by_piece.get(piece);
            if (piece_board.get() & mask) !=0{
                return Ok(piece);
            } 
        }
        Err("piece_not_found")
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        let mut fmt_str= String::from("");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                let bit = if self.occupied.get_bit(index) { 
                    let color = if self.by_color.white.get_bit(index){
                        "yellow"
                    }else{
                        "blue"
                    };
                    color_str(&self.get_piece_at_index(index).unwrap().to_unicode().to_string(), color) 
                }else {
                    color_str("◻", "gray") 
                };
                fmt_str.push_str(&format!("{} ", bit)) ;
            }
            fmt_str.push_str("\n\r");
        }

        
        
        f.write_str(&fmt_str)

    }
}



fn color_str(str: &str, color:&str)->String{
    
    let code = if color=="yellow"{
        "33m"
    } else if color=="blue"{
        "34m"
    } else if color=="gray"{
        "30m"
    }
    else{
        "37m"
    };
    
    format!("\x1b[{}{}\x1b[0m",code,str )
}

