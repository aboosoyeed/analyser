mod pgn;
mod bitboard;
mod fen;

mod pgn_header;
mod board;
mod role;
mod color;
mod r#move;
mod components;
mod utils;

mod engine{
    pub mod engine;
}
use engine::engine::Engine;
use std::fs;
use pgn::PGN;

fn main() {
    let contents = fs::read_to_string("./tests/pgn/3.pgn")
        .expect("Should have been able to read the file");
    let mut engine = Engine::new();
    let mut pgn = PGN::new(contents);
    let fens = pgn.parse();
    for (i,f) in fens.iter().enumerate(){
        //println!("{}. {}",(i+1),f);
        let best_move = engine.process_fen(&f);
        println!("{}. Best move: {}", i, best_move);
    }    
    // Terminate the engine process
    engine.quit();
}



