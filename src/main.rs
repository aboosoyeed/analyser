
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

fn main() {

    let mut engine = Engine::new();
    let fens = PGN::parse("./test.pgn");

    for (i,f) in fens.iter().enumerate(){
        //println!("{}. {}",(i+1),f);
        let best_move = engine.process_fen(&f);
        println!("{}. Best move: {}", i, best_move);
    }    
    // Terminate the engine process
    engine.quit();
}



