use std::io;
use minesweeper::*;

fn main() {
  let mut s = Game::new_random(5, 5, 2);
  loop {
    match s {
      GameState::InProgress(g) => {
        print_board(&g);
        let mut ix = String::new();

        io::stdin()
            .read_line(&mut ix)
            .expect("Failed to read line");
        let ix: usize = ix.trim().parse().expect("Please input a number");
        s = g.reveal_index(ix);
      },
      GameState::Won(_) => { println!("You Won!"); break; },
      GameState::Lost(_) => { println!("You Lost!"); break; },
    }
  }
}

fn print_board(game: &Game) {
  let line = "x".repeat(game.width * 2 + 1);
  
  println!("{}", line);

  for row in game.board.chunks_exact(game.width) {
    let mut r = String::new();
    for tile in row {
      r.push_str(format!("x{}", &render_tile(tile)).as_str());
    }

    r.push_str("x");
    
    println!("{}", r);
    println!("{}", line);
  }
}

fn render_tile(tile: &Tile) -> String {
  if tile.is_bomb && tile.is_revealed { return "*".to_string(); }
  if tile.is_revealed { return tile.bomb_neighbors.to_string(); }
  
  "?".to_string()
}