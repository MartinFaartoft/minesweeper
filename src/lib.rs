use rand::prelude::*;

type Dim = u8;
type BoardSize = u16;

pub struct Tile {
    pub is_bomb: bool,
    pub is_revealed: bool,
    pub bomb_neighbors: u8
}

pub enum GameState {
    InProgress,
    Won,
    Lost,
}

pub struct Game {
    pub width: Dim,
    pub height: Dim,
    pub board: Vec<Tile>,
    pub state: GameState,
}

impl Game {
    pub fn new(width: Dim, height: Dim, bomb_count: BoardSize) -> Self {
        let board = Game::init_random_game_board(width, height, bomb_count);
        let state = Game::calculate_state(&board);
        Game {
            width: width,
            height: height,
            board,
            state,
        }
    }

    pub fn reveal(&mut self, x: Dim, y: Dim) {
        let index = x + self.width * y;

        let mut tile = &mut self.board[index as usize];
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
        }

        GameState::Won
    }

    fn init_random_game_board(width: Dim, height: Dim, bomb_count: BoardSize) -> Vec<Tile> {
        let mut rng = rand::thread_rng();
        let bomb_indices = rand::seq::index::sample(&mut rng, (width * height) as usize, bomb_count as usize);
        
        // TODO: map bomb_indices to vector of tiles and return
        let mut board: Vec<Tile> = Vec::new();
        board.push(Tile {
            is_bomb: bomb_count > 0,
            is_revealed: false,
            bomb_neighbors: 0
        });

        return board;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_tile_board_should_have_zero_bomb_neighbors() {

    }
}
