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

    fn to_index(&self, x: Dim, y: Dim) -> usize {
        x + self.width * y
    }

    pub fn reveal(self, x: Dim, y: Dim) -> GameState {
        let index = self.to_index(x, y);
        self.reveal_index(index)
    }

    pub fn reveal_index(mut self, ix: usize) -> GameState {
        let mut tile = &mut self.board[ix];
        tile.is_revealed = true;

        if !tile.is_bomb {
            self.auto_reveal_safe_neighbours(ix);
        }

        Game::calculate_state(self)
    }

    fn auto_reveal_safe_neighbours(&mut self, index: usize) {
        if self.board[index].bomb_neighbors > 0 {
            return;
        }

        let all_neighbours: Vec<usize> = Game::neighbour_indices(index, self.width,  self.height);
    
        for n in all_neighbours {
            let mut tile = &mut self.board[n];

            if !tile.is_revealed {
                tile.is_revealed = true;

                if tile.bomb_neighbors == 0 {
                    self.auto_reveal_safe_neighbours(n);
                }
            }
        }
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
            let neighbors = Game::neighbour_indices(i, width, height);
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

    fn neighbour_indices(i: usize, width: usize, height: usize) -> Vec<usize> {
        let on_left_edge = i % width == 0;
        let on_top_row = i < width;
        let on_right_edge = i % width == width - 1;
        let on_bottom_row = i >= (width * (height - 1));

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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_by_one_should_have_no_neighbours() {
        let ix = Game::neighbour_indices(0, 1, 1);
        assert!(ix.len() == 0)
    }

    #[test]
    fn one_by_two_should_have_one_neighbour_above() {
        let ix = Game::neighbour_indices(1, 1, 2);
        assert_eq!(ix.len(), 1);
        assert!(ix[0] == 0)
    }

    #[test]
    fn two_by_two_should_have_three_neighbours() {
        let ix = Game::neighbour_indices(3, 2, 2);
        assert_eq!(ix.len(), 3);
        assert!(ix == vec![1,0,2])
    }
    
}