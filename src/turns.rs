const MAX_NUMBER_OF_TURNS: i32 = 9;
const ODD_PLAYER_CHARACTER: char = 'X';
const EVEN_PLAYER_CHARACTER: char = 'O';

fn current_turn_is_odd(turn: i32) -> bool {
    turn%2 != 0
}

fn current_turn_is_even(turn: i32) -> bool {
    turn%2 == 0
}

pub fn define_next_turn(current_turn: &i32) {

}

#[cfg(test)]
mod turns {
}