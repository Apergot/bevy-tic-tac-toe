mod errors;

const MAX_NUMBER_OF_TURNS: i32 = 9;
const ODD_TURN_CHARACTER: char = 'X';
const EVEN_TURN_CHARACTER: char = 'O';

pub fn fill_game_board_coordinates_based_on_turn(
    current_turn: i32, 
    game_board: &mut [[i32; 3]; 3],
    coordinates: (i32, i32)
) -> Result<(), &'static str>{
    Ok(())
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
        _ => Err(errors::OUT_OF_RANGE_ERROR_MESSAGE)
    }
}

#[cfg(test)]
mod turns {
    use super::*;

    #[test]
    fn fill_game_board_coordinates_based_on_turn_given_invalid_coordinates () {
        let mut game_board: [[i32; 3]; 3] = [[0; 3]; 3];
        let starting_turn: i32 = 1;
        let coordinates: (i32, i32) = (-1, 12);

        assert_eq!(
            fill_game_board_coordinates_based_on_turn(
                starting_turn, 
                &mut game_board, 
                coordinates
            ),
            Err(errors::INVALID_COORDINATES_PROVIDED)
        )
    }

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
            Err(errors::OUT_OF_RANGE_ERROR_MESSAGE)
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
            Err(errors::OUT_OF_RANGE_ERROR_MESSAGE)
        );

        assert_eq!(current_turn, 10);
        assert!(!game_has_finished);
    }
}