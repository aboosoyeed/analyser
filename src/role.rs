use crate::{bitboard::Bitboard, components::Piece};

pub struct ByPiece{
    pub pawn: Bitboard,
    pub knight: Bitboard,
    pub bishop: Bitboard,
    pub rook: Bitboard,
    pub queen: Bitboard,
    pub king: Bitboard,
}

impl ByPiece {
    pub fn init()->ByPiece{
        ByPiece {
            pawn: Bitboard(0x00ff_0000_0000_ff00),
            knight: Bitboard(0x4200_0000_0000_0042),
            bishop: Bitboard(0x2400_0000_0000_0024),
            rook: Bitboard(0x8100_0000_0000_0081),
            queen: Bitboard(0x0800_0000_0000_0008),
            king: Bitboard(0x1000_0000_0000_0010),
        }
    }

    pub fn get(&self, piece:Piece) ->Bitboard{
        match piece {
            Piece::Pawn(_) => self.pawn,
            Piece::Knight(_) => self.knight,
            Piece::Bishop(_) => self.bishop,
            Piece::Rook(_) => self.rook,
            Piece::Queen(_) => self.queen,
            Piece::King(_) => self.king,
             
        }
    }

    pub fn get_mut(&mut self, piece:Piece) ->&mut Bitboard{
        match piece {
            Piece::Pawn(_) => &mut self.pawn,
            Piece::Knight(_) => &mut self.knight,
            Piece::Bishop(_) => &mut self.bishop,
            Piece::Rook(_) => &mut self.rook,
            Piece::Queen(_) => &mut self.queen,
            Piece::King(_) => &mut self.king,
             
        }
    }

    

}