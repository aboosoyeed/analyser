
mod bitboard;
mod fen;
mod pgn;
mod pgn_header;
mod board;
mod role;
mod color;
mod r#move;
mod components;
mod utils;
use pgn::PGN;




fn main() {

    
    let fens = PGN::parse("./test.pgn");

    for (i,f) in fens.iter().enumerate(){
        println!("{}. {}",(i+1),f)
    }    
    
}



