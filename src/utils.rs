use crate::{components::{Rank, File}, bitboard::Bitboard};

/*
pub fn index_to_file_rank(index: usize) -> Option<(char, usize)> {
    if index >= 0 && index <= 63 {
        let file = ('a' as u8 + (index % 8) as u8) as char;
        let rank = (index / 8)+1;

        Some((file, rank))
    } else {
        None
    }
}
*/

pub fn compute_attack_squares(occupancy:Bitboard,init_pos:i8,deltas:&[i8], step_only:bool)->u64{
    
    let mut attack_bitboard: u64 = 0;
    for delta in deltas {
        let mut pos = init_pos; 
        loop{
            pos = pos + delta;
            let file_diff = (pos & 0x7) - (init_pos & 0x7);
            if _is_index_out_of_board(pos) || (step_only && (file_diff > 2 || file_diff < -2))   {
                break;
            }else{
                attack_bitboard |= 1 << pos;
                if _is_index_on_edge(pos) || _ray_obstructed(occupancy, step_only, pos) {
                    break
                }
            }
            if step_only{
                break;
            }
                
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

fn _is_index_on_edge(index:i8)->bool{
    index%8==0 || (index+1)%8==0 
}

pub fn square_to_index(square: &str) -> u8 {
    let (file, rank) =  square_to_file_rank(square);
    file_rank_to_index(file, rank)
}

pub fn file_rank_to_index(file:File,rank:Rank)->u8{
    ((rank as u8) ) * 8 + (file as u8)
}

pub fn square_to_file_rank(square: &str) -> (File,Rank) {
    let mut chars = square.chars();
    let file = chars.nth(0).unwrap();
    let rank = chars.nth(0).unwrap();
    (File::from_char(file).unwrap(),Rank::from_char(rank).unwrap())

}


