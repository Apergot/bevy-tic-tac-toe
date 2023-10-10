use crate::constants::*;
mod errors;

pub fn fill_game_board_coordinates_based_on_turn(
    current_turn: i32, 
    game_board: &mut [[char; 3]; 3],
    coordinates: (i32, i32)
) -> Result<(), &'static str>{
    let(row, col) = coordinates;

    if row < 0 || row > 2 || col < 0 || col > 2 {
        return Err(errors::INVALID_COORDINATES_PROVIDED)
    }

    if game_board[row as usize][col as usize] != EMPTY_CELL_CHARACTER {
        return Err(errors::CELL_ALREADY_OCCUPIED)
    }
    
    let character = if current_turn % 2 == 0 { EVEN_TURN_CHARACTER } else { ODD_TURN_CHARACTER };
    game_board[row as usize][col as usize] = character;

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

mod fill_game_board_coordinates_based_on_turn {
    use super::*;
    #[test]
    fn given_invalid_coordinates_should_return_error () {
        let mut game_board: [[char; 3]; 3] = [[EMPTY_CELL_CHARACTER; 3]; 3];
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
    fn given_coordinates_for_already_occupied_cell_should_return_error() {
        let mut game_board: [[char; 3]; 3] = [[EMPTY_CELL_CHARACTER; 3]; 3];
        game_board[0][0] = 'A';
        let starting_turn: i32 = 1;
        let coordinates: (i32, i32) = (0, 0);

        assert_eq!(
            fill_game_board_coordinates_based_on_turn(
                starting_turn,
                &mut game_board,
                coordinates
            ),
            Err(errors::CELL_ALREADY_OCCUPIED)
        )
    }

    #[test]
    fn given_valid_coordinates_and_even_turn_should_place_even_character_into_cell() {
        let mut game_board: [[char; 3]; 3] = [[EMPTY_CELL_CHARACTER; 3]; 3];
        let turn: i32 = 2;
        let coordinates: (i32, i32) = (0, 0);

        assert_eq!(
            fill_game_board_coordinates_based_on_turn(
                turn,
                &mut game_board,
                coordinates
            ),
            Ok(())
        );
        assert_ne!(game_board[0][0], EMPTY_CELL_CHARACTER);
        assert_eq!(game_board[0][0], EVEN_TURN_CHARACTER);
    }

    #[test]
    fn given_valid_coordinates_and_even_turn_should_place_odd_character_into_cell() {
        let mut game_board: [[char; 3]; 3] = [[EMPTY_CELL_CHARACTER; 3]; 3];
        let turn: i32 = 1;
        let coordinates: (i32, i32) = (0, 0);

        assert_eq!(
            fill_game_board_coordinates_based_on_turn(
                turn,
                &mut game_board,
                coordinates
            ),
            Ok(())
        );
        assert_ne!(game_board[0][0], EMPTY_CELL_CHARACTER);
        assert_eq!(game_board[0][0], ODD_TURN_CHARACTER);
    }
}

#[cfg(test)]
mod define_next_turn {
    use super::*;
    #[test]
    fn given_starting_turn_should_only_increment_turn() {
        let mut current_turn: i32 = 1;
        let mut game_has_finished: bool = false;

        assert_eq!(define_next_turn(&mut current_turn, &mut game_has_finished), Ok(()));

        assert!(!game_has_finished);
        assert_eq!(current_turn, 2);
    }

    #[test]
    fn given_max_turn_number_should_mark_game_as_finished() {
        let mut current_turn: i32 = 9;
        let mut game_has_finished: bool = false;

        assert_eq!(define_next_turn(&mut current_turn, &mut game_has_finished), Ok(()));

        assert_eq!(current_turn, 9);
        assert!(game_has_finished);
    }

    #[test]
    fn given_invalid_lower_limit_should_return_error() {
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
    fn given_invalid_higher_limit_should_return_error() {
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