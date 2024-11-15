use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut board: [u64; 64] = [0; 64];

    // Initialize the board with a random generation
    initialize_board_randomly(&mut board);

    // Read the number of generations to simulate
    let generations: u32 = read_input("Enter the number of generations: ");

    // Iterate through generations
    for gen in 1..=generations {
        if gen == 1 || gen == generations {
            println!("Generation: {}", gen);
            print_board(&board);
        }
        board = update_board(&board);
    }
}

// Function to randomly initialize the board
fn initialize_board_randomly(board: &mut [u64; 64]) {
    let mut rng = rand::thread_rng();
    for row in 0..64 {
        for col in 0..64 {
            if rng.gen_bool(0.5) {
                board[row] |= 1 << (63 - col);
            }
        }
    }
}

// Helper function to read user input of a specific type
fn read_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("Failed to read line");

        if let Ok(val) = input_line.trim().parse::<T>() {
            return val;
        }
        eprintln!("Invalid input. Please enter a valid number.");
    }
}

// Function to print the board
fn print_board(board: &[u64; 64]) {
    for row in 0..64 {
        for col in 0..64 {
            if board[row] & (1 << (63 - col)) > 0 {
                print!("+ ");
            } else {
                print!("- ");
            }
        }
        println!();
    }
}

// Function to update the board based on the game rules
fn update_board(board: &[u64; 64]) -> [u64; 64] {
    let mut new_board: [u64; 64] = [0; 64];

    // Iterate over each cell on the board
    for row in 0..64 {
        for col in 0..64 {
            let live_neighbors = count_live_neighbors(board, row, col);

            // Determine whether the cell should live or die
            let current_cell_alive = board[row] & (1 << (63 - col)) > 0;
            if current_cell_alive {
                // Cell is alive
                if live_neighbors == 2 || live_neighbors == 3 {
                    new_board[row] |= 1 << (63 - col);  // Cell stays alive
                }
            } else {
                // Cell is dead
                if live_neighbors == 3 {
                    new_board[row] |= 1 << (63 - col);  // Cell becomes alive
                }
            }
        }
    }
    new_board
}

// Function to count the live neighbors of a cell
fn count_live_neighbors(board: &[u64; 64], row: usize, col: usize) -> u8 {
    let mut count = 0;

    // Iterate over the 8 neighbors (including wrapping around)
    for i in -1..=1 {
        for j in -1..=1 {
            // Skip the cell itself
            if i == 0 && j == 0 {
                continue;
            }

            let neighbor_row = (row as isize + i).rem_euclid(64) as usize;
            let neighbor_col = (col as isize + j).rem_euclid(64) as usize;

            if board[neighbor_row] & (1 << (63 - neighbor_col)) > 0 {
                count += 1;
            }
        }
    }

    count
}
