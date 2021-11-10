#[derive(Debug, Eq, PartialEq)]
enum Player {
    First,
    Second
}

type Board = [[char; 3]; 3];

fn clean_board(board: &mut Board) {
    for row in board.iter_mut() {
        *row = [' '; 3];
    }
}

fn draw_board(board: Board) {
    let size: usize = board.len();
    let mut border = "*---".repeat(size);
    border.push('*');
    let space = ' ';

    println!("");
    println!("{}", border);
    for row in board {
        let mut display_row: String = '|'.to_string();
        for cell in row {
            if cell == space {
                display_row.push_str("   ");
            } else {
                display_row.push(space);
                display_row.push(cell);
                display_row.push(space);
            }
            display_row.push('|');

        }
        println!("{}", display_row);
        println!("{}", border);
    }
}


fn print_winner(player: Player) {
    let player = match player {
        Player::First => "1",
        Player::Second => "2",
    };

    println!("\n{}", "***".repeat(4));
    println!("Player {} won!", player);
    println!("{}\n", "***".repeat(4));
}
fn check_winner(player: Player, board: Board) -> bool {
    let player_char = if player == Player::First { 'X' } else { 'O' };

    for row in board {
        if row[0] == player_char && row[1] == player_char && row[2] == player_char {
            return true;
        }
    }
    for i in 0..3 {
        if board[0][i] == player_char && board[1][i] == player_char && board[2][i] == player_char {
            return true;
        }
    }


    if board[2][0] == player_char && board[1][1] == player_char && board[0][2] == player_char {
        return true;
    }

    if board[0][0] == player_char && board[1][1] == player_char && board[2][2] == player_char {
        return true;
    }
    false
}

fn main() {
    println!("Hello, tic-tac-toe! \n");

    let mut board = [[' '; 3]; 3];
    let mut current_turn: Player = Player::First;

    loop {
        println!("Choose an option:");
        println!("1. Play");
        println!("2. Exit");
        println!(">");

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();

        match line.trim() {
            "1" => {
                clean_board(&mut board);
                draw_board(board);

                loop {
                    let mut input = String::new();
                    println!("Row:");
                    std::io::stdin().read_line(&mut input).unwrap();

                    let row: usize = input.trim().parse().unwrap();
                    if row == 9 {
                        break;
                    }

                    let mut input = String::new();
                    println!("Column:");
                    std::io::stdin().read_line(&mut input).unwrap();

                    let column: usize = input.trim().parse().unwrap();
                    if column == 9 {
                        break;
                    }

                    if board[row][column] == ' ' {
                        if current_turn ==  Player::First {
                            current_turn = Player::Second;
                            board[row][column] = 'X';
                        } else {
                            current_turn = Player::First;
                            board[row][column] = 'O';
                        }
                    } else {
                       println!("Cell already taken!");
                    }


                    draw_board(board);
                    if check_winner(Player::First, board) {
                        print_winner(Player::First);
                        break;

                    }
                    if check_winner(Player::Second, board) {
                        print_winner(Player::Second);
                        break;
                    }
                }
            },
            "2" => {
                println!("BYE!");
                break;
            },
            _ => {}
        }
    }
}
