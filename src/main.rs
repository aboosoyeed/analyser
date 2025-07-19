use analyzer::*;
use analyzer::{board::Board, pgn::Pgn, engine::engine::Engine, constants::defaults};
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
        #[arg(default_value_t = defaults::DEFAULT_PGN_PATH.to_string())]
        pgn_path: String,
    },
    /// Navigate through game interactively
    Navigate {
        /// Path to PGN file
        #[arg(default_value_t = defaults::DEFAULT_PGN_PATH.to_string())]
        pgn_path: String,
    },
}

#[derive(Debug)]
enum Command {
    Next,
    Previous,
    Quit,
    Help,
    Empty,
}

fn parse_command(input: &str) -> Result<Command, String> {
    match input.trim().to_lowercase().as_str() {
        "n" | "next" => Ok(Command::Next),
        "p" | "prev" | "previous" => Ok(Command::Previous),
        "q" | "quit" => Ok(Command::Quit),
        "h" | "help" => Ok(Command::Help),
        "" => Ok(Command::Empty),
        unknown => Err(format!("[Chess Analyzer] Input error: Unknown command '{}'. Type 'h' for help.", unknown))
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Analyze { pgn_path } => {
            if let Err(error) = analyze_game(pgn_path) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
        Commands::Navigate { pgn_path } => {
            if let Err(error) = navigate_game(pgn_path) {
                eprintln!("{}", error);
                std::process::exit(1);
            }
        }
    }
}

fn analyze_game(pgn_path: &str) -> Result<(), String> {
    let contents = fs::read_to_string(pgn_path)
        .map_err(|e| format!("[Chess Analyzer] File error: Could not read file '{}': {}", pgn_path, e))?;
    let mut engine = Engine::new();
    let fens = Pgn::parse(contents);

    for (i, f) in fens.iter().enumerate() {
        let best_move = engine.process_fen(&f);
        println!("{}. Best move: {}", i + 1, best_move);
    }
    engine.quit();
    Ok(())
}

fn navigate_game(pgn_path: &str) -> Result<(), String> {
    let contents = fs::read_to_string(pgn_path)
        .map_err(|e| format!("[Chess Analyzer] File error: Could not read file '{}': {}", pgn_path, e))?;
    
    let mut pgn = Pgn{ 
        headers: pgn_header::PgnHeaders::new(), 
        moves: Vec::new(), 
        _move_counter: 0 
    };
    
    pgn.extract_headers(contents.clone());
    pgn.extract_moves(contents);
    
    let moves = &pgn.moves; // Use reference instead of clone
    let mut current_position = 0;
    
    println!("Chess Game Navigator");
    println!("Commands: 'n' (next), 'p' (previous), 'q' (quit), 'h' (help)");
    println!("Current position: {}/{}", current_position, moves.len());
    
    loop {
        // Clear screen
        print!("\x1B[2J\x1B[1;1H");
        
        // Generate current board state on-demand
        let mut current_board = Board::init();
        for i in 0..current_position {
            current_board.apply_move(&moves[i]);
        }
        
        // Display current board
        println!("Position: {}/{}", current_position, moves.len());
        if current_position > 0 {
            println!("Last move: {}", moves[current_position - 1].san);
        }
        println!();
        println!("{}", current_board);
        
        print!("Enter command (n/p/q/h): ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("[Chess Analyzer] Warning: Failed to flush output: {}", e);
        }
        
        let mut input = String::new();
        if let Err(e) = io::stdin().read_line(&mut input) {
            eprintln!("[Chess Analyzer] Input error: Failed to read input: {}", e);
            break;
        }
        let input = input.trim();
        
        match parse_command(input) {
            Ok(Command::Next) => {
                if current_position < moves.len() {
                    current_position += 1;
                } else {
                    println!("[Chess Analyzer] Navigation: Already at the end of the game!");
                }
            }
            Ok(Command::Previous) => {
                if current_position > 0 {
                    current_position -= 1;
                } else {
                    println!("[Chess Analyzer] Navigation: Already at the start of the game!");
                }
            }
            Ok(Command::Quit) => {
                println!("[Chess Analyzer] Goodbye!");
                break;
            }
            Ok(Command::Help) => {
                println!("[Chess Analyzer] Available commands:");
                println!("  n, next     - Move to next position");
                println!("  p, previous - Move to previous position");
                println!("  q, quit     - Exit navigator");
                println!("  h, help     - Show this help");
                println!("Press Enter to continue...");
                let mut _dummy = String::new();
                if let Err(e) = io::stdin().read_line(&mut _dummy) {
                    eprintln!("[Chess Analyzer] Input error: Failed to read input: {}", e);
                }
            }
            Ok(Command::Empty) => {
                // Do nothing for empty input
            }
            Err(error_msg) => {
                println!("{}", error_msg);
            }
        }
    }
    Ok(())
}



