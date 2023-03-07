
use std::io;

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut player = 'X';
    let mut moves_left = 9;
    
    loop {
        print_board(&board);
        println!("Player {}, enter your move (row, col):", player);
        let (row, col) = get_input();
        
        if board[row][col] != ' ' {
            println!("Invalid move, please choose an empty cell.");
            continue;
        }
        
        board[row][col] = player;
        moves_left -= 1;
        
        if check_win(&board, player) {
            print_board(&board);
            println!("Player {} wins!", player);
            break;
        } else if moves_left == 0 {
            print_board(&board);
            println!("The game is a tie!");
            break;
        }
        
        player = if player == 'X' { 'O' } else { 'X' };
    }
}

fn print_board(board: &[[char; 3]; 3]) {
    for row in board.iter() {
        println!(" {} | {} | {}", row[0], row[1], row[2]);
        println!("-----------");
    }
}

fn get_input() -> (usize, usize) {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let mut parts = input.trim().split(",");
        let row = match parts.next() {
            Some(value) => value.parse::<usize>().unwrap(),
            None => continue,
        };
        
        let col = match parts.next() {
            Some(value) => value.parse::<usize>().unwrap(),
            None => continue,
        };
        
        if row >= 3 || col >= 3 {
            println!("Invalid input, please enter a value between 0 and 2.");
            continue;
        }
        
        return (row, col);
    }
}

fn check_win(board: &[[char; 3]; 3], player: char) -> bool {
    for i in 0..3 {
        if board[i][0] == player && board[i][1] == player && board[i][2] == player {
            return true;
        }
        if board[0][i] == player && board[1][i] == player && board[2][i] == player {
            return true;
        }
    }
    
    if board[0][0] == player && board[1][1] == player && board[2][2] == player {
        return true;
    }
    if board[0][2] == player && board[1][1] == player && board[2][0] == player {
        return true;
    }
    
    false
}
