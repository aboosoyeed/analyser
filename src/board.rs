use core::fmt;

use crate::{bitboard::Bitboard, role::ByPiece, color::{ByColor, self, Color}, move_::Move, fen::generate, components::{Piece, King, Rook},};

pub struct Board{
    pub by_piece: ByPiece,
    pub by_color: ByColor,
    pub occupied: Bitboard,
}

impl Board {
    pub fn init()->Board{
        Board { 
            by_piece: ByPiece::init(), 
            by_color: ByColor::init(), 
            occupied: Bitboard(0xffff_0000_0000_ffff) 
        }
    }

    pub fn apply_move(&mut self, mov : Move){
        //println!("{:?}",target);
        if mov.index < 14 {
            if mov.castling.is_some() {
                self.apply_castling(mov)
            }else{
                self.apply_normal_move(mov)
            }
            
            
            //println!("{}",piece_board.printable())    

            //println!("{}",self.generate_fen())
        }
    }

    fn apply_castling(&mut self, mov:Move){
        let color = mov.color();
        let ((ks,kt),(rs,rt)) = mov.castling.unwrap().compute_squares(color);
        self.move_piece(ks, kt, color, Piece::King(King));
        self.move_piece(rs, rt, color, Piece::Rook(Rook));
    }

    fn apply_normal_move(&mut self, mov:Move){
        let target = mov.get_target_index();
        let color = mov.color();
        let piece = mov.piece;
            
        let source = self.get_source_index(mov);
            
        self.move_piece(source, target.unwrap(), color, piece)
    }
    
    fn move_piece(&mut self, source:u8, target:u8, color:Color, piece:Piece){
        self.occupied.toggle(source,target);
            
        let color_board = &mut self.by_color.get_mut(color);
        color_board.toggle(source,target);
        
        let piece_board = &mut self.by_piece.get_mut(piece);
        piece_board.toggle(source,target);
    }

    pub fn generate_fen(&self)->String{
        generate(self)
    }

    

    pub fn get_source_index(&self, mov : Move) ->u8{
        let piece = mov.piece;
        piece.compute_source(self,mov)
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.occupied.printable())
    }
}

