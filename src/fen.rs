use crate::{board::Board, color::Color, components::{Piece, Pawn}, bitboard::Bitboard};


const FEN_SQUARE_INDICES: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
    8,  9, 10, 11, 12, 13, 14, 15,
    0,  1,  2,  3,  4,  5,  6,  7,
];


pub fn generate(board:&Board)->String{
    let occupied =board.occupied.get(); 
    let mut fen = String::from("");
    let mut count_empty=0;
    for i in FEN_SQUARE_INDICES{
        let mask = 1<< i;
        let v = mask & occupied;
        
        if v==0{
            count_empty +=1;    
        }else{
            if count_empty>0{
                fen.push_str(count_empty.to_string().as_str());
            }
            let color = _get_color(board, mask);
            let piece = _get_piece(board, mask);
            let piece_char = piece.unwrap().to_char(color);
            fen.push(piece_char);
            
            count_empty = 0
        }

        if (i+1)%8==0{
            if count_empty>0{
                fen.push_str(count_empty.to_string().as_str());
            }
            if i!=7 {
                fen.push('/'); // we dont want to put for the last index
            }
            
            count_empty = 0
        }

    }

    fen
    
}

fn _get_color(board:&Board, mask:u64) ->Color{
    let white = board.by_color.white.get() & mask;
    //let black: u64 = board.by_color.black.get() & mask;
    if white==0{
        Color::black
    }else{
        Color::white
    }

}

fn _get_piece(board:&Board, mask:u64) ->Result<Piece, &str>{
    for piece in Piece::get_all(){
        let piece_board = board.by_piece.get(piece);
        if (piece_board.get() & mask) !=0{
            return Ok(piece);
        } 
    }

    println!("{}",Bitboard(mask).printable());

    let piece_board = board.by_piece.get(Piece::Pawn(Pawn));

    println!("{}",piece_board.printable());


    println!("{}", board);

    Err("Value not found")
    
}


