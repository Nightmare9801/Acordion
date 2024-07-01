use neuroflow::FeedForward;

use crate::{bitboard_functions, board::Board};

pub(crate) struct Minimax{
    evaluater: FeedForward,
}

impl Minimax{
    /// The function `new` creates a new instance of a struct `Minimax` with an evaluator loaded from a
    /// file.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `Minimax` struct is being returned.
    pub fn new()-> Self{
        let evaluater= neuroflow::io::load("src/Trainee/acordion.flow").unwrap();
        Minimax{
            evaluater: evaluater,
        }
    }

    /// This function sets the training mode with a specified trainer in Rust.
    /// 
    /// Arguments:
    /// 
    /// * `trainer`: The `trainer` parameter is a reference to a string that specifies the name or path
    /// of the file containing the training data for the neural network model.
    pub fn set_training_mode_with_type(&mut self, trainer: &str) {
        self.evaluater= neuroflow::io::load(trainer).unwrap();
    }

    /// The `minimax` function in Rust implements the minimax algorithm to determine the best move in a
    /// board game using neural network evaluation.
    /// 
    /// Arguments:
    /// 
    /// * `board`: The `board` parameter in the `minimax` function represents the game board state on
    /// which the minimax algorithm will be applied to determine the best move. The `board` is a data
    /// structure that contains the current state of the game, including the positions of all pieces and
    /// any other relevant information
    /// 
    /// Returns:
    /// 
    /// The `minimax` function is returning the best move (an `i16` representing the index of the best
    /// move) based on the evaluation scores calculated for each possible move in the given board state.
    pub fn minimax(&mut self, mut board: Board) -> i16{
        let moves= bitboard_functions::get_indi_bits(board.gen_move());
        if moves.len() == 1 {
            return moves[0];
        }
        let mut best_move: i16= 0;
        let mut best_move_score: f64 =0.0;
        let mut is_first= true;
        for i in moves {
            board.play_move(i);
            let score= -self.evaluater.activation(neuroflow::activators::Type::Tanh).calc(&(*board.get_board_in_array().into_boxed_slice()))[0];
            board.undo_move(i);
            if is_first {
                is_first = false;
                best_move_score= score;
                best_move= i;
                continue;
            }
            if score > best_move_score {
                best_move= i;
                best_move_score = score;
            }
        }
        return best_move;
    }
    
}

