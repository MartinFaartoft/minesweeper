use minesweeper::*;
use std::collections::HashSet;

macro_rules! hash_set {
    ( $( $x:expr ),* ) => {  // Match zero or more comma delimited items
        {
            let mut temp_set = HashSet::new();  // Create a mutable HashSet
            $(
                temp_set.insert($x); // Insert each item matched into the HashSet
            )*
            temp_set // Return the populated HashSet
        }
    };
}

#[test]
fn game_with_unrevealed_non_bomb_tile_should_be_in_progress() {
    let state = Game::new_random(1, 1, 0);
    assert!(matches!(state, GameState::InProgress(_)));
}

#[test]
fn game_with_revealed_bomb_tile_should_be_lost() {
    let state = Game::new(2, 1, &hash_set![0]);
    let lost_game = to_in_progress(state).reveal(0, 0);
    assert!(matches!(lost_game, GameState::Lost(_)));
}

#[test]
fn game_with_single_non_bomb_tile_that_is_revealed_should_be_won() {
    let state = Game::new_random(1, 1, 0);
    let won_game = to_in_progress(state).reveal(0, 0);
    assert!(matches!(won_game, GameState::Won(_)));
}

#[test]
fn single_tile_board_should_have_zero_bomb_neighbors() {
    let state = Game::new_random(1, 1, 0);
    assert_eq!(to_in_progress(state).board[0].bomb_neighbors, 0);
}

#[test]
fn tile_with_one_bomb_neighbour_should_have_correct_neighbour_count() {
    let state = Game::new(2, 1, &&hash_set![1]);
    let game = to_in_progress(state);
    assert_eq!(game.board[0].bomb_neighbors, 1);
}

#[test]
fn three_by_three_board_all_bombs_center_tile_should_have_eight_neighbors() {
    let state = Game::new(3, 3, &hash_set![0, 1, 2, 3, 5, 6, 7, 8]);
    let game = to_in_progress(state);
    assert_eq!(game.board[4].bomb_neighbors, 8);
}

fn to_in_progress(s: GameState) -> Game {
    match s {
        GameState::InProgress(g) => g,
        _ => panic!(),
    }
}

fn _to_finished_game(s: GameState) -> FinishedGame {
    match s {
        GameState::Won(g) => g,
        GameState::Lost(g) => g,
        _ => panic!("State was InProgress"),
    }
}
