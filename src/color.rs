use crate::bitboard::Bitboard;

#[derive(Clone, Copy,PartialEq)]
pub enum  Color{
    White,
    Black
}

impl Color {
    pub fn get_opposite(&self)->Color{
        if *self==Color::White{
            Color::Black
        }else{
            Color::White
        }
    }
}


pub struct ByColor {
    pub black: Bitboard,
    pub white: Bitboard,
}

impl ByColor {
    pub fn init()->ByColor{
        ByColor {
            black: Bitboard(0xffff_0000_0000_0000),
            white: Bitboard(0xffff),
        }
    }

    pub fn get(&self,color:Color)->Bitboard{
        match color {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }

    pub fn get_mut(&mut self, color:Color) -> &mut Bitboard {
        match color {
            Color::White => &mut self.white,
            Color::Black => &mut self.black,
        }
    }
}