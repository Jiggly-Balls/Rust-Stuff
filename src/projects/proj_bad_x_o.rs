use rand::Rng;
use std::io;

#[allow(dead_code)]
pub fn run() -> () {
    //  0 -> Empty cell
    //  1 -> X Cell
    // -1 -> O Cell
    let mut board: [[i8; 3]; 3] = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
    let mut game_outcome: u8;
    let mut player_turn: bool = false;

    loop {
        player_turn = !player_turn;
        display_board(&board);
        handle_board_turn(&mut board, player_turn);
        print!("\n=====\n\n");
        game_outcome = win_lose(&board);

        if game_outcome == 0 {
            println!("X has won!");
        } else if game_outcome == 1 {
            println!("O has won!");
        } else if game_outcome == 2 {
            println!("It's a tie!");
        }

        if game_outcome != 3 {
            display_board(&board);
            break;
        }
    }
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

fn handle_board_turn(board_arr: &mut [[i8; 3]; 3], player_turn: bool) -> () {
    if player_turn {
        loop {
            let row_input: i8 = get_input("Enter the row: ", 1, 3);
            let col_input: i8 = get_input("Enter the column: ", 1, 3);

            let cell: i8 = board_arr[(row_input - 1) as usize][(col_input - 1) as usize];
            if cell != 0 {
                println!("That cell is already occupied!");
                continue;
            }
            board_arr[(row_input - 1) as usize][(col_input - 1) as usize] = 1;
            break;
        }
    } else {
        loop {
            let npc_row: i32 = rand::rng().random_range(0..3);
            let npc_col: i32 = rand::rng().random_range(0..3);

            let cell: i8 = board_arr[(npc_row - 1) as usize][(npc_col - 1) as usize];
            if cell == 0 {
                board_arr[(npc_row - 1) as usize][(npc_col - 1) as usize] = -1;
                break;
            }
        }
    }
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

fn win_lose(board_arr: &[[i8; 3]; 3]) -> u8 {
    // Returns either one of the following values
    // 0 -> X won
    // 1 -> O won
    // 2 -> Tie
    // 3 -> No winner nor a tie

    let mut total: i8 = 0;
    let mut empty_cell: bool = false;

    // Horizontal check
    for row in board_arr.iter() {
        for col in row.iter() {
            total += col;
            if !empty_cell && *col == 0 {
                empty_cell = true;
            }
        }
        if total == 3 {
            return 0;
        } else if total == -3 {
            return 1;
        }
        total = 0;
    }

    // Vertical check
    for col in 0..3 {
        for row in board_arr.iter() {
            total += row[col];
        }
        if total == 3 {
            return 0;
        } else if total == -3 {
            return 1;
        }
        total = 0;
    }

    // Horizontal check
    let mut diagonal: i8 = board_arr[0][0] + board_arr[1][1] + board_arr[2][2];
    if diagonal == 3 {
        return 0;
    } else if diagonal == -3 {
        return 1;
    }

    diagonal = board_arr[0][2] + board_arr[1][1] + board_arr[2][0];
    if diagonal == 3 {
        return 0;
    } else if diagonal == -3 {
        return 1;
    }

    if empty_cell {
        return 3;
    }
    return 2;
}
