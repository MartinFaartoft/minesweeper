use rand::seq::index::sample;
use std::collections::HashSet;

type Dim = usize;
type BoardSize = usize;

pub struct Tile {
    pub is_bomb: bool,
    pub is_revealed: bool,
    pub bomb_neighbors: u8,
}

pub enum GameState {
    InProgress(Game),
    Won(FinishedGame),
    Lost(FinishedGame),
}

pub struct Game {
    pub width: Dim,
    pub height: Dim,
    pub board: Vec<Tile>,
}

pub struct FinishedGame {
    pub board: Vec<Tile>,
}

impl Game {
    pub fn new_random(width: Dim, height: Dim, bomb_count: BoardSize) -> GameState {
        let bomb_indices: HashSet<usize> = sample(
            &mut rand::thread_rng(),
            (width * height) as usize,
            bomb_count as usize,
        )
        .into_iter()
        .collect();

        Game::new(width, height, &bomb_indices)
    }

    pub fn new(width: Dim, height: Dim, bomb_indices: &HashSet<usize>) -> GameState {
        let game = Game {
            width,
            height,
            board: Game::init_game_board(width, height, &bomb_indices),
        };

        game.calculate_state()
    }

    pub fn reveal(mut self, x: Dim, y: Dim) -> GameState {
        let index = x + self.width * y;

        let mut tile = &mut self.board[index as usize];
        tile.is_revealed = true;

        Game::calculate_state(self)
    }

    fn calculate_state(self) -> GameState {
        let mut unrevealed_non_bomb = false;
        for tile in &self.board {
            if tile.is_bomb && tile.is_revealed {
                return GameState::Lost(FinishedGame { board: self.board });
            }

            if !tile.is_bomb && !tile.is_revealed {
                unrevealed_non_bomb = true;
            }
        }

        if unrevealed_non_bomb {
            GameState::InProgress(self)
        } else {
            GameState::Won(FinishedGame { board: self.board })
        }
    }

    fn init_game_board(width: Dim, height: Dim, bomb_indices: &HashSet<usize>) -> Vec<Tile> {
        let size = (width * height) as usize;
        let mut bomb_count = vec![0; size];

        for i in 0..size {
            let neighbors = Game::neighbour_indices(i, width as usize, size);
            bomb_count[i] += neighbors
                .into_iter()
                .filter(|i| bomb_indices.contains(i))
                .count() as u8;
        }

        bomb_count
            .into_iter()
            .enumerate()
            .map(|(i, count)| Tile {
                is_bomb: bomb_indices.contains(&i),
                is_revealed: false,
                bomb_neighbors: count,
            })
            .collect()
    }

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

        neighbors
    }
}
