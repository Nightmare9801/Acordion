use std::{fs, io};

use neuroflow::{data::DataSet, FeedForward};

use crate::{board::Board, game_tree::GameTree, minimax::Minimax};
pub(crate) struct Trainer{
    trainee: FeedForward,
    storer_no: i16,
    total_no: i16,
}

impl Trainer {
    /// The function `new` creates a new `Trainer` instance with a specified `FeedForward` neural
    /// network and default values for `storer_no` and `total_no`.
    /// 
    /// Arguments:
    /// 
    /// * `nn`: The `nn` parameter in the `new` function is of type `FeedForward`. It is used to
    /// initialize the `trainee` field of the `Trainer` struct.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `Trainer` struct is being returned with the specified values for the
    /// `trainee`, `storer_no`, and `total_no` fields.
    pub fn new(nn: FeedForward) -> Self {
        Trainer {  
            trainee: nn,
            storer_no: 100,
            total_no: 100,
        }
    }

    /// The function `spawn_trainers` creates and saves neural network trainers based on the specified
    /// size.
    /// 
    /// Arguments:
    /// 
    /// * `size`: The `size` parameter in the `spawn_trainers` function represents the number of
    /// trainers that will be spawned. It is used to determine how many instances of the `FeedForward`
    /// neural network model will be created and saved to files.
    pub fn spawn_trainers(&self, size: i16) {
        for i in 0..size {
            let mut nn: FeedForward= neuroflow::io::load("src/Trainee/acordion.flow").unwrap();
            let y= format!("src/Trainers/acordion-trainers-{}.flow", i);
            neuroflow::io::save(&mut nn, &y).unwrap();

        }

    }

    /// The `reset` function in Rust deletes all files in the "src/Trainers" and "src/Storers"
    /// directories.
    /// 
    /// Returns:
    /// 
    /// The `reset` function returns a `Result` type from the `io` module. The `Ok(())` value is
    /// returned if the function executes successfully without any errors.
    pub fn reset(&self) -> io::Result<()> {
        for entry in fs::read_dir("src/Trainers")? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }
        for entry in fs::read_dir("src/Storers")? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                fs::remove_file(path)?;
            }
        }
        Ok(())
    }

    /// The function `store_storer` loads a neural network model, saves it to a file with a specific
    /// name based on the storer number, and increments the storer number.
    pub fn store_storer(&mut self) {
        let mut nn: FeedForward = neuroflow::io::load("src/Trainee/acordion.flow").unwrap();
        let y= format!("src/Storers/acordion-storers-{}.flow", self.storer_no);
        neuroflow::io::save(&mut nn, &y).unwrap();
        if self.storer_no == self.total_no {
            self.storer_no = 0;
        } else {
            self.storer_no += 1;
        }
    }

    /// The function `spawn_storers` creates and saves neural network models for storing data.
    /// 
    /// Arguments:
    /// 
    /// * `size`: The `size` parameter in the `spawn_storers` function represents the number of storers
    /// to be spawned. It is used to determine how many storers will be created and saved to files with
    /// unique names.
    pub fn spawn_storers (&self, size: i16) {
        for i in 0..size {
            let mut nn: FeedForward= FeedForward::new(&[9, 16, 32, 16, 1]);
            let y= format!("src/Storers/acordion-storers-{}.flow", i);
            neuroflow::io::save(&mut nn, &y).unwrap();
        }
    }

    /// The function `change_trainers` iterates through a range of values, loads a neural network model
    /// from a file, and saves it to a new file with a different name.
    /// 
    /// Arguments:
    /// 
    /// * `size`: The `size` parameter in the `change_trainers` function represents the number of
    /// trainers that you want to change. It is used to iterate over a range from 0 to `size` in order
    /// to load and save neural network models for each trainer.
    pub fn change_trainers(&self, size: i16) {
        for i in 0..size {
            let y1 = format!("src/Storers/acordion-storers-{}.flow", i);
            let y2 = format!("src/Storers/acordion-trainers-{}.flow", i);
            let mut nn: FeedForward = neuroflow::io::load(&y1).unwrap();
            neuroflow::io::save(&mut nn, &y2).unwrap();
        }
    }

    /// The `train` function in Rust is used to start the training process by spawning storers and
    /// trainers, and then executing the middle trainer achiever with specified parameters.
    /// 
    /// Arguments:
    /// 
    /// * `size`: The `size` parameter represents the size of the training environment or dataset. It is
    /// used to determine the number of storers and trainers to spawn in the training process.
    /// * `iterations`: The `iterations` parameter in the `train` function represents the number of
    /// training iterations or epochs that will be performed during the training process. Each iteration
    /// typically involves one forward pass and one backward pass of all the training examples.
    /// * `ghost_steps`: The `ghost_steps` parameter in the `train` function likely represents the
    /// number of steps taken by a ghost entity during training. This parameter is used in the
    /// `middle_trainer_achiever` method to control the behavior of the ghost entity during the training
    /// process.
    /// * `reset_steps`: The `reset_steps` parameter in the `train` function represents the number of
    /// steps after which a reset operation will be triggered during the training process.
    pub fn train(&mut self, size: i16, iterations: i16, ghost_steps : i16, reset_steps : i16){
        println!("Training starts");
        self.spawn_storers(size);
        self.spawn_trainers(size);
        self.middle_trainer_achiever(size, iterations, ghost_steps, reset_steps);
        let _ = self.reset();
    }

    /// The function `middle_trainer_achiever` iterates a specified number of times, training for
    /// opponents, storing data periodically, and changing trainers at certain intervals.
    /// 
    /// Arguments:
    /// 
    /// * `size`: The `size` parameter represents the size of the training data or the number of
    /// opponents to train against.
    /// * `iterations`: The `iterations` parameter specifies the total number of iterations the
    /// `middle_trainer_achiever` function will run for. Each iteration includes training for opponents,
    /// storing data, and potentially changing trainers based on the `ghost_steps` and `reset_steps`
    /// conditions.
    /// * `ghost_steps`: The `ghost_steps` parameter determines how often the `store_storer` method is
    /// called during the iterations. If `i % ghost_steps == 0`, then the `store_storer` method will be
    /// called.
    /// * `reset_steps`: The `reset_steps` parameter determines after how many iterations the trainers
    /// should be reset in the `middle_trainer_achiever` function. When the current iteration count is a
    /// multiple of `reset_steps`, the trainers will be changed/reset.
    pub fn middle_trainer_achiever(&mut self, size: i16, iterations: i16, ghost_steps : i16, reset_steps : i16) {
        for i in 0..iterations {
            println!("Starting iteration {}", i);
            self.train_for_opponents(size);
            //self.change_trainers(size);
            if i % ghost_steps == 0 {
                self.store_storer();
            }
            if i % reset_steps == 0 {
                self.change_trainers(size);
            }
        }
    }

    /// The function `train_for_opponents` trains the model iteratively for a specified number of
    /// opponents.
    /// 
    /// Arguments:
    /// 
    /// * `size`: The `size` parameter in the `train_for_opponents` function represents the number of
    /// opponents for which the training will be conducted. It is used to determine the range of
    /// opponents to iterate over and train against.
    pub fn train_for_opponents(&mut self, size: i16) {
        for i in 0..size{
            println!("Opponent {}", i);
            let y2 = format!("src/Trainers/acordion-trainers-{}.flow", i);
            self.train_iteratively(&y2);
        }
    }
    
    /// The function `train_iteratively` trains a model iteratively by playing games with two different
    /// types and updating the model based on the results.
    /// 
    /// Arguments:
    /// 
    /// * `trainer`: The `trainer` parameter in the `train_iteratively` function is a string that
    /// specifies the type of trainer being used for training the model. It is used to determine which
    /// training method to apply during the iterative training process.
    pub fn train_iteratively(&mut self, trainer: &str) {
        for _i in 0..1 {
            let tree= self.play_game_with_type_1(trainer);
            let tree2 = self.play_game_with_type_2(trainer);
            
            self.train_for_tree(tree);
            self.train_for_tree(tree2)
        }
    }

    /// The function `train_for_tree` trains a neural network model using data from a game tree and
    /// saves the trained model to a file.
    /// 
    /// Arguments:
    /// 
    /// * `tree`: The `tree` parameter in the `train_for_tree` function is of type `GameTree`. It is
    /// used as input to calculate data and create a dataset for training a neural network model.
    fn train_for_tree(&mut self, tree: GameTree) {
        let (positions, predicted_rewards) = self.calculate_data(tree);
    
        
        let data = self.create_dataset(positions, predicted_rewards);
    
        self.trainee.activation(neuroflow::activators::Type::Tanh).learning_rate(0.01).train(&data, 50_000);
        neuroflow::io::save(&mut self.trainee, "src/Trainee/acordion.flow").unwrap();
    }
    
    /// The function `calculate_data` takes a `GameTree` as input, calculates predicted rewards based on
    /// rewards and positions in the tree, and returns a tuple containing the positions and predicted
    /// rewards.
    /// 
    /// Arguments:
    /// 
    /// * `tree`: The `tree` parameter in the `calculate_data` function is of type `GameTree`. It seems
    /// to contain information about rewards and positions related to a game. The function retrieves
    /// rewards and positions from the `tree` object, calculates predicted rewards based on a discount
    /// factor and the position index
    pub fn calculate_data(&mut self, tree: GameTree) -> (Vec<Vec<f64>>, Vec<f64>) {
        let rewards = tree.get_rewards();
        let positions = tree.get_positions();
        let mut predicted_rewards = Vec::new();
        let discount_factor = 0.99; // Adjusted discount factor
    
        for i in 0..rewards.len() {
            let mut reward_to_be_pushed: f64 = 0.0;
            for j in i..rewards.len() {
                let discount = self.power(discount_factor, j - i); // Calculate discount for each time step
                reward_to_be_pushed += rewards[j] as f64 * discount;
            }
            predicted_rewards.push(reward_to_be_pushed);
        }
    
        (positions, predicted_rewards)
    }
    

    /// The `create_dataset` function in Rust creates a DataSet from provided positions and predicted
    /// rewards.
    /// 
    /// Arguments:
    /// 
    /// * `positions`: The `positions` parameter is a vector of vectors of type `f64`. Each inner vector
    /// represents a position with its coordinates specified as `f64` values.
    /// * `predicted_rewards`: The `predicted_rewards` parameter in the `create_dataset` function
    /// represents a vector of predicted rewards corresponding to each position in the `positions`
    /// vector. Each element in the `predicted_rewards` vector is a single predicted reward value
    /// associated with a specific position in the `positions` vector.
    /// 
    /// Returns:
    /// 
    /// The `create_dataset` function returns a `DataSet` object after populating it with the provided
    /// positions and predicted rewards.
    pub fn create_dataset(&self, positions: Vec<Vec<f64>>, predicted_rewards: Vec<f64>) -> DataSet {
        let mut data: DataSet = DataSet::new();
        for i in 0..positions.len() {
            data.push(&positions[i], &[predicted_rewards[i]])
        }
        data
    }
    

    /// This Rust function `play_game_with_type_2` plays a game with a specific type, updating a game
    /// tree based on the moves made.
    /// 
    /// Arguments:
    /// 
    /// * `trainer`: The `trainer` parameter in the `play_game_with_type_2` function is a reference to a
    /// string that represents the trainer playing the game. It is used to initialize the game state and
    /// control the game flow during the gameplay.
    /// 
    /// Returns:
    /// 
    /// The function `play_game_with_type_2` is returning a `GameTree` object.
    pub fn play_game_with_type_2(&self, trainer: &str) -> GameTree {
        let (mut tree, mut board, mut board2, mut mm, mut nm, mut turn) = self.game_init_controlled_2(trainer);
        loop {
            let mut _move= self.get_move(turn, &mut mm, &board, &mut nm, &board2);

            self.play_move(&mut board2, _move, &mut board);
            
            if board.has_game_ended() || board2.has_game_ended() {
                self.add_reward_on_ending(turn, &board, &board2, &mut tree);
                break;
            }

            self.add_reward(turn, &mut tree, &board);
            
            turn = !turn;
        }
        tree
    }

    /// This Rust function `play_game_with_type_1` plays a game with a specific type of logic and
    /// returns a `GameTree`.
    /// 
    /// Arguments:
    /// 
    /// * `trainer`: The `trainer` parameter in the `play_game_with_type_1` function is a reference to a
    /// string that represents the trainer in the game. It is used to initialize the game state and
    /// control the game flow based on the actions of the trainer.
    /// 
    /// Returns:
    /// 
    /// The function `play_game_with_type_1` is returning a `GameTree` object.
    pub fn play_game_with_type_1(&self, trainer: &str) -> GameTree {
        let (mut tree, mut board, mut board2, mut mm, mut nm, mut turn) = self.game_init_controlled(trainer);
        loop {
            let mut _move= self.get_move(turn, &mut mm, &board, &mut nm, &board2);

            self.play_move(&mut board2, _move, &mut board);
            
            if board.has_game_ended() || board2.has_game_ended() {
                self.add_reward_on_ending(turn, &board, &board2, &mut tree);
                break;
            }

            self.add_reward(turn, &mut tree, &board);
            
            turn = !turn;
        }
        tree
    }
    
    /// The function `game_init_controlled` initializes a game with controlled settings, including
    /// creating game boards and minimax algorithms.
    /// 
    /// Arguments:
    /// 
    /// * `trainer`: The `trainer` parameter in the `game_init_controlled` function is a reference to a
    /// string that represents the type of training mode for the Minimax algorithm.
    pub fn game_init_controlled(&self, trainer: &str) -> (GameTree, Board, Board, Minimax, Minimax, bool) {
        let mut board2: Board = Board::new();
        board2.change_type();
        let mut nm = Minimax::new();
        nm.set_training_mode_with_type(trainer);
        let turn= true;
        (GameTree::new(Vec::new(), Vec::new()), Board::new(), board2, Minimax::new(), nm, turn)
    }

    /// The function `game_init_controlled_2` initializes a game with controlled settings in Rust,
    /// including setting up the game board, minimax algorithms, and training mode.
    /// 
    /// Arguments:
    /// 
    /// * `trainer`: The `trainer` parameter in the `game_init_controlled_2` function is a reference to
    /// a string that specifies the type of training mode for the Minimax algorithm.
    pub fn game_init_controlled_2(&self, trainer: &str) -> (GameTree, Board, Board, Minimax, Minimax, bool) {
        let mut board: Board= Board::new();
        board.change_type();
        let mut mm= Minimax::new();
        mm.set_training_mode_with_type(trainer);
        let turn= true;
        (GameTree::new(Vec::new(), Vec::new()), board, Board::new(), mm, Minimax::new(), turn)
    }

    /// This Rust function `get_move` returns the best move using minimax algorithm based on the turn
    /// and board state provided.
    /// 
    /// Arguments:
    /// 
    /// * `turn`: The `turn` parameter is a boolean value that indicates whose turn it is in the game.
    /// `true` typically represents the player's turn, while `false` represents the opponent's turn.
    /// * `mm`: Minimax algorithm instance for player 1.
    /// * `board`: The `board` parameter in the `get_move` function represents the game board state for
    /// the current player. It is used as input to the minimax algorithm to determine the best move to
    /// make based on the current game state.
    /// * `nm`: `nm` appears to be an instance of the `Minimax` struct that is passed by mutable
    /// reference. It is used to perform the minimax algorithm on `board2`, which is an instance of the
    /// `Board` struct.
    /// * `board2`: The `board2` parameter in the `get_move` function represents the second game board
    /// that is used for the minimax algorithm. It is a reference to a `Board` object which contains the
    /// state of the game board at a particular point in the game. This board is used by the `
    /// 
    /// Returns:
    /// 
    /// The function `get_move` returns an `i16` value, which represents the move calculated by the
    /// Minimax algorithm based on the current game state and player's turn.
    pub fn get_move(&self, turn: bool, mm: &mut Minimax, board: &Board, nm: &mut Minimax, board2: &Board) -> i16 {
        let mut _move: i16 = 0;
        if turn {
            _move= mm.minimax(board.clone());
        }else {
            _move= nm.minimax(board2.clone());
        }
        _move
    }

    /// The function `add_reward` adds a reward of 0 to a game tree if the turn is true.
    /// 
    /// Arguments:
    /// 
    /// * `turn`: The `turn` parameter is a boolean value that indicates whose turn it is in the game.
    /// If `turn` is `true`, it means it is the player's turn; if `turn` is `false`, it means it is the
    /// opponent's turn.
    /// * `tree`: The `tree` parameter is a mutable reference to a `GameTree` object.
    /// * `board`: The `board` parameter in the `add_reward` function is of type `Board`. It is used to
    /// access the game board state in the form of an array through the `get_board_in_array` method.
    /// This array representation of the board is then added to the `GameTree` for further
    pub fn add_reward(&self, turn: bool, tree: &mut GameTree, board: &Board) {
        if turn {
            tree.add_position(board.get_board_in_array());
            tree.add_reward(0);
        }
    }

    /// This function adds a reward to a game tree based on the game board state and player turn.
    /// 
    /// Arguments:
    /// 
    /// * `turn`: The `turn` parameter is a boolean value that indicates whose turn it is in the game.
    /// `true` typically represents Player 1's turn, while `false` represents Player 2's turn.
    /// * `board`: The `board` parameter in the `add_reward_on_ending` function represents the current
    /// state of the game board. It is of type `Board` and is passed as a reference to the function. The
    /// function checks if this board state or another board state (`board2`) is a winning state
    /// * `board2`: The `board2` parameter in the `add_reward_on_ending` function is of type `&Board`.
    /// It is a reference to another instance of the `Board` struct that is being passed as an argument
    /// to the function. This parameter is used in the function to check if either `board
    /// * `tree`: The `tree` parameter is a mutable reference to a `GameTree` struct. This function
    /// `add_reward_on_ending` takes in several parameters including `turn` which is a boolean flag
    /// indicating the current player's turn, `board` and `board2` which are references to `Board
    pub fn add_reward_on_ending(&self, turn: bool, board: &Board, board2: &Board, tree: &mut GameTree) {
        if turn && (board.is_a_win() || board2.is_a_win()) {
            tree.add_position(board.get_board_in_array());
            tree.add_reward(1);
        } else if !turn && (board.is_a_win() || board2.is_a_win()) {
            tree.add_position(board.get_board_in_array());
            tree.add_reward(-1);
        } else {
            tree.add_position(board.get_board_in_array());
            tree.add_reward(0);
        }
    }

    /// The function `play_move` takes a move as input and plays it on two different game boards.
    /// 
    /// Arguments:
    /// 
    /// * `board2`: The `board2` parameter is a mutable reference to a `Board` object. This parameter is
    /// used to call the `play_move` method on the `board2` object to make a move in the game.
    /// * `_move`: The `_move` parameter is of type `i16`, which is a 16-bit signed integer. It is used
    /// to represent the move that a player wants to make in a game.
    /// * `board`: The `board` parameter in the `play_move` function is a mutable reference to a `Board`
    /// object. This parameter is used to call the `play_move` method on the `board` object to play a
    /// move.
    pub fn play_move(&self, board2: &mut Board, _move: i16, board: &mut Board) {
        board2.play_move(_move);
        board.play_move(_move);
    }
    
    /// The function calculates the power of a given number using an optimized algorithm in Rust.
    /// 
    /// Arguments:
    /// 
    /// * `x`: The parameter `x` represents the base value for the power operation.
    /// * `n`: The parameter `n` represents the exponent to which the base `x` is raised in the `power`
    /// function.
    /// 
    /// Returns:
    /// 
    /// the result of raising the base `x` to the power of `n`, calculated using an optimized algorithm
    /// for exponentiation.
    fn power(&self, x: f64, n: usize) -> f64 {
        if n == 0 {
            return 1.0;
        }
    
        let mut result = 1.0;
        let mut base = x;
        let mut exp = n;
    
        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }
            base *= base;
            exp /= 2;
        }
    
        result
    }
    
}