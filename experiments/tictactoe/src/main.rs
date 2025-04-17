use std::{
    fmt::Display,
    io,
    ops::{Index, IndexMut},
};

/*

Potential points to improve:
- Add a config file
- Store previous games in a CSV file
- GUI?
- Better TUI?

*/

type GridLine = [GridValue; 3];

#[derive(Clone, PartialEq)]
enum GridValue {
    X,
    O,
    None,
}

enum GameState {
    Playing,
    Tie,
    Win(GridValue),
}

struct Grid {
    grid: [GridLine; 3],
}

struct GridDraw;

struct Game {
    grid: Grid,
    current_player: GridValue,
}

impl GridValue {
    fn invert(self) -> GridValue {
        match self {
            GridValue::X => GridValue::O,
            GridValue::O => GridValue::X,
            GridValue::None => panic!("Attempted to invert a GridValue that is None."),
        }
    }
}

impl Grid {
    fn new() -> Self {
        Grid {
            grid: [
                [GridValue::None, GridValue::None, GridValue::None],
                [GridValue::None, GridValue::None, GridValue::None],
                [GridValue::None, GridValue::None, GridValue::None],
            ],
        }
    }

    fn is_full(&self) -> bool {
        !(self.get_line(0).contains(&GridValue::None)
            || self.get_line(1).contains(&GridValue::None)
            || self.get_line(2).contains(&GridValue::None))
    }

    fn get_line(&self, index: usize) -> &GridLine {
        assert!(
            (0..=2).contains(&index),
            "Invalid index for 'get_line': needs to be between 0 and 2"
        );
        &self.grid[index]
    }

    fn is_filled_the_same(
        &self,
        index_0: usize,
        index_1: usize,
        index_2: usize,
    ) -> Option<GridValue> {
        match (&self[index_0], &self[index_1], &self[index_2]) {
            (GridValue::O, GridValue::O, GridValue::O) => Some(GridValue::O),
            (GridValue::X, GridValue::X, GridValue::X) => Some(GridValue::X),
            _ => None,
        }
    }
}

impl GridDraw {
    fn draw(grid: &Grid) {
        if cfg!(target_family = "windows") {
            println!("\n┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄┄\n");
        } else {
            print!("{}c", 27 as char);
        }

        println!("Welcome to TicTacToe.rs");
        println!("━━━━━━━━━━━━━━━━━━━━━━━\n");

        println!("┌───┬───┬───┐");
        println!("{}", GridDraw::draw_line(grid, 0));
        println!("│───┼───┼───│");
        println!("{}", GridDraw::draw_line(grid, 1));
        println!("│───┼───┼───│");
        println!("{}", GridDraw::draw_line(grid, 2));
        println!("└───┴───┴───┘");
    }

    fn draw_line(grid: &Grid, line_index: usize) -> String {
        let line = grid.get_line(line_index);

        let value_0 = &line[0];
        let value_1 = &line[1];
        let value_2 = &line[2];

        format!("│ {value_0} │ {value_1} │ {value_2} │")
    }
}

impl Game {
    fn new() -> Self {
        let grid = Grid::new();

        Self {
            grid,
            current_player: GridValue::X,
        }
    }

    fn run(mut self) {
        loop {
            GridDraw::draw(&self.grid);

            let player = self.current_player.clone();
            let cell = self.prompt_cell();
            *cell = player;

            match self.get_game_state() {
                GameState::Playing => {}
                GameState::Tie => {
                    GridDraw::draw(&self.grid);

                    println!("This game is a tie.");
                    break;
                }
                GameState::Win(winner) => {
                    GridDraw::draw(&self.grid);

                    println!("{winner} wins the game.");
                    break;
                }
            }

            self.current_player = self.current_player.invert();
        }
    }

    fn prompt_cell(&mut self) -> &mut GridValue {
        loop {
            println!("{} is playing. Enter a cell number:", self.current_player);

            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            match buffer.trim().parse::<usize>() {
                Ok(num)
                if (1..10).contains(&num) && matches!(self.grid[num - 1], GridValue::None) =>
                    {
                        return &mut self.grid[num - 1];
                    }
                Ok(num) if (1..10).contains(&num) => {
                    println!(
                        "The selected cell is already taken by {}, please choose another",
                        self.grid[num - 1]
                    );
                }
                _ => println!("Please enter a valid number between 1 and 9."),
            }
        }
    }

    fn get_game_state(&self) -> GameState {
        if let Some(winner) = self.get_winner() {
            GameState::Win(winner)
        } else if self.grid.is_full() {
            GameState::Tie
        } else {
            GameState::Playing
        }
    }

    fn get_winner(&self) -> Option<GridValue> {
        // By row
        self.grid
            .is_filled_the_same(0, 1, 2)
            .or_else(|| self.grid.is_filled_the_same(3, 4, 5))
            .or_else(|| self.grid.is_filled_the_same(6, 7, 8))
            // By column
            .or_else(|| self.grid.is_filled_the_same(0, 3, 6))
            .or_else(|| self.grid.is_filled_the_same(1, 4, 7))
            .or_else(|| self.grid.is_filled_the_same(2, 5, 8))
            // By diagonal
            .or_else(|| self.grid.is_filled_the_same(0, 4, 8))
            .or_else(|| self.grid.is_filled_the_same(2, 7, 6))
    }
}

impl Display for GridValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GridValue::X => write!(f, "X"),
            GridValue::O => write!(f, "O"),
            GridValue::None => write!(f, " "),
        }
    }
}

impl Index<usize> for Grid {
    type Output = GridValue;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            (0..=8).contains(&index),
            "Invalid index for 'Grid::Index': needs to be between 0 and 8"
        );
        let column: usize = index % 3;
        let line: usize = index / 3;

        &self.grid[line][column]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(
            (0..=8).contains(&index),
            "Invalid index for 'Grid::IndexMut': needs to be between 0 and 8"
        );
        let column: usize = index % 3;
        let line: usize = index / 3;

        &mut self.grid[line][column]
    }
}

fn main() {
    let game = Game::new();
    game.run();
}
