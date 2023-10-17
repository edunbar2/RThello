use rand::Rng;

pub fn get_random_move(possible_moves: u64) -> u64 {
    // Count positions in possible_moves and place the indexes in an array
    let mut move_positions: Vec<usize> = Vec::new();
    let mut bit_mask = 1u64;
    for index in 0..64 {
        // get position at I to see if it is a move
        if(possible_moves & bit_mask) != 0 {
            move_positions.push(index);
        }
        bit_mask <<= 1;

    }
    let random_index = rand::thread_rng().gen_range(0..move_positions.len());
    1 << move_positions[random_index]
}