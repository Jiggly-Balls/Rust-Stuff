#[allow(dead_code)]
pub fn run() {
    let board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    display_board(&board);
}

fn display_board(board_arr: &[[i8; 3]; 3]) -> () {
    for row in board_arr.iter() {
        for col in row.iter() {
            if *col == 0 {
                print!("- ");
            } else if *col == 1 {
                print!("X ");
            } else {
                print!("O ");
            }
        }
        print!("\n");
    }
}
