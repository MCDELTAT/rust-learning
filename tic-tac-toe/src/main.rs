// Todo:
// 1. Change vals based off of user row and column selections
// 2. Loop game
// 3. End game checking

use std::io;

fn main() {
    play_game();
}

fn play_game() {
    println!("Let's Play Tic-Tac-Toe.");
    println!("X starts first.");
    let turn = false;
    // create the empty array of values 0 = x, 1 = O
    let mut vals = [' ',' ',' ',' ',' ',' ',' ',' ',' '];
    print_board(vals);
    let (row, column) = ask_move(); // ask and return
    // print pre modify
    print_board(vals);
    modify_vals(&mut vals, row, column, turn); // modify vals
    print_board(vals);
}

fn print_board(vals: [char; 9]) {
    for x in 1..4 {
        println!("|{}|{}|{}|", vals[x*0], vals[x*1], vals[x*2]);
    }
}

fn ask_move() -> (u8, u8) {
    let row = loop {
        println!("Enter row number (1-3): ");
        let mut row = String::new();
        io::stdin().read_line(&mut row)
            .expect("Failed to read line.");

        let row: u8 = match row.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the match above already limits numbers above 255
        // and values that are negative, just check in bounds
        if row > 0 && row <= 3 {
            break row;
        } else {
            continue;
        }
    };

    let column = loop {
        println!("Enter column number (1-3): ");
        let mut column = String::new();
        io::stdin().read_line(&mut column)
            .expect("Failed to read line.");

        let column: u8 = match column.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if column > 0 && column <= 3 {
            break column;
        } else {
            continue;
        }
    };

    // return the validated row and column
    (row, column)
}

fn modify_vals(vals: &mut [char; 9]) {
    println!("Entered here.");
}
