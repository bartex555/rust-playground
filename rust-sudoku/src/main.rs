fn main() {
    let board = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];
    println!("{}",check_sudoku_board(board));
}




fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {

   
    for i in 0..9{
        let mut numbersV: [u8; 10] = [0,0,0,0,0,0,0,0,0,0];
        let mut numbersH: [u8; 10] = [0,0,0,0,0,0,0,0,0,0];
        for j in 0..9{
            if board[j][i] > 9 || board[i][j] > 9{return false;}
            numbersV[board[j][i] as usize] += 1;
            numbersH[board[i][j] as usize] += 1;
        }
        if check_numbers_wrong(numbersV) || check_numbers_wrong(numbersH) {return false;}
    }

    for i in 0..3{
        for j in 0..3{
            let mut numbers: [u8; 10] = [0,0,0,0,0,0,0,0,0,0];
            for y in 0..3{
                for x in 0..3{
                    numbers[board[i*3+y][j*3+x] as usize] += 1;
                }
            }
            if check_numbers_wrong(numbers){return false;}
        }
    }

    true
}

fn check_numbers_wrong(numbers: [u8; 10]) -> bool{
    for i in 1..10{
        if numbers[i] > 1{return true;}
    }
    return false;
}