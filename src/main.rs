use std::io;
use std::io::Write;

#[derive(Debug)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

const ALL_DIRECTION: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

enum ActivePlayer {
    B,
    W,
}

impl ActivePlayer {
    //returns the opposite player to the active
    fn opposite(&self) -> ActivePlayer {
        match self {
            ActivePlayer::B => ActivePlayer::W,
            ActivePlayer::W => ActivePlayer::B,
        }
    }

    //returns the ennum as a char
    fn to_char(&self) -> char {
        match self {
            ActivePlayer::B => 'B',
            ActivePlayer::W => 'W',
        }
    }
}

struct BoardLocation(isize, isize); //row, col

impl BoardLocation {
    //return new struct, used for move_ fn
    fn new(row: isize, col: isize) -> BoardLocation {
        BoardLocation(row, col)
    }

    //move the location in the direction requested, no bound check
    fn move_in_direction(&self, direction: &Direction) -> BoardLocation {
        match direction {
            Direction::N => BoardLocation::new(self.0 - 1, self.1),
            Direction::NE => BoardLocation::new(self.0 - 1, self.1 + 1),
            Direction::E => BoardLocation::new(self.0, self.1 + 1),
            Direction::SE => BoardLocation::new(self.0 + 1, self.1 + 1),
            Direction::S => BoardLocation::new(self.0 + 1, self.1),
            Direction::SW => BoardLocation::new(self.0 + 1, self.1 - 1),
            Direction::W => BoardLocation::new(self.0, self.1 - 1),
            Direction::NW => BoardLocation::new(self.0 - 1, self.1 - 1),
        }
    }

    //move the location in the opposite direction, no bound check
    fn move_in_opposite(&self, direction: &Direction) -> BoardLocation {
        match direction {
            Direction::N => BoardLocation::new(self.0 + 1, self.1),
            Direction::NE => BoardLocation::new(self.0 + 1, self.1 - 1),
            Direction::E => BoardLocation::new(self.0, self.1 - 1),
            Direction::SE => BoardLocation::new(self.0 - 1, self.1 - 1),
            Direction::S => BoardLocation::new(self.0 - 1, self.1),
            Direction::SW => BoardLocation::new(self.0 - 1, self.1 + 1),
            Direction::W => BoardLocation::new(self.0, self.1 + 1),
            Direction::NW => BoardLocation::new(self.0 + 1, self.1 + 1),
        }
    }
}

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
    let mut active_player = ActivePlayer::B;

    //start main game loop
    'main_loop: loop {
        print_gameboard(&game_board);

        //get input
        let mut input = String::new();
        print!(
            "Enter move for colour {} (RowCol): ",
            active_player.to_char()
        );

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
            println!("Invalid Move. Try again.");
            continue;
        }
        //match chars to index values in game_board arr
        let mut location = [0, 0];

        for (iterator, chars) in input.char_indices() {
            match chars {
                'a' => location[iterator] = 0,
                'b' => location[iterator] = 1,
                'c' => location[iterator] = 2,
                'd' => location[iterator] = 3,
                'e' => location[iterator] = 4,
                'f' => location[iterator] = 5,
                'g' => location[iterator] = 6,
                'h' => location[iterator] = 7,
                _ => {
                    println!("Invalid move. Try again.");
                    continue 'main_loop;
                }
            }
        }

        let location = BoardLocation::new(location[0], location[1]);
        let mut some_peices_changed = false;

        'dir_loop: for direction in ALL_DIRECTION {
            let mut is_valid = false;

            //reset location
            let mut check_location = BoardLocation::new(location.0, location.1); //copy current inputted location
            let mut intermediate_pieces = 0;

            //loop to check if the other side of the line has a matching piece as Active Player
            loop {
                //get next location to check
                check_location = check_location.move_in_direction(&direction);

                //check bounds
                if check_location.0 < 0
                    || check_location.0 > 7
                    || check_location.1 < 0
                    || check_location.1 > 7
                {
                    //debug message
                    //println!("Out of bounds check: {:?}", direction);
                    continue 'dir_loop;
                }

                //check peice at location
                match game_board[check_location.0 as usize][check_location.1 as usize] {
                    'B' => match active_player {
                        ActivePlayer::B => {
                            if intermediate_pieces > 0 {
                                is_valid = true;
                            } else {
                                continue 'dir_loop;
                            }
                        }
                        ActivePlayer::W => {
                            intermediate_pieces += 1;
                            continue;
                        }
                    },
                    'W' => match active_player {
                        ActivePlayer::W => {
                            if intermediate_pieces > 0 {
                                is_valid = true;
                            } else {
                                continue 'dir_loop;
                            }
                        }
                        ActivePlayer::B => {
                            intermediate_pieces += 1;
                            continue;
                        }
                    },
                    '.' => {} //if '.' then its not valid and is_valid stays false
                    _ => println!("nope, this is a bug"),
                }

                //if its not a valid move, go to next direction
                if !is_valid {
                    //debug message
                    //println!("{:?} is not valid", direction);
                    continue 'dir_loop;
                }

                //set location of chosen postion
                game_board[location.0 as usize][location.1 as usize] = active_player.to_char();
                some_peices_changed = true;

                //loop back and update all pieces
                loop {
                    //get next location to update
                    check_location = check_location.move_in_opposite(&direction);

                    //check bounds
                    if check_location.0 < 0
                        || check_location.0 > 7
                        || check_location.1 < 0
                        || check_location.1 > 7
                    {
                        //debug message
                        //println!("Out of bounds Loopback: {:?}", direction);
                        continue 'dir_loop;
                    }

                    //check peice at location
                    match game_board[check_location.0 as usize][check_location.1 as usize] {
                        'B' => match active_player {
                            ActivePlayer::B => continue 'dir_loop,
                            ActivePlayer::W => {
                                game_board[check_location.0 as usize][check_location.1 as usize] =
                                    'W'
                            }
                        },
                        'W' => match active_player {
                            ActivePlayer::W => continue 'dir_loop,
                            ActivePlayer::B => {
                                game_board[check_location.0 as usize][check_location.1 as usize] =
                                    'B'
                            }
                        },
                        _ => println!("nope, this is a bug again"),
                    }
                }
            }
        }

        //if move was not valid
        if !some_peices_changed {
            println!("Invalid move. Try again.");
            continue 'main_loop;
        }

        let mut avail_move = false; //store whether any player has an available move for game over check after

        //check for valid player moves
        for row in 0..=7 {
            'opp_player_chk: for col in 0..=7 {
                'dir_loop: for direction in ALL_DIRECTION {
                    //debug
                    //println!("{row}{col}{:?}", direction);
                    //if current location is '.' then move to next point
                    if game_board[row][col] == '.' {
                        continue 'opp_player_chk;
                    }

                    //reset location
                    let checking_colour = game_board[row][col];
                    let mut check_location = BoardLocation::new(row as isize, col as isize);
                    let mut intermediate_pieces_opp = 0;
                    let mut intermediate_pieces_act = 0;

                    //loop to check if the other side of the line has a '.' and intermediate pieces
                    loop {
                        //get next location to check
                        check_location = check_location.move_in_direction(&direction);

                        //check bounds
                        if check_location.0 < 0
                            || check_location.0 > 7
                            || check_location.1 < 0
                            || check_location.1 > 7
                        {
                            //debug message
                            //println!("{row}{col} Out of bounds check: {:?}", direction);
                            continue 'dir_loop;
                        }

                        //check piece at location
                        match game_board[check_location.0 as usize][check_location.1 as usize] {
                            'B' => match active_player.opposite() {
                                ActivePlayer::W => intermediate_pieces_opp += 1,
                                ActivePlayer::B => {
                                    intermediate_pieces_act += 1;
                                    continue;
                                }
                            },
                            'W' => match active_player.opposite() {
                                ActivePlayer::W => {
                                    intermediate_pieces_act += 1;
                                    continue;
                                }
                                ActivePlayer::B => intermediate_pieces_opp += 1,
                            },
                            '.' => {
                                if intermediate_pieces_opp > 0
                                    && checking_colour == active_player.opposite().to_char()
                                {
                                    //debug msg
                                    //println!("valid move, switching player");
                                    active_player = active_player.opposite();
                                    continue 'main_loop;
                                }
                                if intermediate_pieces_act > 0
                                    && checking_colour == active_player.to_char()
                                {
                                    avail_move = true;
                                }
                                continue 'dir_loop;
                            }
                            _ => println!("nope, this is a bug"),
                        }
                    }
                }
            }
        }
        println!(
            "{} player has no valid move.",
            active_player.opposite().to_char()
        );

        //check for game-end state
        if !avail_move {
            print_gameboard(&game_board);

            //count scores
            let mut score: isize = 0;

            for row in game_board {
                for location in row {
                    match location {
                        'B' => score += 1,
                        'W' => score -= 1,
                        _ => {}
                    }
                }
            }

            match score {
                1.. => println!("Black wins by {} points!", score),
                0 => println!("Draw!"),
                ..0 => println!("White winds by {} points!", score.abs()),
            }

            //close program
            std::process::exit(0);
        }

        //debug
        //println!("position is {}{}", location.0, location.1);
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
