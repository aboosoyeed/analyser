
mod bitboard;
mod fen;
mod pgn;
mod pgn_header;
mod board;
mod role;
mod color;
mod move_;
mod components;
mod utils;
use pgn::PGN;


use crate::board::Board;


fn main() {

    
    let mut pgn = PGN::new();  
    pgn.parse("./test.pgn");
    //pgn.print_headers();
    let mut board = Board::init();
    for mov in pgn.moves{
        board.apply_move(mov);
        
    }
    println!("{}",board)
    
    
}



