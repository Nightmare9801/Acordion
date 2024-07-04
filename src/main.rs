use board::Board;
use minimax::Minimax;
use trainer::Trainer;

pub mod board;
pub mod minimax;
pub mod bitboard_functions;
pub mod trainer;
pub mod game_tree;


fn main() {
    let mut train= Trainer::new(neuroflow::io::load("src/Trainee/acordion.flow").unwrap());
    let _ = train.reset();
    train.train(1, 0000, 2, 8);
    play_game_with_user();
}

pub fn play_game_with_user() {
    let mut board: Board = Board::new();
    let mut mm: Minimax = Minimax::new();

    let (mut turn, player_type) = get_user_preference();

    println!("Please enter the moves according to the table given below:");
    for i in 1..10 {
        print!("| {} ", 10 - i);
        if i % 3 == 0 {
            println!("|");
            //println!("______________");
        }
    }

    for _i in 0..10 {
        if !turn == player_type {
            play_user_move(&mut board);
        } else {
            play_computer_move(&mut mm, &mut board);
        }

        if player_type {
            print_board_1(&board);
        } else {
            print_board_2(&board);
        }

        if board.has_game_ended() {

            if player_type {
                print_win_1(turn, &board);
            } else {
                print_win_2(turn, &board)
            }

            break;
        }
        
        turn = !turn;
        println!("----------------------------------------------------------------------------------------------------------------");
    }
}

/// The function `get_user_preference` in Rust prompts the user to choose between 'X' and 'O' and
/// returns a tuple indicating the player's turn and type based on the choice.
fn get_user_preference() -> (bool, bool) {
    let mut turn = true;
    let mut player_type = false;
    
    println!("Do you want to take 'X' or 'O'?");

    let mut line: String = String::new();

    let _ = std::io::stdin().read_line(&mut line).unwrap();

    if line.starts_with("X") {
        turn = false;
        player_type = true;
    }
    (turn, player_type)
}

/// The function `print_win_1` prints out the result of a game based on the current turn and the state
/// of the game board.
/// 
/// Arguments:
/// 
/// * `turn`: The `turn` parameter represents whose turn it is in the game. It is a boolean value where
/// `true` typically represents the player's turn and `false` represents the AI's turn.
/// * `board`: The `board` parameter is a reference to an instance of the `Board` struct. It is used to
/// represent the current state of the game board, which likely includes information about the positions
/// of the players' pieces and the game's progress.
fn print_win_1(turn: bool, board: &Board) {
    if !turn && (board.is_a_win()) {
        println!("You won.");
    } else if turn && (board.is_a_win()) {
        println!("AI won.");
    } else {
        println!("It is a draw.");
    }
}

/// The function `print_win_2` prints a message based on the game outcome (win/lose/draw) for either the
/// AI or the player.
/// 
/// Arguments:
/// 
/// * `turn`: The `turn` parameter is a boolean value that indicates whose turn it is in the game.
/// `true` typically represents the player's turn, while `false` represents the AI's turn.
/// * `board`: The `board` parameter is a reference to an instance of the `Board` struct.
fn print_win_2(turn: bool, board: &Board) {
    if !turn && (board.is_a_win()) {
        println!("AI won.");
    } else if turn && (board.is_a_win()) {
        println!("You won.");
    } else {
        println!("It is a draw.");
    }
}

/// The function `play_computer_move` uses the minimax algorithm to determine the best move for the
/// computer player and then plays that move on the board.
/// 
/// Arguments:
/// 
/// * `mm`: Minimax algorithm implementation used for determining the best move in a game.
/// * `board`: The `board` parameter represents the game board on which the minimax algorithm will make
/// its move. It is a mutable reference to the game board, allowing the algorithm to make changes to the
/// board during the minimax calculation and when playing the move.
fn play_computer_move(mm: &mut Minimax, board: &mut Board) {
    let mut _move: i16 = mm.minimax(board.clone());
    board.play_move(_move)
}

/// The function `play_user_move` reads a user input for a move, converts it to an integer, and then
/// plays that move on the board.
/// 
/// Arguments:
/// 
/// * `board`: The `board` parameter is a mutable reference to a `Board` object.
fn play_user_move(board: &mut Board) {
    let mut _move: String = String::new();
    println!("Enter the move: ");
    let _ = std::io::stdin().read_line(&mut _move).unwrap();
    let _move: i8 = _move.trim().parse().expect("REASON");
    board.play_move(1<<(9 - _move));
}

/// The function `print_board_1` prints a Tic-Tac-Toe board represented by a vector of floats.
/// 
/// Arguments:
/// 
/// * `board`: The function `print_board_1` is to print a Tic-Tac-Toe board
/// represented by a vector of floats. The board is expected to be a 3x3 grid where each cell can
/// contain a value of 1.0 (for 'X'), -1
fn print_board_1(board: &Board) {
    let x: Vec<f64> = board.get_board_in_array();
    for i in 0..x.len(){
        print!("| ");
        let _type = if x[i] == 1.0 {
            "X"
        } else if x[i] == -1.0 {
            "O"
        } else {
            " "
        };
        print!("{}", _type);
        print!(" ");
        if (i + 1) % 3 == 0 {
            println!(" |");
        }
    
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
}

/// The function `print_board_2` prints a Tic-Tac-Toe board represented by a vector of floats.
/// 
/// Arguments:
/// 
/// * `board`: The function `print_board_2` is to print a Tic-Tac-Toe board
/// represented by a vector of `f64` values. The values -1.0, 1.0, and 0.0 represent 'X', 'O', and empty
/// space
fn print_board_2(board: &Board) {
    let x: Vec<f64> = board.get_board_in_array();
    for i in 0..x.len(){
        print!("| ");
        let _type = if x[i] == -1.0 {
            "X"
        } else if x[i] == 1.0 {
            "O"
        } else {
            " "
        };
        print!("{}", _type);
        print!(" ");
        if (i + 1) % 3 == 0 {
            println!(" |");
        }
    
        if (i + 1) % 3 == 0 {
            println!();
        }
    }
}
