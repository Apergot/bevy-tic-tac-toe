mod turns;

fn main() {
    let mut starting_turn: i32 = 1;
    let mut game_has_finished: bool = false;
    let mut game_board: [[i32; 3]; 3] = [[0; 3]; 3];


    match turns::define_next_turn(&mut starting_turn, &mut game_has_finished) {
        Ok(()) => {
            println!("Turn incremented successfully")
        },
        Err(err_msg) => {
            println!("Error: {}", err_msg)
        }
    }
}