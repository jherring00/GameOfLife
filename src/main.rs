use std::{env, io, thread, time};

use fltk::{enums::Color, prelude::*, *};

use fltk_grid::Grid;

const CELL_SIZE: u32 = 35;
const BOARD_WIDTH: i32 = 600;
const BOARD_HEIGHT: i32 = 700;
pub struct GameBoard {
    pub board: Vec<Vec<u8>>,
    pub wind: window::Window,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Cell { x_coord: usize, y_coord: usize },
    StartGame,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    //see if there are any command line args, otherwise open interactive session
    let mut width = String::new();
    let mut height = String::new();
    println!("Enter the gameboard's width: ");
    io::stdin().read_line(&mut width).unwrap();
    println!("Enter the gameboard's height: ");
    io::stdin().read_line(&mut height).unwrap();
    let width = width.trim().to_string().parse::<u32>().unwrap();
    let height = height.trim().to_string().parse::<u32>().unwrap();
    println!("Creating a {} x {} board", width, height);
    new_game(&args, width, height);
}
//this function takes in a gameboard and calculates the next state
fn game_of_life(grid: &mut Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = grid.len();
    let cells = grid[0].len();
    let mut new_grid = vec![vec![0; cells]; rows];
    for x in 0..rows {
        for y in 0..cells {
            //if if the current cell is at the edge of the board then we only check the indeces on the board
            let mut neighbors = 0;
            for i in -1i8..=1 {
                for j in -1i8..=1 {
                    let new_x = x as i8 + i;
                    let new_y = y as i8 + j;
                    if new_x >= 0 && new_y >= 0 && new_x < rows as i8 && new_y < cells as i8 {
                        neighbors += grid[new_x as usize][new_y as usize];
                    }
                }
            }
            neighbors -= grid[x][y];
            // println!("cell @ {},{} has {} neighbors", x, y, neighbors);
            if grid[x][y] == 0 {
                if neighbors == 3 {
                    new_grid[x][y] = 1;
                }
            } else {
                if neighbors < 2 {
                    new_grid[x][y] = 0;
                } else if neighbors == 2 || neighbors == 3 {
                    new_grid[x][y] = 1;
                } else {
                    new_grid[x][y] = 0;
                }
            }
        }
    }
    return new_grid;
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

fn new_game(args: &Vec<String>, x: u32, y: u32) {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    app::background(0xd3, 0xd3, 0xd3);
    let mut board = vec![vec![0; y as usize]; x as usize];
    let mut wind = window::Window::new(
        (x * CELL_SIZE) as i32,
        (y * CELL_SIZE) as i32,
        BOARD_WIDTH,
        BOARD_HEIGHT,
        "Game of Life",
    );
    let group = group::Group::new(0, 0, BOARD_WIDTH, BOARD_WIDTH, "");
    let mut grid = Grid::default_fill();
    grid.set_layout(x as i32, y as i32);

    let (s, r) = app::channel::<Message>();

    for i in 0..x {
        for j in 0..y {
            let mut text = i.to_string();
            text.push_str(",");
            text.push_str(&(j.to_string()));
            let mut btn = button::ToggleButton::new(0, 0, CELL_SIZE as i32, CELL_SIZE as i32, "");
            btn.set_color(Color::from_hex(0xfffffff));
            grid.insert(&mut btn, i as i32, j as i32);
            let x_coord = i as usize;
            let y_coord = j as usize;
            btn.emit(s, Message::Cell { x_coord, y_coord });
        }
    }
    group.end();
    let group_2 = group::Group::new(0, BOARD_WIDTH, BOARD_WIDTH, BOARD_HEIGHT - BOARD_WIDTH, "");
    let mut start_btn = button::ToggleButton::new(
        BOARD_WIDTH / 2 - 75,
        BOARD_HEIGHT - 50,
        150,
        30,
        "Start the Game",
    );
    start_btn.set_callback(move |_| s.send(Message::StartGame));
    group_2.end();
    wind.end();
    wind.show();
    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Cell { x_coord, y_coord } => {
                    println!("{},{}", x_coord, y_coord);
                    if board[x_coord][y_coord] == 0 {
                        board[x_coord][y_coord] = 1;
                    } else {
                        board[x_coord][y_coord] = 0;
                    }
                }
                Message::StartGame => {
                    if args.len() > 1 {
                        if args[1] == "continuous" {
                            print_board(&board);
                            let mut x = 500;
                            if args.len() > 2 {
                                x = args[2].parse::<u64>().unwrap();
                            }
                            let x_milliseconds = time::Duration::from_millis(x);
                            let mut gen = 0;
                            loop {
                                thread::sleep(x_milliseconds);
                                println!("Generation {}", gen);
                                board = game_of_life(&mut board);
                                print_board(&board);
                                gen += 1;
                            }
                        }
                        else if args[1] == "generate"
                        {
                            if args.len()>2{
                                let generation = args[2].parse::<u64>().unwrap();
                                for _x in 0..generation{
                                    board = game_of_life(&mut board);
                                }
                                print_board(&board);

                            }
                        }
                    }
                }
            }
        }
    }
}
