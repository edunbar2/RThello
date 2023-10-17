//Imports
use std::io;

//Modules
use crate::constants;
use crate::ai_logic;
use crate::ai_logic::AI_Logic_Handler;

#[derive(Clone)]
pub struct Board {
    pub(crate) black: u64,
    pub(crate) white: u64,
    pub(crate) current_player: constants::Color
}

enum Mode{
    AI,
    Player
}
impl Board{
    /// Initializes the variables in Board and returns the object.
    ///
    /// # Arguments
    ///
    ///
    ///
    ///  # Examples
    ///
    /// ```
    /// let mut board:Board = Board::new();
    /// ```
    pub fn new() ->Self {
        let black = constants::STARTING_BLACK;
        let white = constants::STARTING_WHITE;
        let current_player = constants::STARTING_PLAYER;
        Self {black, white, current_player}
    }

    pub fn get_possible_moves(&self) -> u64 {
        let (player, opponent) = self.get_bitboards();

        let empty_squares: u64 = !(self.black ^ self.white);
        let mut moves: u64 = 0;

        for dir in constants::DIRECTIONS {
            let mut candidates: u64 = opponent & if dir <= 9 { player >> dir } else { player << (dir % 10) };
            while candidates != 0 {
                moves |= empty_squares & if dir <= 9 { candidates >> dir } else { candidates << (dir % 10) };
                candidates = opponent & if dir <= 9 { candidates >> dir } else { candidates << (dir % 10) };
            }
        }
        moves
    }

    pub fn get_possible_moves_of_color(&self, color: &constants::Color) -> u64 {
        let (player, opponent) = match color {
            constants::Color::Black => (self.black, self.white),
            constants::Color::White => (self.white, self.black)
        };
        // generate bitboard of all empty squares
        let empty_squares: u64 = !(self.black ^ self.white);
        let mut moves: u64 = 0;
        //check each direction for legal moves
        for dir in constants::DIRECTIONS {
            let mut candidates: u64 = opponent & if dir <= 9 { player >> dir } else { player << (dir % 10) };
            while candidates != 0 {
                moves |= empty_squares & if dir <= 9 { candidates >> dir } else { candidates << (dir % 10) };
                candidates = opponent & if dir <= 9 { candidates >> dir } else { candidates << (dir % 10) };
            }
        }
        moves
    }

    ///Places the current move on the board and handles flipping affected tiles
    /// Returns a boolean showing whether the move was made or not
    ///
    ///  # Arguments
    ///
    ///  * 'possible_moves' - The bitboard containing positions of all legal moves
    ///  * 'picked_move' - The position of the picked move as a bitboard
    ///
    ///  # Examples
    ///
    /// ```
    /// Let board = Board::new();
    /// let moves = board.get_possible_moves();
    /// let (row, col) = get_input("What move do you want to make?");
    /// let pos = get_pos(row, col);
    /// board.place_piece(moves, pos);
    /// ```
    pub fn place_move(&mut self, possible_moves: &u64, selected_move: u64) -> bool {
        //invalid move
        if (possible_moves & selected_move == 0) || selected_move == 0 {
           return false;
        }

        let (mut player, mut opponent) = self.get_bitboards();
        //place move on corresponding board
        player |= selected_move;

        //find valid tiles to flip
        let empty_squares: u64 = !(self.black ^ self.white);
        let mut flip_mask: u64 = 0;
        for dir in constants::DIRECTIONS {
            // Get  enemy tile at that direction
            let mut direction_tile = if dir <= 9 { selected_move << dir } else { selected_move >> (dir % 10) } & opponent;
            // print_as_board(&direction_tile);// Testing purposes
            let mut current_mask: u64 = 0;
            // Loop travel in that direction until hits empty space or player tile
            // print_as_board(&player);// Testing purposes
            // print_as_board(&opponent);// Testing purposes
            while direction_tile != 0 {
                // Add current bit to
                current_mask |= direction_tile;
                direction_tile = if dir <= 9 { direction_tile << dir } else { direction_tile >> (dir % 10) };
                // print_as_board(&direction_tile); // Testing purposes
                if direction_tile & player != 0 {
                    flip_mask |= current_mask;
                    break;
                } else if direction_tile & opponent == 0 {
                    direction_tile = 0;
                }
            }
            if direction_tile & player > 0 {
                flip_mask |= current_mask;
            }
        }
        //apply flip to both bitboards
        player ^= flip_mask;
        opponent ^= flip_mask;
        // Update Active Bitboards and current_player
        match self.current_player {
            constants::Color::Black => {
                self.black = player;
                self.white = opponent;
                self.current_player = constants::Color::White;
            }
            constants::Color::White => {
                self.white = player;
                self.black = opponent;
                self.current_player = constants::Color::Black;
            }
        };
        // Move was made successfully
        true
    }
    ///Returns the number of pieces owned by the specified player
    ///
    ///  # Arguments
    ///
    ///  * 'color' - The color of pieces to count
    ///
    /// # Examples
    ///
    ///  ```
    /// let black_pieces = board.get_pieces(Color::Black);
    ///  ```
    pub fn get_pieces(&self, color: constants::Color) -> u8 {
        let mut count: u8 = 0;
        let pieces: u64 = match color {
            constants::Color::Black => self.black,
            constants::Color::White => self.white,
        };

        for i in 1..64 {
            let bit: u64 = pieces & (1 << i);
            if bit > 0 {
                count += 1;
            }
        }
        count
    }
    ///Returns the bitboards in current player / opponent order
    ///
    ///  # Arguments
    ///
    /// # Examples
    ///
    ///  ```
    /// let (player, opponent) = board.get_bitboards();
    ///  ```
    fn get_bitboards(&self) -> (u64, u64) {
        //return the bitboards with the current player first
        match self.current_player {
            constants::Color::Black => (self.black, self.white),
            constants::Color::White => (self.white, self.black)
        }
    }

    // ----- Helper Functions ----- //

    ///Prints the current game-state of the board given.
    /// This includes possible moves for the current player
    ///
    ///  # Arguments
    ///
    /// * 'board' - A reference to the Board containing the game-state to print
    /// * 'moves' - A reference to a bitboard containing current legal moves
    ///
    ///  # Examples
    ///
    /// ```
    /// Let board = Board::new();
    /// let moves = board.get_possible_moves();
    /// print_board(board, moves);
    /// ```
    pub fn print_board(board: &Board, moves: &u64) {
        print!("X\tA\tB\tC\tD\tE\tF\tG\tH\n1\t");
        for i in 0..64 {
            // print!("{}", i); // Used for testing.
            let white_bit = board.white & (1 << i);
            let black_bit = board.black & (1 << i);
            if white_bit >= 1 {
                print!("W\t");
            } else if black_bit >= 1 {
                print!("B\t");
            } else if moves & (1 << i) > 0 {
                print!("X\t");
            } else {
                print!("-\t");
            }

            if i % 8 == 7 && i > 0 && i < 60 {
                print!("\n{}\t", i / 8 + 2);
            }
        }
        print!("\nBlack Pieces: {}\tWhite Pieces {}\n", board.get_pieces(constants::Color::Black), board.get_pieces(constants::Color::White))
    }


    ///This function takes in any single bitboard and prints it as a game-board.
    /// All bits are printed as either '-' or 'X' depending on if the tile is blank or filled respectively.
    /// This is used primarily for testing purposes
    ///
    ///  # Arguments
    ///
    ///  * 'board' - The bitboard containing the values to print
    pub fn print_as_board(board: &u64) {
        print!("X\tA\tB\tC\tD\tE\tF\tG\tH\n1\t");
        for i in 0..64 {
            // print!("{}", i); // Used for testing.

            let bit = board & (1 << i);
            if bit >= 1 {
                print!("X\t");
            } else {
                print!("-\t");
            }

            if i % 8 == 7 && i > 0 && i < 60 {
                print!("\n{}\t", i / 8 + 2);
            }
        }
        print!("\n")
    }

    /// Takes in the row and column of a move and converts it to a bitboard position
    ///
    ///  # Arguments
    ///
    ///  * 'row' - The row of the move
    ///  * 'col' - The column of the move
    ///
    ///  # Examples
    ///
    /// ```
    /// let (row, col) = get_input("What move do you want to make?");
    /// let pos = match got_pos(row, col) {
    ///     Ok(value) => value,
    ///     Err(error) => print!(error),
    /// }
    /// ```
    pub fn get_pos(row: usize, col: usize) -> Result<u64, u64> {
        if (row < 0 || row > 7) || (col < 0 || col > 7) {
            return Err(u64::MAX);
        };
        Ok(1 << (row * constants::BOARD_SIZE + col) as u64)
    }

    pub fn convert_to_cords(bitboard: u64) -> String{
        let move_index = bitboard.trailing_zeros() as usize;
        let row = (move_index / 8) as u8;
        let col = (move_index % 8) as u8;
        let col_char = (col + b'a') as char;
        let row_char = (row + b'1') as char;
        format!("{} {}", col_char, row_char)
    }

    ///Checks whether win conditions have been met. If so, return true, else false
    ///
    ///  # Arguments
    ///
    ///  # Examples
    ///
    ///  ```
    ///  let end_game = check_game_over();
    /// if end game {
    ///     break;
    /// }
    ///  ```
    pub fn check_game_over(board: &Board) -> bool {
        //if no empty pieces on board or black has no pieces or white has no pieces, end game
        let black_pieces = board.get_pieces(constants::Color::Black);
        let white_pieces = board.get_pieces(constants::Color::White);
        if !(board.black ^ board.white) == 0 || black_pieces == 0 || white_pieces == 0 {
            return true;
        }

        if board.get_possible_moves_of_color(&constants::Color::Black) == 0 && board.get_possible_moves_of_color(&constants::Color::White) == 0 {
            return true;
        }

        false
    }

    pub fn get_input(prompt: String, ) -> String {
        let mut input = String::from("");
        loop {
            print!("{}\n", prompt);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            return input.trim().to_uppercase();
        }
    }

    /// Gets desired move from user as a string and converts it to
    /// two unsigned 64 bit integers.
    ///
    ///  # Arguments
    ///
    /// * 'prompt' - the prompt to give to the user
    ///
    ///  # Examples
    ///
    /// ```
    /// let (row, col) = get_input("What move do you want to make?");
    ///```
    pub fn get_user_move() -> (usize, usize) {
        let prompt:String = String::from("Please input your move: (column-letter, row-number)");
        let mut input = String::from("");
        let mut col: usize;
        let mut row: usize;
        // Get and parse move from player
        loop {
            print!("{}\n", prompt);
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input = input.trim().to_uppercase();

            let (col_char, row_str) = input.split_at(1);
            // Convert column character to integer value
            col = match col_char {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                "D" => 3,
                "E" => 4,
                "F" => 5,
                "G" => 6,
                "H" => 7,
                _ => {
                    print!("Invalid Column\n");
                    input = String::from("");
                    continue;
                }
            };
            // Convert row character to integer value
            row = match row_str.trim().parse::<usize>() {
                Ok(n) => n - 1,
                Err(_) => {
                    print!("Invalid Row\n");
                    input = String::from("");
                    continue;
                }
            };
            return (row, col);
        }
        }
    }

    pub fn run_game() {
        // Define and initialize objects

        // Define Board
        let mut game_board: Board = Board::new();
        // Define Player Color
        let player_return_type = Board::get_input(String::from("What color do you want to play?"));
        let player: constants::Color = match player_return_type.to_lowercase().as_str() {
            "black" => constants::Color::Black,
            "white" => constants::Color::White,
            _ => constants::Color::Black,
        };

        // Define AI difficulty
        let difficulty_string = Board::get_input(String::from("What difficulty do you want to play? (easy, normal, hard)"));
        let difficulty = match difficulty_string.to_lowercase().as_str() {
            "easy" => constants::Difficulty::Easy, // random moves
            "normal" => constants::Difficulty::Normal, // minimax algorithm
            "hard" => constants::Difficulty::Hard, //monte carlo tree search
            _ => constants::Difficulty::Normal,
        };

        let ai_color = match player {
            constants::Color::Black => constants::Color::White,
            constants::Color::White => constants::Color:: Black,
        };
        let ai_handler= AI_Logic_Handler::new(ai_color, difficulty);


        // Game loop
        loop {
            let mut played_move: bool;

            let possible_moves = game_board.get_possible_moves();
            // Print Current Board
            Board::print_board(&game_board, &possible_moves);
            // If it is blacks turn
            match game_board.current_player {
                constants::Color::Black =>
                    {
                        match player {
                        constants::Color::Black => {
                            // Get move
                            let (move_row, move_col) = Board::get_user_move();
                            let player_move = Board::get_pos(move_row, move_col);
                            // Try to play move
                            played_move = match player_move {
                                Ok(value) => game_board.place_move(&possible_moves, value),
                                Err(value) => false,
                            };
                        }
                            constants::Color::White => {
                               let ai_move =  ai_handler.get_ai_move(&possible_moves, game_board.clone());
                                played_move = game_board.place_move(&possible_moves, ai_move);
                            }


                        }
                    }
                constants::Color::White => {
                    match player {
                        constants::Color::White => {
                            let (move_row, move_col) = Board::get_user_move();
                            let player_move = Board::get_pos(move_row, move_col);
                            // Try to play move
                            played_move = match player_move {
                                Ok(value) => game_board.place_move(&possible_moves, value),
                                Err(value) => false,
                            };

                        }
                        constants::Color::Black => {
                            let ai_move =  ai_handler.get_ai_move(&possible_moves, game_board.clone());
                           played_move = game_board.place_move(&possible_moves, ai_move);
                        }
                    }
                }
            }
            if Board::check_game_over(&game_board) {
                break;
            }
        }

}

