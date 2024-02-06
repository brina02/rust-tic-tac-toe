use std::io;

fn main() {
    let mut board: [[char; 3]; 3] = [
       ['a', 'a', 'a'],
       ['a', 'a', 'a'],
       ['a', 'a', 'a']
    ];
    print_board(&board);

    for value in 0..8 {
        get_user_input(&mut board, value);
        print_board(&board);
        let winner = check_for_winner(&board);
        if winner == true {
            println!("Congrats Player {} Wins!!", (value % 2) + 1);
            break;
        }
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    for (row_index, row) in board.iter().enumerate() {
        println!("-------------");
        for (element_index, &element) in row.iter().enumerate() {
            if element != 'a' {
                print!("| {} ", element);
            } else {
                print!("| {} ", (row_index * 3) + element_index);
            } 
        }
        println!("|");
    }
     println!("-------------");
}

fn check_for_winner(board: &[[char; 3]; 3]) -> bool {
    if board[0][0] != 'a' && board[0][0] == board[0][1] && board[0][0] == board[0][2] {
        return true;
    } else if board[1][0] != 'a' && board[1][0] == board[1][1] && board[1][0] == board[1][2] {
        return true;
    } else if board[2][0] != 'a' && board[2][0] == board[2][1] && board[2][0] == board[2][2] {
        return true;
    } else if board[0][0] != 'a' && board[0][0] == board[1][0] && board[0][0] == board[2][0] {
        return true;
    } else if board[0][1] != 'a' && board[0][1] == board[1][1] && board[0][1] == board[2][1] {
        return true;
    } else if board[0][2] != 'a' && board[0][2] == board[1][2] && board[0][2] == board[2][2] {
        return true;
    } else if board[0][0] != 'a' && board[0][0] == board[1][1] && board[0][0] == board[2][2] {
        return true; 
    } else if board[0][2] != 'a' && board[0][2] == board[1][1] && board[0][2] == board[2][0] {
        return true;
    }

    false
}

fn get_user_input(board: &mut [[char; 3]; 3], value: i32) {
    let mut player = 1;
    let mut player_symbol = 'X';
    if value % 2 != 0 {
        player = 2;
        player_symbol = 'O';
    }
    loop {
        println!("Player {} make your move (0-8):", player);

        let mut input = String::new();
    
        io::stdin().read_line(&mut input).expect("Failed to read line.");
    
        let trimmed_input = input.trim();
        
        // Parse the input as an integer
        match trimmed_input.parse::<u8>() {
            Ok(value) => {
                // Check if the value is within the specified range
                if value <= 8 {
                    let row_index = (value / 3) as usize;
                    let element_index = (value % 3) as usize;
                    if board[row_index][element_index] == 'a' {
                        board[row_index][element_index] = player_symbol;
                        break;
                    } else {
                        println!("Error: This position has already been taken.");
                    }   
                } else {
                    // Throw an error if the value is outside the range
                    println!("Error: Value must be between 0 and 8.");
                }
            }
            Err(_) => {
                // Throw an error if the input cannot be parsed as an integer
                println!("Error: Invalid input. Please enter a valid integer.");
            }
        }
    }
}