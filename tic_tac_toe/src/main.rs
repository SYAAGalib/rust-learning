use std::io;

const PLAYER_X: char = 'X';
const PLAYER_O: char = 'O';
const BOARD_SIZE: usize = 3;

type Board = [[char; BOARD_SIZE]; BOARD_SIZE];


struct NewBoard([[char; BOARD_SIZE]; BOARD_SIZE]);


impl NewBoard {
    fn initial_board() -> Board {
        [[' '; BOARD_SIZE]; BOARD_SIZE]
    }

    fn print_board(&self) {
        for row in self.0.iter() {
            for cell in row.iter() {
                print!("{} ", cell);
            }
            println!();
        }
    }

    fn is_full(&self) -> bool {
        for row in self.0.iter() {
            for cell in row.iter() {
                if *cell == ' ' {
                    return false;
                }
            }
        }
        true
    }
}


struct GamePlay {
    board: NewBoard,
    player: char,
}

impl GamePlay {
    fn play(&mut self) {
        loop {
            self.board.print_board();
            println!("Player {}'s turn", self.player);
            let (row, col) = self.get_move();
            self.board.0[row][col] = self.player;
            if self.is_winner() {
                self.board.print_board();
                println!("Player {} wins!", self.player);
                break;
            } else if self.board.is_full() {
                self.board.print_board();
                println!("It's a draw!");
                break;
            }
            self.player = if self.player == PLAYER_X {
                PLAYER_O
            } else {
                PLAYER_X
            };
        }
    }

    fn get_move(&self) -> (usize, usize) {
        loop {
            println!("Enter your move (row[1-3] column[1-3]): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let coords: Vec<&str> = input.trim().split(' ').collect();
            if coords.len() != 2 {
                println!("Please enter two numbers");
                continue;
            }
            let row: usize = match coords[0].parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };
            let col: usize = match coords[1].parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    println!("Please enter a number");
                    continue;
                }
            };
            if row >= BOARD_SIZE || col >= BOARD_SIZE {
                println!("Please enter a number between 1 and 3");
                continue;
            }
            if self.board.0[row][col] != ' ' {
                println!("Cell already taken");
                continue;
            }
            return (row, col);
        }
    }


    fn is_winner(&self) -> bool {
        for i in 0..BOARD_SIZE {
            if self.board.0[i][0] != ' ' && self.board.0[i][0] == self.board.0[i][1] && self.board.0[i][0] == self.board.0[i][2] {
                return true;
            }
            if self.board.0[0][i] != ' ' && self.board.0[0][i] == self.board.0[1][i] && self.board.0[0][i] == self.board.0[2][i] {
                return true;
            }
        }
        if self.board.0[0][0] != ' ' && self.board.0[0][0] == self.board.0[1][1] && self.board.0[0][0] == self.board.0[2][2] {
            return true;
        }
        if self.board.0[0][2] != ' ' && self.board.0[0][2] == self.board.0[1][1] && self.board.0[0][2] == self.board.0[2][0] {
            return true;
        }
        false
    }
}

fn main() {
    let mut game = GamePlay {
        board: NewBoard(NewBoard::initial_board()),
        player: PLAYER_X,
    };
    game.play();
}