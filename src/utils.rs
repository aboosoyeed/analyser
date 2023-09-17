use regex::Regex;

use crate::{components::{Rank, File}, bitboard::Bitboard};

pub fn compute_attack_squares(occupancy:Bitboard,init_pos:i8,deltas:&[i8], step_only:bool)->u64{
    let mut attack_bitboard: u64 = 0;
    for delta in deltas {
        let mut last_pos = init_pos; 
        loop{
            let pos = last_pos + delta;
            let file_diff = (pos & 0x7) - (last_pos & 0x7);
            if _is_index_out_of_board(pos) || (  (file_diff > 2 || file_diff < -2))   {
                break;
            }else{
                attack_bitboard |= 1 << pos;
                if _ray_obstructed(occupancy, step_only, pos) {
                    break
                }
                
            }
            if step_only{
                break;
            }
            last_pos = pos;    
        }
        
    }

    attack_bitboard    
}

fn _is_index_out_of_board(index:i8)->bool{
    index < 0 || index>63
}

fn _ray_obstructed(occupancy:Bitboard, step_only:bool, pos:i8) ->bool{
    !step_only && occupancy.get_bit((pos as i8).try_into().unwrap())
}


pub fn index_to_file_rank(index: u8) -> (Option<File>, Option<Rank>) {
    let file = ('a' as u8 + (index % 8) as u8) as char;
    let rank = (index / 8)+1;
    let rank_char = rank.to_string().chars().next().unwrap();
    (File::from_char(file),Rank::from_char( rank_char))
}

pub fn file_rank_to_index(file:File,rank:Rank)->u8{
    ((rank as u8) ) * 8 + (file as u8)
}

pub fn is_piece(c: char) -> bool {
    match c {
        'K' | 'N' | 'B' | 'Q' | 'R' => true,
        _ => false,
    }
}

pub fn get_header_regex()->Regex{
     Regex::new(r"(\[.*?\])").unwrap()
}


pub fn color_str(str: &str, color:&str)->String{
    
    let code = if color=="yellow"{
        "33m"
    } else if color=="blue"{
        "34m"
    } else if color=="gray"{
        "30m"
    }
    else{
        "37m"
    };
    
    format!("\x1b[{}{}\x1b[0m",code,str )
}
