use rand::Rng;
use std::io::{self, Write};

// define size of game board as const
const BOARD_SIZE: usize = 10;

struct Board {
    grid: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    ships: Vec<(usize, usize)>,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Ship,
    Hit,
    Miss,
}

// Implement methods for the Board struct.
impl Board {
    // Constructor for Board, initializes the grid with all cells empty and no ships.
    fn new() -> Self {
        Board {
            grid: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            ships: Vec::new(),
        }
    }

    // Method to randomly place a ship of given size on the board, ensuring it doesn't overlap or go out of bounds.
    fn place_ship(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        loop {
            let row = rng.gen_range(0..BOARD_SIZE);
            let col = rng.gen_range(0..BOARD_SIZE);
            let direction = rng.gen::<bool>();
            // Check if the chosen position can accommodate the ship without overlapping or going out of bounds.
            if self.can_place_ship(row, col, size, direction) {
                for i in 0..size {
                    let (r, c) = if direction {
                        (row, col + i)
                    } else {
                        (row + i, col)
                    };
                    self.grid[r][c] = CellState::Ship;
                    self.ships.push((r, c));
                }
                break;
            }
        }
    }

    // Helper method to check if a ship can be placed at a specified location without conflicts.
    fn can_place_ship(&self, row: usize, col: usize, size: usize, direction: bool) -> bool {
        if direction {
            if col + size > BOARD_SIZE {
                return false;
            }
            for i in 0..size {
                if self.grid[row][col + i] != CellState::Empty {
                    return false;
                }
            }
        } else {
            if row + size > BOARD_SIZE {
                return false;
            }
            for i in 0..size {
                if self.grid[row + i][col] != CellState::Empty {
                    return false;
                }
            }
        }
        true
    }

    // Method for firing at a specified cell, changing its state based on whether a ship is hit or not.
    fn fire(&mut self, row: usize, col: usize) -> CellState {
        match self.grid[row][col] {
            CellState::Empty => {
                self.grid[row][col] = CellState::Miss;
                CellState::Miss
            }
            CellState::Ship => {
                self.grid[row][col] = CellState::Hit;
                CellState::Hit
            }
            _ => CellState::Miss,
        }
    }

    // Method to display the game board, optionally hiding the ships (for the opponent's view).
    fn display(&self, hide_ships: bool) {
        print!("   ");
        for i in 0..BOARD_SIZE {
            print!(" {} ", i);
        }
        println!();
        for (i, row) in self.grid.iter().enumerate() {
            print!("{:2} ", i);
            for cell in row {
                match cell {
                    CellState::Empty => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A1} "); // □ Water
                        }
                    }
                    CellState::Ship => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" \u{25A0} ");
                        }
                    }
                    CellState::Hit => print!("\x1b[31m \u{25CF} \x1b[0m"),
                    CellState::Miss => print!("\x1b[36m \u{00B7} \x1b[0m"),
                }
            }
            println!();
        }
    }

    // Method to determine if all ships have been hit, indicating game over.
    fn is_game_over(&self) -> bool {
        self.ships
            .iter()
            .all(|&(r, c)| self.grid[r][c] == CellState::Hit)
    }
}
fn main() {
    let mut player_board: Board = Board::new();
    let mut opponent_board: Board = Board::new();

    player_board.place_ship(5);
    player_board.place_ship(4);
    player_board.place_ship(3);
    player_board.place_ship(3);
    player_board.place_ship(2);

    opponent_board.place_ship(5);
    opponent_board.place_ship(4);
    opponent_board.place_ship(3);
    opponent_board.place_ship(3);
    opponent_board.place_ship(2);

    // Main game loop
    loop {
        // Clear the screen for a fresh display of the game board each turn
        print!("\x1b[2J\x1b[1;1H");

        // Display the player's board and the opponent's board
        println!("\x1b[1;37mYour Board:\x1b[0m");
        player_board.display(false);
        println!("\x1b[1;37mOpponent's Board:\x1b[0m");
        opponent_board.display(true);
        // Player's turn: prompt for input and process the firing result
        let (player_row, player_col) = get_player_input();
        let result = opponent_board.fire(player_row, player_col);
        match result {
            CellState::Miss => println!("\x1b[36mYou missed!\x1b[0m"),
            CellState::Hit => println!("\x1b[31mYou hit a ship!\x1b[0m"),
            _ => (),
        }
        println!("Press Enter to continue...");
        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");

        // Check if all opponent ships have been sunk
        if opponent_board.is_game_over() {
            println!("\x1b[1;32mCongratulations! You sank all of your opponent's ships!\x1b[0m");
            break;
        }

        // Opponent's turn: simulate opponent move (could be AI-controlled in future enhancements)
        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("\x1b[36mOpponent missed!\x1b[0m"),
            CellState::Hit => println!("\x1b[31mOpponent hit one of your ships!\x1b[0m"),
            _ => (),
        }
        println!("Press Enter to continue...");
        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");

        // Check if all player ships have been sunk
        if player_board.is_game_over() {
            println!("\x1b[1;31mOh no! All of your ships have been sunk!\x1b[0m");
            break;
        }
    }
}

// Function to get player input for firing
fn get_player_input() -> (usize, usize) {
    loop {
        print!("\x1b[1;37mEnter coordinates to fire (row, col): \x1b[0m");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let coordinates: Option<Vec<usize>> = input
            .trim()
            .split(',')
            .map(|s| s.trim().parse().ok())
            .collect();

        if let Some(coordinates) = coordinates {
            if coordinates.len() == 2 && coordinates[0] < BOARD_SIZE && coordinates[1] < BOARD_SIZE
            {
                return (coordinates[0], coordinates[1]);
            } else {
                print_error_message();
            }
        } else {
            print_error_message();
        }
    }
}
fn print_error_message() {
    println!(
        "\x1b[1;31mInvalid input. Please enter row and column numbers separated by a comma.\x1b[0m"
    );
}

// Function to generate a random move for the opponent
fn generate_opponent_move() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}
