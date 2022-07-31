use rand::seq::index::sample;
use std::collections::HashSet;

type Dim = u8;
type BoardSize = u16;

pub struct Tile {
    pub is_bomb: bool,
    pub is_revealed: bool,
    pub bomb_neighbors: u8,
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
        let size = (width * height) as usize;
        let bombs: HashSet<usize> = sample(&mut rand::thread_rng(), size, bomb_count as usize)
            .into_iter()
            .collect();

        let mut bomb_count = vec![0; size];

        fn neighbour_indices(i: usize, width: usize, max: usize) -> Vec<usize> {
            let on_left_edge = i % width == 0;
            let on_top_row = i < width;
            let on_right_edge = i % width == width - 1;
            let on_bottom_row = max - i < width;

            let mut neighbors = Vec::new();

            if !on_top_row {
                neighbors.push(i - width);
                if !on_left_edge {
                    neighbors.push(i - width - 1);
                }

                if !on_right_edge {
                    neighbors.push(i - width + 1);
                }
            }

            if !on_left_edge {
                neighbors.push(i - 1);
            }

            if !on_right_edge {
                neighbors.push(i + 1);
            }

            if !on_bottom_row {
                neighbors.push(i + width);
                if !on_left_edge {
                    neighbors.push(i + width - 1);
                }

                if !on_right_edge {
                    neighbors.push(i + width + 1);
                }
            }

            return neighbors;
        }

        for i in 0..size {
            let neighbors = neighbour_indices(i, width as usize, size);
            bomb_count[i] += neighbors.into_iter().filter(|i| bombs.contains(i)).count() as u8;
        }

        let tiles = bomb_count
            .into_iter()
            .enumerate()
            .map(|(i, count)| Tile {
                is_bomb: bombs.contains(&i),
                is_revealed: false,
                bomb_neighbors: count,
            })
            .collect();

        return tiles;
    }
}