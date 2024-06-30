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
    train.train(2, 100, 2, 10);
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

fn print_win_1(turn: bool, board: &Board) {
    if !turn && (board.is_a_win()) {
        println!("You won.");
    } else if turn && (board.is_a_win()) {
        println!("AI won.");
    } else {
        println!("It is a draw.");
    }
}

fn print_win_2(turn: bool, board: &Board) {
    if !turn && (board.is_a_win()) {
        println!("AI won.");
    } else if turn && (board.is_a_win()) {
        println!("You won.");
    } else {
        println!("It is a draw.");
    }
}

fn play_computer_move(mm: &mut Minimax, board: &mut Board) {
    let mut _move: i16 = mm.minimax(board.clone());
    board.play_move(_move)
}

fn play_user_move(board: &mut Board) {
    let mut _move: String = String::new();
    println!("Enter the move: ");
    let _ = std::io::stdin().read_line(&mut _move).unwrap();
    let _move: i8 = _move.trim().parse().expect("REASON");
    board.play_move(1<<(9 - _move));
}

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





pub fn play_game() {
    let mut board: Board= Board::new();
    let mut board2: Board = Board::new();
    board2.change_type();
    let mut mm= Minimax::new();
    let mut nm = Minimax::new();
    nm.set_training_mode();
    let mut turn= true;

    for _i in 0..10 {
        
        if board.has_game_ended() {

            if !turn && (board.is_a_win()) {
                println!("Trainee won");
            } else if turn && (board.is_a_win()) {
                println!("Trainee lost");
            } else {
                println!("Drawn");
            }

            break;
        }

        println!();
        let mut _move= 0;
        if turn {
            _move= mm.minimax(board.clone());
        }else {
            _move= nm.minimax(board2.clone());
        }

        board2.play_move(_move);
        board.play_move(_move);
        let x = board.get_board_in_array();
        for i in 0..x.len(){
            print!("{}", x[i]);
            print!("  ");
            if (i + 1) % 3 == 0 {
                println!();
            }
        }
        
        turn = !turn;
    }
}
