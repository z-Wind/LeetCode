impl Solution {
    pub fn update_board(mut board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        update_board(&mut board, click.into_iter().map(|x| x as usize).collect());
        board
    }
}

fn update_board(board: &mut Vec<Vec<char>>, click: Vec<usize>) {
    let i = click[0];
    let j = click[1];
    if i >= board.len() || j >= board[0].len() {
        return;
    }

    match board[i][j] {
        'M' => board[i][j] = 'X',
        'E' => {
            let adjacents = vec![
                (i - 1, j - 1),
                (i - 1, j),
                (i - 1, j + 1),
                (i, j - 1),
                // (i,j),
                (i, j + 1),
                (i + 1, j - 1),
                (i + 1, j),
                (i + 1, j + 1),
            ];
            let mut mines = b'0';
            for (x, y) in adjacents.iter() {
                mines += check_mine(board, *x, *y);
            }

            if mines != b'0' {
                board[i][j] = mines as char;
            } else {
                board[i][j] = 'B';
                for (x, y) in adjacents.iter() {
                    update_board(board, vec![*x, *y]);
                }
            }
        }
        _ => (),
    }
}

fn check_mine(board: &Vec<Vec<char>>, i: usize, j: usize) -> u8 {
    if i >= board.len() || j >= board[0].len() {
        return 0;
    }

    match board[i][j] {
        'M' => 1,
        _ => 0,
    }
}
