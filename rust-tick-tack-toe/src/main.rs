use std::io;

fn main() {
    let mut input: char;
    loop{
        print!("\x1B[2J\x1B[H"); 
    
        game();

        println!("Do you want to start a new game? [y/N]");
        input = get_user_input();

        if input != 'y' && input != 'Y'{
            break;
        } 
    }
}

fn game(){
    let mut board: [char; 9]  = ['1','2','3','4','5','6','7','8','9'];
    let mut player: bool = false;
    let mut valid_move: bool = false;
    let mut board_condition:u8;
    let mut turn: i32 = 0;
    let mut input: char ;

    //Choosing the starting player
    loop{
        println!("Choose starting player: 1 (O) or 2 (X)");
        input = get_user_input();

        if input == '1' || input == 'O'{
            break;
        } 
        if input == '2' || input == 'X'{
            player = true;
            break;
        } 
        print!("\x1B[2J\x1B[H");
        println!("That wasn't a valid starting player");
    }

    //Proper game loop
    loop {
        print_board(&board);

        //Getting a valid move
        while !valid_move{
            println!("Type your command player {}:",if player { "2 (X)"} else { "1 (O)" });
            input = get_user_input();

            valid_move = check_move_validity(&board, input);
            if !valid_move{
                print_board(&board);
                println!("Move is not valid");
            }
        }
        valid_move = false;

        //Executing the move
        board[input.to_digit(10).unwrap() as usize - 1] = if player {'X'} else {'O'};
        player = !player;

        //Advancing the game and checking the board's condition
        turn += 1;
        board_condition = check_board_condition(&board);
        if turn >= 9 && board_condition == 0{board_condition = 3;}

        if board_condition != 0{break;}
    }
    print_board(&board);
    if board_condition != 3{
        println!("Player {} won!!!!!",if board_condition == 2 { "2 (X)"} else { "1 (O)" });
    } else {
        println!("It's a draw!!!!")
    }
}

fn get_user_input() -> char{
    let mut user_input: String = String::new();

    let _ = io::stdin().read_line(&mut user_input);
    return user_input.chars().nth(0).unwrap();
}


fn check_board_condition(board: &[char; 9]) -> u8{
    let mut player = '-';
    for i in 0..3{
        //Vertical
        if board[i] == board[i+3] && board[i+3] == board[i+6]{
            player = board[i]; break;
        }
        //Horizontal
        if board[i*3] == board[i*3+1] && board[i+3] == board[i*3+2]{
            player = board[i*3]; break;
        }
    }
    //Diagonal
    if board[0] == board[4] && board[4] == board[8]{player = board[0];}
    if board[2] == board[4] && board[4] == board[6]{player = board[0];}

    return if player == '-' {0} else if player == 'O' {1} else {2};
}


fn check_move_validity(board: &[char; 9],player_move: char) -> bool{
    if player_move < '1' || player_move > '9'{return false;}
    let index: usize = player_move.to_digit(10).unwrap() as usize - 1;
    if board[index] != player_move {return false;}
    return true;
}


fn print_board(board: &[char; 9]){
    print!("\x1B[2J\x1B[H");
    for i in 0..3{
        println!("-------------");
        println!("| {} | {} | {} |",board[i*3],board[i*3+1],board[i*3+2]);
    }
    println!("-------------");
}
