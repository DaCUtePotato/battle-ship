use colored::Colorize;
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
                            print!(" □ "); // □ Water
                        }
                    }
                    CellState::Ship => {
                        if hide_ships {
                            print!("   ");
                        } else {
                            print!(" ■ ");
                        }
                    }
                    CellState::Hit => print!(" {} ", "●".red()),
                    CellState::Miss => print!(" {} ", "·".cyan()),
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
        println!("{}", "Your Board:".bold());
        player_board.display(false);
        println!("{}", "Opponent's Board:".bold());
        opponent_board.display(true);
        // Player's turn: prompt for input and process the firing result
        let (player_row, player_col) = get_player_input();
        let result = opponent_board.fire(player_row, player_col);
        match result {
            CellState::Miss => println!("{}", "You missed!".cyan()),
            CellState::Hit => println!("{}", "You hit a ship!".red()),
            _ => (),
        }
        println!("Press Enter to continue...");
        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");

        // Check if all opponent ships have been sunk
        if opponent_board.is_game_over() {
            println!(
                "{}",
                "Congratulations! You sank all of your opponent's ships!"
                    .bold()
                    .green()
            );
            break;
        }

        // Opponent's turn: simulate opponent move (could be AI-controlled in future enhancements)
        let (opponent_row, opponent_col) = generate_opponent_move();
        let result = player_board.fire(opponent_row, opponent_col);
        match result {
            CellState::Miss => println!("{}", "Opponent missed!".cyan()),
            CellState::Hit => println!("{}", "Opponent hit one of your ships!".red()),
            _ => (),
        }
        println!("Press Enter to continue...");
        io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");

        // Check if all player ships have been sunk
        if player_board.is_game_over() {
            println!(
                "{}",
                "Oh no! All of your ships have been sunk!".bold().red()
            );
            break;
        }
    }
}

// Function to get player input for firing
fn get_player_input() -> (usize, usize) {
    loop {
        print!("{}", "Enter coordinates to fire (row, col): ".bold());
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
        "{}",
        "Invalid input. Please enter row and column numbers separated by a comma."
            .bold()
            .red()
    );
}

// Function to generate a random move for the opponent
fn generate_opponent_move() -> (usize, usize) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..BOARD_SIZE), rng.gen_range(0..BOARD_SIZE))
}
