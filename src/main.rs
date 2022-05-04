use std::{env, io, thread, time};

use fltk::{
    app,
    button::Button,
    enums::{Align, Color, Font, FrameType},
    frame::Frame,
    prelude::*,
    table,
    window::Window,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    //see if there are any command line args, otherwise open interactive session
    if args.len() > 1 {
        // will be deprecated. Only for testing purposes.
        startSession(&args);
    } else {
        let mut width = String::new();
        let mut height = String::new();
        println!("Enter the gameboard's width: ");
        io::stdin().read_line(&mut width).unwrap();
        println!("Enter the gameboard's height: ");
        io::stdin().read_line(&mut height).unwrap();
        let width = width.trim().to_string().parse::<u64>().unwrap();
        let height = height.trim().to_string().parse::<u64>().unwrap();
        println!("Creating a {} x {} board", width, height);
        let mut gameBoard = vec![vec![0; height as usize]; width as usize];
    }
}
//this function takes in a gameboard and calculates the next state
fn game_of_life(grid: &Vec<Vec<u8>>) -> &Vec<Vec<u8>> {
    let rows = grid.len();
    let cells = grid[0].len();
    let old = grid;
    for x in 0..rows {
        for y in 0..cells {
            //if if the current cell is at the edge of the board then we only check the indeces on the board
            let mut neighbors = 0;
            for i in -1i8..=1 {
                for j in -1i8..=1 {
                    let new_x = x as i8 + i;
                    let new_y = y as i8 + j;
                    if new_x >= 0 && new_y >= 0 && new_x < rows as i8 && new_y < cells as i8 {
                        neighbors += old[new_x as usize][new_y as usize];
                    }
                }
            }
            neighbors -= old[x][y];
            // println!("cell @ {},{} has {} neighbors", x, y, neighbors);
            if old[x][y] == 0 {
                if neighbors == 3 {
                    grid[x][y] = 1;
                }
            } else {
                if neighbors < 2 {
                    grid[x][y] = 0;
                } else if neighbors == 2 || neighbors == 3 {
                    grid[x][y] = 1;
                } else {
                    grid[x][y] = 0;
                }
            }
        }
    }
}

fn print_board(grid: &Vec<Vec<u8>>) {
    for row in grid {
        for cell in row {
            let symbol = if *cell == 0 { '⬛' } else { '⬜' };
            print!("{}", symbol);
        }
        println!();
    }
    println!();
}

fn startSession(args: &Vec<String>) {
    let mut grid: Vec<Vec<u8>> = vec![
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0 ,0],

    ];
    let mut new_grid = vec![vec![0;grid[0].len()];grid.len()];
    if args[1] == "continuous" {
        print_board(&grid);
        let mut x = 500;
        if args.len() > 2 {
            x = args[2].parse::<u64>().unwrap();
        }
        let x_milliseconds = time::Duration::from_millis(x);
        let mut gen = 1;

        loop {
            thread::sleep(x_milliseconds);
            println!("Generation {}", gen);
            let mut new_grid_ptr = &new_grid;
            new_grid = game_of_life(&grid);
            let grid = new_grid_ptr;
            print_board(&new_grid);
            gen+=1;
        }
    }
    if args[1] == "generate" {
        if args.len() > 2 {
            println!("Initial State: ");
            print_board(&grid);
            let count = args[2].parse::<u32>().unwrap();
            for x in 0..count {
                game_of_life(&mut grid)
            }
            println!("Generation {}", count);
            print_board(&grid);
        }
    }
}

fn toggleCell(grid: &mut Vec<Vec<u8>>, x: u64, y: u64) {
    grid[x as usize][y as usize] = if grid[x as usize][y as usize] == 1 {
        0
    } else {
        1
    };
}

fn showApp() {
    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut wind = Window::new(100, 100, 400, 300, "Hello from rust");
    let mut table = table::TableRow::new(0, 0, 400, 300, "");
    table.set_type(table::TableRowSelectMode::Single);
    table.set_rows(5);
    table.set_cols(5);
    table.set_col_header(true);
    table.set_col_width_all(table.width() / 5);
    table.set_row_height_all(table.height() / 5 - table.col_header_height());
    table.end();
    wind.make_modal(true);
    wind.end();
    wind.show();

    app.run().unwrap();
}
