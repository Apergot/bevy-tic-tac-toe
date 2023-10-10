const MAX_NUMBER_OF_TURNS: i32 = 9;
const PLAYER_ONE_CHARACTER: char = 'X';
const PLAYER_TWO_CHARACTER: char = 'O';

fn current_turn_is_odd(turn: i32) -> bool {
    turn%2 != 0
}

fn current_turn_is_even(turn: i32) -> bool {
    turn%2 == 0
}

#[cfg(test)]
mod turns {
    use super::*;

    #[test]
    fn test_current_turn_is_odd_given_odd_number() {
        assert!(current_turn_is_odd(1));
    }

    #[test]
    fn test_current_turn_is_odd_given_even_number() {
        assert!(!current_turn_is_odd(2))
    }

    #[test]
    fn test_current_turn_is_even_given_even_number() {
        assert!(current_turn_is_even(2));
    }

    #[test]
    fn test_current_turn_is_even_given_odd_number() {
        assert!(!current_turn_is_even(1))
    }
}