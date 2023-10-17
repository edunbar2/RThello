use std::cmp::{max, min};
use std::fs::copy;
use std::ptr::read_unaligned;
use std::time::{Duration, Instant};

//modules
use crate::{board, constants};

// Constants
const POSITIONAL_WEIGHT: [i32; 64] = [
    30, -12,  0, -1, -1,  0, -12,  30,
    -12, -15, -3, -3, -3, -3, -15, -12,
    0,  -3,  0, -1, -1,  0,  -3,   0,
    -1,  -3, -1, -1, -1, -1,  -3,  -1,
    -1,  -3, -1, -1, -1, -1,  -3,  -1,
    0,  -3,  0, -1, -1,  0,  -3,   0,
    -12, -15, -3, -3, -3, -3, -15, -12,
    30, -12,  0, -1, -1,  0, -12,  30,
];
const CORNER_WEIGHT: i32 = 25;
const EDGE_WEIGHT: i32 = 5;
const CORNER_MASK: u64 = 0x8100000000000081;
const EDGE_MASK: u64 = 0xff818181818181ff;

pub fn get_minimax_move(board: board::Board, duration: u64) -> u64 {
    let mut best_move: u64 = 0;
    let mut last_score: i32 = std::i32::MIN;
    //generate possible game states

    //generate moves
    let possible_moves = board.get_possible_moves();
    //for each possible move
    for i in 0..64 {
        //get move bitboards
        let move_mask: u64 = possible_moves & (1 << i);
        if move_mask != 0 {
            let mut copy_board: board::Board = board.clone();
            copy_board.place_move(&possible_moves, move_mask);
            // run minimax and pick highest score
            let start_time = Instant::now();
            let duration_for_move = Duration::from_secs(duration / possible_moves.count_ones() as u64);
            let end_time = start_time + duration_for_move;

            print!("Iterating move at index: {} move {}\n", i, board::Board::convert_to_cords(move_mask));
            let current_score = minimax(copy_board, 0, true, std::i32::MIN, std::i32::MAX, end_time);
            print!("Move {} scored {} compared to previous best move {} with a score of {}\n\n", board::Board::convert_to_cords(move_mask), current_score, board::Board::convert_to_cords(best_move), last_score);
            if current_score > last_score {
                print!("new move was better, moving to best move!\n\n");
                best_move = move_mask;
                last_score = current_score;
            }
        }

    }
        print!("Playing move {} with score {}\n\n", board::Board::convert_to_cords(best_move), last_score);
        best_move
}

fn minimax(game_state: board::Board, depth: i32, maximizing_player: bool, mut alpha: i32, mut beta: i32, end_time: Instant) -> i32 {
    if Instant::now() >= end_time || board::Board::check_game_over(&game_state) {
        return evaluate(&game_state, maximizing_player, depth);
    }

    let mut eval: i32 = match maximizing_player {
        true => std::i32::MIN,
        false => std::i32::MAX,
    };
    let moves = game_state.get_possible_moves();
    // For each possible move in current state
    for i in 0..64 {
        let current_move = moves & (1u64 << i);
        // If bit is a legal move
        if current_move != 0 {
            //create a copy of the board and play move
            let mut new_state: board::Board = game_state.clone();
            new_state.place_move(&moves, current_move);
            //recursively evaluate nwe game state
            let new_eval = minimax(new_state, depth+1, !maximizing_player, alpha, beta, end_time);
            // Check if move was better than previous best move
            match maximizing_player {
                true => {
                    eval = max(new_eval, eval);
                    alpha = max(alpha, eval);
                    if beta <= alpha {
                        break;
                    }
                }
                false => {
                    eval = min(new_eval, eval);
                    beta = min(beta, eval);
                }
            };
        }
    }
    eval
}


fn evaluate(state: &board::Board, maximizing_player: bool, depth: i32) -> i32{
    let mut score: i32 = 0;
    let (player, opponent, player_color, opponent_color) = match maximizing_player {
        true => match state.current_player {
            constants::Color::Black => (state.black, state.white, constants::Color::Black, constants::Color::White),
            constants::Color::White => (state.white, state.black, constants::Color::White, constants::Color::Black),
        },
        false => match state.current_player {
            constants::Color::Black => (state.white, state.black, constants::Color::White, constants::Color::Black),
            constants::Color::White => (state.black, state.white, constants::Color::Black, constants::Color::White)
        }
    };

    // Player pieces
    let player_pieces = state.get_pieces(player_color) as i32;
    let opponent_pieces = state.get_pieces(opponent_color) as i32;
    let piece_count = player_pieces - opponent_pieces;
    score += piece_count * calculate_piece_importance(depth);
    // Winning position
    if board::Board::check_game_over(&state) {
        if player_pieces > opponent_pieces{
            score = std::i32::MAX;
        } else {
            score = std::i32::MIN;
        }
        return score;
    }

    //Mobility
    let player_moves = count_bits(state.get_possible_moves_of_color(&player_color));
    let opponent_moves = count_bits(state.get_possible_moves_of_color(&opponent_color));
    let mobility = player_moves - opponent_moves;
    score += mobility * 10;

    // Stability, Positional weight, Corners, and Edges
    let mut stability = 0;

    for i in 0..64{
        let pos: u64 = 1 << i;
        if player & pos != 0 {
            score += POSITIONAL_WEIGHT[i];
            if is_stable(&player, &opponent, &pos){
                stability +=1
            }
            if pos & CORNER_MASK != 0{
                score += CORNER_WEIGHT;
            } else if pos & EDGE_MASK != 0{
                score += EDGE_WEIGHT;
            }
        }else if opponent & pos != 0 {
            score -= POSITIONAL_WEIGHT[i];
            if is_stable(&opponent, &player, &pos) {
                stability -= 1;
            }
        }
    }
    score += stability * 20;
    // Return score
    score
}

fn count_bits(bitboard: u64) -> i32 {
    let mut count: i32 = 0;
    for i in 0..64 {
        let bit = bitboard & (1 << i);
        if bit != 0{
            count += 1;
        }
    }
    count
}

fn is_stable(color: &u64, opp_color: &u64, piece: &u64) -> bool {
    // Check if the piece is on a stable square
    if is_stable_square(piece) {
        return true;
    }

    // Check if the piece is surrounded by same-colored pieces on all sides
    let north = (piece << 8) & opp_color;
    let south = (piece >> 8) & opp_color;
    let west = (piece << 1) & 0x7f7f7f7f7f7f7f7f & opp_color;
    let east = (piece >> 1) & 0xfefefefefefefefe & opp_color;
    let nw = (piece << 9) & 0xfefefefefefefefe & opp_color;
    let ne = (piece << 7) & 0x7f7f7f7f7f7f7f00 & opp_color;
    let sw = (piece >> 7) & 0x00fefefefefefefe & opp_color;
    let se = (piece >> 9) & 0x007f7f7f7f7f7f7f & opp_color;

    if north == 0 && south == 0 && west == 0 && east == 0 && nw == 0 && ne == 0 && sw == 0 && se == 0 {
        return true;
    }

    // If neither condition is met, the piece is unstable
    false
}

fn is_stable_square(piece: &u64) -> bool {
    let corners = 0x8100000000000081;
    let edges = 0x7e0000000000007e;
    let x_squares = 0x42c300000000c342;
    let c_squares = 0x3c42424242423c00;

    // Check if the piece is on a stable square
    if (corners & piece) != 0 || (edges & piece) != 0 || (x_squares & piece) != 0 || (c_squares & piece) != 0 {
        return true;
    }

    // If not, the piece is not stable
    false
}

fn calculate_piece_importance(x: i32) -> i32 {
    print!("depth {}", x);
    let k = -20;
    let y = 40 * 2_i32.pow(((64 - x) / k) as u32);
    print!("importance: {}\n", y);
    y
}