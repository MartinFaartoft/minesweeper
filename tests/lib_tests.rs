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
