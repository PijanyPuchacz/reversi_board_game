use std::io;
use std::io::Write;

fn main() {
    //intialize game board
    let mut game_board = [['.'; 8]; 8];

    game_board[3][3] = 'W';
    game_board[4][4] = 'W';
    game_board[3][4] = 'B';
    game_board[4][3] = 'B';

    //test [row][column]
    //game_board[6][2] = 'T';

    //black starts game
    let active_player = 'B';

    //start main game loop
    'main_loop: loop {
        print_gameboard(&game_board);

        //get input
        let mut input = String::new();
        print!("Enter move for colour {active_player} (RowCol): ");

        io::stdout().flush().expect("Failed to flush stdout.");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        //remove trailing "\n"
        if input.ends_with("\n") {
            input.pop();
        }

        //test
        //println!("input is: {input}!");

        //if the input is NOT 2 chars long
        if input.chars().count() != 2 {
            println!("Invalid input. Try again.");
            continue;
        }
        //match chars to index values in game_board arr
        let mut position = [0, 0];

        for (iterator, chars) in input.char_indices() {
            match chars {
                'a' => position[iterator] = 0,
                'b' => position[iterator] = 1,
                'c' => position[iterator] = 2,
                'd' => position[iterator] = 3,
                'e' => position[iterator] = 4,
                'f' => position[iterator] = 5,
                'g' => position[iterator] = 6,
                'h' => position[iterator] = 7,
                _ => {
                    println!("Invalid move. Try again");
                    continue 'main_loop;
                }
            }
        }

        //verify is move is valid
        //check at least one neighboring position is the opposite colour
        //check for each position found above that far side has matching colour
        //update middle position

        //check if next player has valid moves availble
        //if yes, switch player and continue
        //check if current player has valid moves availble
        //if no, determine winner

        println!("position is {}{}", position[0], position[1]);
    }
}

//prints the current board state
fn print_gameboard(gm_brd: &[[char; 8]; 8]) {
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    print!("  a  b  c  d  e  f  g  h ");
    for (i, row) in gm_brd.iter().enumerate() {
        print!("\n{}", chars[i]);
        for column in row {
            print!(" {column} ");
        }
    }
    print!("\n");
}
