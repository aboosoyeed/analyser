
#[derive(Copy,Clone,Debug)]
pub struct Bitboard(pub u64);

impl Bitboard {
    
    pub fn get(&self)->u64 {
        self.0
    }

    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1 << index;
    }

    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1 << index);
    }

    pub fn get_bit(&self, index: u8) -> bool {
        (self.0 & (1 << index)) != 0
    }

    pub fn toggle(&mut self, source: u8, target:u8) {
        self.set_bit(target);
        self.clear_bit(source);
            
    }
    pub fn printable(&self) ->String{
        let mut fmt_str= String::from("");
        for rank in (0..8).rev() {
            for file in 0..8 {
                let index = rank * 8 + file;
                let bit = if self.get_bit(index) { '1' } else { '0' };
                fmt_str.push_str(&format!("{} ", bit)) ;
            }
            fmt_str.push_str("\n\r");
        }

        return fmt_str;
    }

    

}
