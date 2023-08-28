use std::{fs, thread, time::Duration};
use pgn_reader::{Visitor, BufferedReader, SanPlus};
use shakmaty::{Chess, Position};
mod engine{
    pub mod engine;
}

use engine::engine::Engine;



struct PgnParser {
    moves: Vec<String>,
    pos: Chess
}

impl PgnParser {
    fn new() -> PgnParser {
        PgnParser { moves: Vec::new() , pos: Chess::default() }
    }
}

impl Visitor for PgnParser {
    type Result = usize;

    fn begin_game(&mut self) {
        self.moves = Vec::new();
    }

    fn san(&mut self, _san_plus: SanPlus) {
        if let Ok(m) = _san_plus.san.to_move(&self.pos) {
            self.pos.play_unchecked(&m);
            self.moves.push( self.pos.board().to_string() );
        }
    }

    fn end_game(&mut self) -> Self::Result {
       self.moves.len()
    }

    

    
}

fn generate_moves() -> Vec<String> {
    let pgn = read_pgn();

    let mut reader = BufferedReader::new_cursor(&pgn[..]);

    let mut counter = PgnParser::new();
    let _moves = reader.read_game(&mut counter);

    counter.moves
    
}



fn main() {
    
    let moves = generate_moves();
    println!("No of moves {:?}",moves.len());
    
    // Start Stockfish engine process
    let mut engine = Engine::new();
    
    for (index,fen) in moves.iter().enumerate(){
        // Receive and print the best move from Stockfish
        println!("{}",fen);
        let best_move = engine.process_fen(&fen);
        println!("{}. Best move: {}", index, best_move);
        
    }
    
    
    // Terminate the engine process
    engine.quit();
    
}



fn read_pgn()->String{
    let contents = fs::read_to_string("./test.pgn")
        .expect("Should have been able to read the file");

    return contents;
}


