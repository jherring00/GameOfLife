fn main() {

    let grid: Vec<Vec<u8>> = vec![
        vec![0,0,0,0],
        vec![0,0,0,0],
        vec![0,0,0,0],
        vec![0,0,0,0],
    ];

    let new_state = game_of_life(&grid);
}

//this function takes in a gameboard and calculates the next state
fn game_of_life(grid:&Vec<Vec<u8>>)->Vec<Vec<u8>>{
    printBoard(grid);
    let rows = grid.len();
    let cells = grid[0].len();
    let mut new_board: Vec<Vec<u8>> = vec![ vec![0;rows];cells];
    for x in 0..rows{
        for y in 0..cells{
            // println!("{},{}", x,y)
            
        }
    }
    return new_board;
}


fn printBoard(grid:&Vec<Vec<u8>>){
    for row in grid{
        for cell in row{
            print!("{} ",cell);
        }
        println!();
    }
}