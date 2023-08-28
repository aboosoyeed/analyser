use crate::bitboard::Bitboard;

#[derive(Clone, Copy,PartialEq)]
pub enum  Color{
    white,
    black
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
            Color::white => self.white,
            Color::black => self.black,
        }
    }

    pub fn get_mut(&mut self, color:Color) -> &mut Bitboard {
        match color {
            Color::white => &mut self.white,
            Color::black => &mut self.black,
        }
    }
}