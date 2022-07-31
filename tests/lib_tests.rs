use minesweeper::*;

#[test]
fn game_with_unrevealed_non_bomb_tile_should_be_in_progress() {
    let game = Game::new(1, 1, 0);
    assert!(matches!(game.state, GameState::InProgress));
}

#[test]
fn game_with_revealed_bomb_tile_should_be_lost() {
    let mut game = Game::new(1, 1, 1);
    game.reveal(0, 0);
    assert!(matches!(game.state, GameState::Lost));
}

#[test]
fn game_with_single_non_bomb_tile_that_is_revealed_should_be_won() {
    let mut game = Game::new(1, 1, 0);
    game.reveal(0, 0);
    assert!(matches!(game.state, GameState::Won));
}

#[test]
fn single_tile_board_should_have_zero_bomb_neighbors() {
    let game = Game::new(1, 1, 0);
    assert_eq!(game.board[0].bomb_neighbors, 0);
}

#[test]
fn two_by_one_board_with_two_bombs_should_both_have_one_neighbor() {
    let game = Game::new(2, 1, 2);
    assert_eq!(game.board[0].bomb_neighbors, 1);
    assert_eq!(game.board[1].bomb_neighbors, 1);
}

#[test]
fn three_by_three_board_all_bombs_center_tile_should_have_eight_neighbors() {
    let game = Game::new(3, 3, 9);
    assert_eq!(game.board[4].bomb_neighbors, 8);
}
