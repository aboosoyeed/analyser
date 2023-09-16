use crate::{board::Board, color::Color, components::Piece, r#move::Move};


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


pub fn generate(board:&Board, last_move :Move)->String{
    let pp = piece_placement(board);
    let fen = format!("{} {} {} {} {} {}",
        pp, 
        last_move.color().get_opposite(), 
        extract_castling_rights(board), 
        extract_en_passant(last_move),
        board.half_move_count,
        board.full_move_count
    );
   
    fen
}

fn piece_placement(board:&Board) ->String{
    let occupied =board.occupied.get(); 
    let mut piece_placement = String::from("");
    let mut count_empty=0;
    for i in FEN_SQUARE_INDICES{
        let mask = 1<< i;
        let v = mask & occupied;
        
        if v==0{
            count_empty +=1;    
        }else{
            if count_empty>0{
                piece_placement.push_str(count_empty.to_string().as_str());
            }
            let color = _get_color(board, mask);
            let piece = _get_piece(board, mask);
            let piece_char = piece.unwrap().to_char(color);
            piece_placement.push(piece_char);
            
            count_empty = 0
        }

        if (i+1)%8==0{
            if count_empty>0{
                piece_placement.push_str(count_empty.to_string().as_str());
            }
            if i!=7 {
                piece_placement.push('/'); // we dont want to put for the last index
            }
            
            count_empty = 0
        }

    }

    piece_placement
}

fn extract_en_passant(mov :Move)-> String{
    if mov.piece==Piece::Pawn{
        let (file,source_rank) = mov.source;
        let (_,target_rank) = mov.target;
        let diff = target_rank.unwrap() - source_rank.unwrap();
        
        if diff==2{
            let mut s = String::from("");
            s.push(file.unwrap().to_char());
            if mov.color()==Color::White{
                s.push('3')
            }else {
                s.push('6')
            }
            return s;
        }
    }
    String::from("-")
}


pub fn extract_castling_rights(board:&Board) -> String {
    let rights = ['K', 'Q', 'k', 'q'];
    let mut castling_fragment = String::new();

    for (bit, right) in (0..4).rev().zip(rights.iter()) {
        if (board.castling_rights & (1 << bit)) != 0 {
            castling_fragment.push(*right);
        }
    }

    castling_fragment.push(if castling_fragment.is_empty() { '-' } else { '\0' });
    castling_fragment
}

fn _get_color(board:&Board, mask:u64) ->Color{
    let white = board.by_color.white.get() & mask;
    
    if white==0{
        Color::Black
    }else{
        Color::White
    }

}

fn _get_piece(board:&Board, mask:u64) ->Result<Piece, &str>{
    for piece in Piece::get_all(){
        let piece_board = board.by_piece.get(piece);
        if (piece_board.get() & mask) !=0{
            return Ok(piece);
        } 
    }

    Err("Value not found")
    
}


