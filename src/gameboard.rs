use fltk::{enums::Color, group::Flex, prelude::*, *};

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
    Cell{x_coord:usize,y_coord:usize},
    StartGame,
}

impl GameBoard {
    //accepts the grid dimensions as parameters
    pub fn new(x: u32, y: u32) {
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
        let mut group = group::Group::new(0,0,BOARD_WIDTH,BOARD_WIDTH,"");
        let mut grid = Grid::default_fill();
        grid.set_layout(x as i32, y as i32);

        let (s, r) = app::channel::<Message>();

        for i in 0..x {
            for j in 0..y {
                let mut text = i.to_string();
                text.push_str(",");
                text.push_str(&(j.to_string()));
                let mut btn =
                    button::ToggleButton::new(0, 0, CELL_SIZE as i32, CELL_SIZE as i32, "")
                        .with_label(&text);
                btn.set_color(Color::from_hex(0xfffffff));
                grid.insert(&mut btn, i as i32, j as i32);
                let x_coord = i as usize;
                let y_coord = j as usize;
                btn.emit(s, Message::Cell{ x_coord , y_coord });
            }
        }
        group.end();
        let mut group_2 = group::Group::new(0,BOARD_WIDTH,BOARD_WIDTH,BOARD_HEIGHT-BOARD_WIDTH,"");
        let mut start_btn = button::ToggleButton::new(BOARD_WIDTH/2 - 75,BOARD_HEIGHT-50,150,30,"Start the Game");
        start_btn.set_callback(move |_| s.send(Message::StartGame));
        group_2.end();
        wind.end();
        wind.show();
        while app.wait() {
            if let Some(msg) = r.recv(){
                match msg {
                    Message::Cell{x_coord, y_coord}=> {
                        println!("{},{}", x_coord, y_coord);
                        if board[x_coord][y_coord] == 0 {
                            board[x_coord][y_coord] = 1;
                        } else {
                            board[x_coord][y_coord] = 0;
                        }
                    }
                    Message::StartGame => {
                        println!("TEST");
                        print_board(&board);
                    }
                    _ => (),
                }
            }
            
        }
        // Self { board, wind }
    }

    pub fn toggle_cell(&mut self, x: usize, y: usize) {
        if self.board[x][y] == 0 {
            self.board[x][y] = 1;
        } else {
            self.board[x][y] = 0;
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
