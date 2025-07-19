use analyzer::*;
use analyzer::{board::Board, pgn::PGN, engine::engine::Engine};
use std::{fs, io::{self, Write}};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "analyzer")]
#[command(about = "Chess game analyzer with PGN support")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze game with Stockfish engine
    Analyze {
        /// Path to PGN file
        #[arg(default_value = "./tests/pgn/3.pgn")]
        pgn_path: String,
    },
    /// Navigate through game interactively
    Navigate {
        /// Path to PGN file
        #[arg(default_value = "./tests/pgn/3.pgn")]
        pgn_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { pgn_path } => {
            analyze_game(pgn_path);
        }
        Commands::Navigate { pgn_path } => {
            navigate_game(pgn_path);
        }
    }
}

fn analyze_game(pgn_path: &str) {
    let contents = fs::read_to_string(pgn_path)
        .expect(&format!("Should have been able to read the file: {}", pgn_path));
    let mut engine = Engine::new();
    let fens = PGN::parse(contents);

    for (i, f) in fens.iter().enumerate() {
        let best_move = engine.process_fen(&f);
        println!("{}. Best move: {}", i + 1, best_move);
    }
    engine.quit();
}

fn navigate_game(pgn_path: &str) {
    let contents = fs::read_to_string(pgn_path)
        .expect(&format!("Should have been able to read the file: {}", pgn_path));
    
    let mut board = Board::init();
    let mut pgn = PGN{ 
        headers: pgn_header::PgnHeaders::new(), 
        moves: Vec::new(), 
        _move_counter: 0 
    };
    
    pgn.extract_headers(contents.clone());
    pgn.extract_moves(contents);
    
    let moves = pgn.moves.clone();
    let mut board_states = vec![board.clone()];
    let mut current_position = 0;
    
    // Apply all moves and store board states
    for mov in moves.iter() {
        board.apply_move(mov);
        board_states.push(board.clone());
    }
    
    println!("Chess Game Navigator");
    println!("Commands: 'n' (next), 'p' (previous), 'q' (quit), 'h' (help)");
    println!("Current position: {}/{}", current_position, moves.len());
    
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        // Display current board
        println!("Position: {}/{}", current_position, moves.len());
        if current_position > 0 {
            println!("Last move: {}", moves[current_position - 1].san);
        }
        println!();
        println!("{}", board_states[current_position]);
        
        print!("Enter command (n/p/q/h): ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        
        match input {
            "n" | "next" => {
                if current_position < moves.len() {
                    current_position += 1;
                } else {
                    println!("Already at the end of the game!");
                }
            }
            "p" | "prev" | "previous" => {
                if current_position > 0 {
                    current_position -= 1;
                } else {
                    println!("Already at the start of the game!");
                }
            }
            "q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "h" | "help" => {
                println!("Available commands:");
                println!("  n, next     - Move to next position");
                println!("  p, previous - Move to previous position");
                println!("  q, quit     - Exit navigator");
                println!("  h, help     - Show this help");
                println!("Press Enter to continue...");
                let mut _dummy = String::new();
                io::stdin().read_line(&mut _dummy).unwrap();
            }
            _ => {
                println!("Unknown command. Type 'h' for help.");
            }
        }
    }
}



