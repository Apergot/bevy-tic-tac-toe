mod turns;
mod constants;
use std::io;

fn main() {
    let mut starting_turn: i32 = 1;
    let mut game_has_finished: bool = false;
    let mut game_board: [[char; 3]; 3] = [[constants::EMPTY_CELL_CHARACTER; 3]; 3];

    while !game_has_finished {
        let coordinates = get_coordinates_from_user_input();
        match turns::define_next_turn(&mut starting_turn, &mut game_has_finished) {
            Ok(()) => {
                match turns::fill_game_board_coordinates_based_on_turn(starting_turn, &mut game_board, coordinates) {
                    Ok(()) => {
                        match turns::check_win_on_last_move(&game_board, coordinates) {
                            Ok(true) => {
                                println!("Player with symbol {} has won!", game_board[coordinates.0 as usize][coordinates.1 as usize]);
                                print_game_board_to_console(game_board);
                                break;
                            },
                            Ok(false) => {
                                print_game_board_to_console(game_board);
                                println!("Starting next turn!")
                            },
                            Err(err_msg) => {
                                println!("Error: {}", err_msg)
                            }
                        }
                    },
                    Err(err_msg) => {
                        println!("Error: {}", err_msg)
                    }
                }
            },
            Err(err_msg) => {
                println!("Error: {}", err_msg);
                println!("Game has finished due to max number of turns reached");
                break;
            }
        }
    }
}

fn get_coordinates_from_user_input() -> (i32, i32) {
    let mut input = String::new();

    println!("Provide row:");
    io::stdin().read_line(&mut input).expect("Error reading input");
    let row: i32 = input.trim().parse().expect("Unable to retrieve row value");

    input.clear();

    println!("Provide col:");
    io::stdin().read_line(&mut input).expect("Error reading input");
    let col: i32 = input.trim().parse().expect("Unable to retrieve col value");

    (row, col)
}

fn print_game_board_to_console(game_board: [[char; 3]; 3]) {
    for row in &game_board {
        for &cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}