use std::io;

#[allow(dead_code)]
pub fn run() -> () {
    let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    display_board(&board);
    handle_board_input(&mut board);
}

fn display_board(board_arr: &[[i8; 3]; 3]) -> () {
    for row in board_arr.iter() {
        for col in row.iter() {
            if *col == 0 {
                print!("- ");
            } else if *col == 1 {
                print!("X ");
            } else if *col == -1 {
                print!("O ");
            }
        }
        print!("\n");
    }
}

fn handle_board_input(board_arr: &mut [[i8; 3]; 3]) -> () {
    let row_input: i8 = get_input("Enter the row: ", 1, 3);
    let col_input: i8 = get_input("Enter the column: ", 1, 3);

    board_arr[row_input - 1][col_input - 1] = 1;
}

fn get_input(prompt: &str, min_range: i8, max_range: i8) -> i8 {
    loop {
        let mut input_data: String = String::new();

        println!("{prompt}");

        io::stdin()
            .read_line(&mut input_data)
            .expect("Couldn't read the input");

        let input_data: i8 = match input_data.trim().parse() {
            Ok(input_data) => input_data,
            Err(_) => {
                println!("Enter a number from 1-3!");
                continue;
            }
        };

        if input_data < min_range || input_data > max_range {
            println!("Enter a number from 1-3!");
            continue;
        }

        return input_data;
    }
}
