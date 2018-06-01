// Todo:
// 4. verify that a spot isn't taken already

use std::io;

fn main() {
    play_game();
}

fn play_game() {
    println!("Let's Play Tic-Tac-Toe.");
    println!("X starts first. Here are the grid numbers.");
    let mut turn = false;
    let mut move_count = 0;
    // print the values of the boxes for the players to know
    let cells = ['1','2','3','4','5','6','7','8','9'];
    print_board(cells);
    // create the empty array of values 0 = x, 1 = O
    let mut vals = [' ',' ',' ',' ',' ',' ',' ',' ',' '];
    loop {
        let cell = ask_move(vals); // ask and return
        move_count += 1;
        modify_vals(&mut vals, cell, turn); // modify vals
        print_board(vals);
        if move_count >= 5 { // no one can win until at least the 5th move
            if check_win(vals) {
            // the game is over so exit and print winner
                if turn {
                    println!("O has won!");
                    break;
                } else {
                    println!("X has won!");
                    break;
                }
            } else if move_count == 9 && !check_win(vals) {
                // no one won so cat's game
                println!("Cat's Game! Try again.");
                break;
            };
        }
        turn = !turn; // it is now the other players turn
    };
}

fn print_board(vals: [char; 9]) {
    println!("|{}|{}|{}|", vals[0], vals[1], vals[2]);
    println!("|{}|{}|{}|", vals[3], vals[4], vals[5]);
    println!("|{}|{}|{}|", vals[6], vals[7], vals[8]);
}

fn ask_move(vals: [char; 9]) -> (usize) {
    let cell = loop {
        println!("Enter cell number (1-9): ");
        let mut cell = String::new();
        io::stdin().read_line(&mut cell)
            .expect("Failed to read line.");

        let cell: usize = match cell.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the match above already limits numbers above 255
        // and values that are negative, just check in bounds
        if cell > 0 && cell <= 9 {
            if vals[cell-1] != ' ' { // if the spot is already taken
                continue;
            } else { // use that value
                break cell;
            }
        } else {
            continue;
        }
    };

    // return the validated cell and column
    (cell)
}

fn modify_vals(vals: &mut [char; 9], cell: usize, turn: bool) {
    let array_index = cell-1;
    if !turn { // it's X's turn
        vals[array_index] = 'X';
    } else {
        vals[array_index] = 'O';
    }
}

fn check_win(vals: [char; 9]) -> (bool) {
    let mut win = false;
    // check columns (0,3,6) (1,4,7) (2,5,8)
    for i in 0..3 {
        if vals[i] == vals[i+3] && vals[i] == vals[i+6] && vals[i] != ' ' {
            // winning condition - exit
            win = true;
            break;
        }
    }

    // check rows (0,1,2) (3,4,5) (6,7,8)
    for i in 0..9 {
        if i == 0 || i == 3 || i == 6 {
            if vals[i] == vals[i+1] && vals[i] == vals[i+2] && vals[i] != ' '{
                // winning condition - exit
                win = true;
                break;
            }
        }
    }

    // check diag (0, 4, 8)
    if vals[0] == vals[4] && vals[0] == vals[8] && vals[0] != ' '{
        // winning condition - exit
        win = true;
    }

    //check anti-diag (2, 4, 6)
    if vals[2] == vals[4] && vals[2] == vals[6] && vals[2] != ' ' {
        // winning condition - exit
        win = true;
    }

    (win)
}