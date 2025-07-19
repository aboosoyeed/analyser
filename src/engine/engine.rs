use std::io::{BufRead, BufReader, Write};
use std::process::{Child, Command, Stdio};
use crate::constants::defaults;

pub struct Engine {
    engine: Child,
    depth:usize
}

impl Engine{
    pub fn new() -> Self{
        let engine = Self::start_engine().expect("Failed to start");
        
        let mut instance = Self{
            engine,
            depth: defaults::DEFAULT_ENGINE_DEPTH as usize
        };
        instance.init();
        instance
    }

    fn start_engine() -> std::io::Result<Child> {
        Command::new("stockfish")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
    }

    fn init(&mut self){
        // Send UCI commands and receive responses
        self.send_uci_command("uci");
        self.send_uci_command("isready");
        //engine.send_uci_command("ucinewgame");
        self.send_uci_command("setoption name MultiPV value 3");
    }

    pub fn send_uci_command(&mut self, command: &str) {
        let stdin = self.engine.stdin.as_mut().expect("Failed to open stdin");
        
        writeln!(stdin, "{}", command).expect(&format!("Failed to write command {}",command));
    }

    pub fn receive_best_move(&mut self) -> String {
        let stdout = self.engine.stdout.as_mut().expect("Failed to open stdout");
        let reader = BufReader::new(stdout);

        for line in reader.lines() {
            let line = line.expect("Failed to read line");
            //println!("{}",line);
            if line.starts_with("bestmove") {
                println!("{}",line);
                let parts: Vec<&str> = line.split_whitespace().collect();
                if let Some(best_move) = parts.get(1) {
                    return best_move.to_string();
                }
            }
        }

        String::new()
    }

    pub fn process_fen(&mut self,fen: &str)->String{

        self.send_uci_command(&format!("position fen {}",fen));
        self.send_uci_command( &format!("go depth {}",self.depth));

        // Receive and print the best move from Stockfish
        let best_move = self.receive_best_move();
        best_move
    }

    pub fn quit(&mut self) {
        self.send_uci_command("quit");
        self.engine.wait().expect("Failed to wait for Stockfish");
    }

}