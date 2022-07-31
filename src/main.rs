struct Tile {
    is_bomb: bool,
    is_revealed: bool,
}

enum GameState {
    InProgress,
    Won,
    Lost
}

struct Game {
    width: usize,
    height: usize,
    board: Vec<Tile>,
    state: GameState
}

impl Game {
    pub fn new(width: usize, height: usize, bomb_count: usize) -> Self {
        let board = Game::init_game_board(width, height, bomb_count);
        let state = Game::calculate_state(&board);
        Game {
            width: width,
            height: height,
            board,
            state
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        let index = x + self.width * y;

        let mut tile = &mut self.board[index];
        tile.is_revealed = true;

        self.state = Game::calculate_state(&self.board);
    }

    fn calculate_state(board: &Vec<Tile>) -> GameState {
        for tile in board.iter() {
            if tile.is_bomb && tile.is_revealed {
                return GameState::Lost;
            }
    
            if !tile.is_bomb && !tile.is_revealed {
                return GameState::InProgress;
            }
        };
    
        GameState::Won
    }
    
    fn init_game_board(width: usize, height: usize, bomb_count: usize) -> Vec<Tile> {
        let mut board: Vec<Tile> = Vec::new();
        board.push(Tile { 
            is_bomb: bomb_count > 0,
            is_revealed: false});

        return board;
    }
}





fn main() {
    let mut game = Game::new(2, 2, 1);
}

#[cfg(test)]
mod tests;

#[test]
fn game_with_unrevealed_non_bomb_tile_should_be_in_progress() {
    let game = Game::new(1, 1, 0);
    assert!(matches!(game.state, GameState::InProgress));
}

#[test]
fn game_with_single_non_bomb_tile_that_is_revealed_should_be_won() {
    let mut game = Game::new(1, 1, 0);
    game.reveal(0,0);
    assert!(matches!(game.state, GameState::Won));
}
