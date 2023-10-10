mod turns;
mod constants;

fn main() {
    let mut starting_turn: i32 = 2;
    let mut game_has_finished: bool = false;
    let mut game_board: [[char; 3]; 3] = [[constants::EMPTY_CELL_CHARACTER; 3]; 3];


    match turns::define_next_turn(&mut starting_turn, &mut game_has_finished) {
        Ok(()) => {
            println!("Turn incremented successfully");
            match turns::fill_game_board_coordinates_based_on_turn(starting_turn, &mut game_board, (0,0)) {
                Ok(()) => {
                    for row in &game_board {
                        for &cell in row {
                            print!("{} ", cell);
                        }
                        println!();
                    }
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
}
