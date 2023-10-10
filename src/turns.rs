const MAX_NUMBER_OF_TURNS: i32 = 9;
const ODD_PLAYER_CHARACTER: char = 'X';
const EVEN_PLAYER_CHARACTER: char = 'O';
const OUT_OF_RANGE_ERROR_MESSAGE: &'static str = "Turn out of range";

fn current_turn_is_odd(turn: i32) -> bool {
    turn%2 != 0
}

fn current_turn_is_even(turn: i32) -> bool {
    turn%2 == 0
}

pub fn define_next_turn(
    current_turn: &mut i32, 
    game_has_finished: &mut bool
) -> Result<(), &'static str> {
    match *current_turn {
        1..=MAX_NUMBER_OF_TURNS => {
            if *current_turn == MAX_NUMBER_OF_TURNS {
                *game_has_finished = true;
                return Ok(());
            }

            *current_turn += 1;
            Ok(())
        },
        _ => Err(OUT_OF_RANGE_ERROR_MESSAGE)
    }
}

#[cfg(test)]
mod turns {
    use super::*;

    #[test]
    fn define_next_turn_for_starting_turn() {
        let mut current_turn: i32 = 1;
        let mut game_has_finished: bool = false;

        assert_eq!(define_next_turn(&mut current_turn, &mut game_has_finished), Ok(()));

        assert!(!game_has_finished);
        assert_eq!(current_turn, 2);
    }

    #[test]
    fn define_next_turn_when_game_should_finish() {
        let mut current_turn: i32 = 9;
        let mut game_has_finished: bool = false;

        assert_eq!(define_next_turn(&mut current_turn, &mut game_has_finished), Ok(()));

        assert_eq!(current_turn, 9);
        assert!(game_has_finished);
    }

    #[test]
    fn define_next_turn_when_turn_is_out_of_range_lower_limit() {
        let mut current_turn: i32 = 0;
        let mut game_has_finished: bool = false;

        assert_eq!(
            define_next_turn(&mut current_turn, &mut game_has_finished), 
            Err(OUT_OF_RANGE_ERROR_MESSAGE)
        );

        assert_eq!(current_turn, 0);
        assert!(!game_has_finished);
    }

    #[test]
    fn define_next_turn_when_turn_is_out_of_range_higher_limit() {
        let mut current_turn: i32 = 10;
        let mut game_has_finished: bool = false;

        assert_eq!(
            define_next_turn(&mut current_turn, &mut game_has_finished),
            Err(OUT_OF_RANGE_ERROR_MESSAGE)
        );

        assert_eq!(current_turn, 10);
        assert!(!game_has_finished);
    }
}