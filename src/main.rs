
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

mod engine{
    pub mod engine;
}
use engine::engine::Engine;
use std::fs;


fn main() {
    let contents = fs::read_to_string("./tests/pgn/2.pgn")
        .expect("Should have been able to read the file");
    let mut engine = Engine::new();
    let fens = PGN::parse(contents);

    for (i,f) in fens.iter().enumerate(){
        //println!("{}. {}",(i+1),f);
        let best_move = engine.process_fen(&f);
        println!("{}. Best move: {}", i, best_move);
    }    
    // Terminate the engine process
    engine.quit();
}



