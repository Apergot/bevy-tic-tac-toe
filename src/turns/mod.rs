use crate::constants::*;
mod errors;

pub fn fill_game_board_coordinates_based_on_turn(
    current_turn: i32, 
    game_board: &mut [[char; 3]; 3],
    coordinates: (i32, i32)
) -> Result<(), &'static str>{
    let(row, col) = coordinates;

    if row < 0 || row > 2 || col < 0 || col > 2 {
        return Err(errors::INVALID_COORDINATES_PROVIDED);
    }

    if game_board[row as usize][col as usize] != EMPTY_CELL_CHARACTER {
        return Err(errors::CELL_ALREADY_OCCUPIED);
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

pub fn check_win_on_last_move(
    game_board: &[[char; 3]; 3],
    last_move: (i32, i32)
) -> Result<bool, &'static str> {
    let row: usize = last_move.0 as usize;
    let col: usize = last_move.1 as usize;

    if row >= 3 || col >= 3 {
        return Err(errors::INVALID_COORDINATES_PROVIDED);
    }

    let player_symbol: char = game_board[row][col];

    if player_symbol != EVEN_TURN_CHARACTER && player_symbol != ODD_TURN_CHARACTER {
        return Err(errors::FOUND_INVALID_CHARACTER_AT_LAST_MOVE_COORDINATES)
    }

    if (0..3).all(|i| game_board[row][i] == player_symbol) {
        return Ok(true);
    }

    if (0..3).all(|i| game_board[i][col] == player_symbol) {
        return Ok(true);
    }

    if row == col && (0..3).all(|i| game_board[i][i] == player_symbol) {
        return Ok(true);
    }

    if row + col == 2 && (0..3).all(|i| game_board[i][2 - i] == player_symbol) {
        return Ok(true);
    }

    Ok(false)
}

#[cfg(test)]
mod check_win_on_last_move {
    use super::*;

    #[test]
    fn first_horizontal_row_check_should_return_true() {
        let game_board: [[char; 3]; 3] = [
            ['X', 'X', 'X'],
            ['O', 'O', '_'],
            ['_', 'X', 'O'],
        ];
        let last_move = (0, 2);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn first_vertical_column_check_should_return_true() {
        let game_board: [[char; 3]; 3] = [
            ['X', 'O', 'X'],
            ['X', 'O', '_'],
            ['X', 'X', 'O'],
        ];
        let last_move = (2, 0);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn primary_diagonal_check_should_return_true() {
        let game_board: [[char; 3]; 3] = [
            ['O', 'X', 'X'],
            ['X', 'O', '_'],
            ['X', 'O', 'O'],
        ];
        let last_move = (2, 2);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn secondary_diagonal_check_should_return_true() {
        let game_board: [[char; 3]; 3] = [
            ['O', 'X', 'X'],
            ['X', 'X', '_'],
            ['X', 'O', 'O'],
        ];
        let last_move = (2, 0);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Ok(true));
    }

    #[test]
    fn last_move_not_winning_turn_should_return_false() {
        let game_board: [[char; 3]; 3] = [
            ['X', 'O', 'X'],
            ['_', 'O', '_'],
            ['X', '_', 'O'],
        ];
        let last_move = (1, 1);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Ok(false));
    }

    #[test]
    fn given_invalid_coordinates_should_return_error() {
        let game_board: [[char; 3]; 3] = [
            ['X', 'O', 'X'],
            ['_', 'O', '_'],
            ['X', '_', 'O'],
        ];
        let last_move = (3, 1);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Err(errors::INVALID_COORDINATES_PROVIDED));
    }

    #[test]
    fn invalid_character_at_provided_coordinates_should_return_error() {
        let game_board: [[char; 3]; 3] = [
            ['X', 'O', 'A'],
            ['_', 'O', '_'],
            ['X', '_', 'O'],
        ];
        let last_move = (0, 2);

        let result = check_win_on_last_move(&game_board, last_move);

        assert_eq!(result, Err(errors::FOUND_INVALID_CHARACTER_AT_LAST_MOVE_COORDINATES));
    }
}

#[cfg(test)]
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