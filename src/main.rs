use std::{thread,time};
fn main() {
    let mut grid: Vec<Vec<u8>> = vec![
        vec![0, 1, 0],
        vec![0, 1, 0],
        vec![0, 1, 0],
    ];
    let ten_millis = time::Duration::from_millis(1000);
    print_board(&grid);

    loop{
        thread::sleep(ten_millis);
        game_of_life(&mut grid);
        print_board(&grid);
    }
    

}
//this function takes in a gameboard and calculates the next state
fn game_of_life(grid: &mut Vec<Vec<u8>>){
    let rows = grid.len();
    let cells = grid[0].len();
    let old = grid.clone();
    for x in 0..rows {
        for y in 0..cells {
            //if if the current cell is at the edge of the board then we only check the indeces on the board
            let mut neighbors = 0;
            for i in -1i8..=1{
                for j in -1i8..=1{
                    let new_x = x as i8 + i;
                    let new_y = y as i8 + j;
                    if new_x >= 0 && new_y >= 0 && new_x < rows as i8 && new_y < cells as i8 {
                        neighbors+=old[new_x as usize][new_y as usize];
                    }
                }

            }
            neighbors-=old[x][y];
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
    println!("{}", "X ".repeat(grid.len() + 2));
    for row in grid {
        print!("{}", "X ");
        for cell in row {
            print!("{} ", cell);
        }
        print!("{}", "X");
        println!();
    }
    println!("{}", "X ".repeat(grid.len() + 2));
    println!();
}
