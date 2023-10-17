use crate::{board, constants};

mod monte_carlo_tree_search;
mod minimax;

mod random_ai;

pub enum AI_Type{
    Random,
    Minimax,
    MCTS
}

pub struct AI_Logic_Handler
{
    AI: AI_Type,
    Color: constants::Color
}

impl AI_Logic_Handler {
    pub fn new(Color: constants::Color, ai_difficulty: constants::Difficulty) -> Self {
        let AI= match ai_difficulty {
            constants::Difficulty::Easy => AI_Type::Random,
            constants::Difficulty::Normal => AI_Type::Minimax,
            constants::Difficulty::Hard => AI_Type::MCTS,
            _ => AI_Type::Minimax
        };
        Self {AI, Color}
    }

    pub fn get_ai_move(&self, &possible_moves: &u64, board: board::Board) -> u64
    {
        match self.AI {
            AI_Type::Random => {
                random_ai::get_random_move(possible_moves)
            }
            AI_Type::Minimax => {
                 minimax::get_minimax_move(board, 30)
            }
            AI_Type::MCTS => {
                // TODO Implement MCTS Algorithm
                0

            }
        }
    }
}
