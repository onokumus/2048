use rand::Rng;
use std::io;

fn filter_zero(row: Vec<i32>) -> Vec<i32> {
    row.into_iter().filter(|&value| value != 0).collect()
}

fn merge_tiles(row: Vec<i32>) -> Vec<i32> {
    let mut row = filter_zero(row);
    if row.len() != 0 {
        for i in 0..row.len() - 1 {
            if row[i] == row[i + 1] {
                row[i] = row[i] * 2;
                row[i + 1] = 0;
            }
        }
    }
    let row = filter_zero(row);
    let mut row: Vec<i32> = row.into_iter().collect();
    while row.len() < 4 {
        row.push(0);
    }
    row
}

fn pick_value() -> i32 {
    let random_number = rand::thread_rng().gen_range(1..=100);
    if random_number <= 90 {
        2
    } else {
        4
    }
}

fn add_new_value(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut board = board;
    let mut empty_tiles: Vec<(usize, usize)> = Vec::new();
    for i in 0..4 {
        for j in 0..4 {
            if board[i][j] == 0 {
                empty_tiles.push((i, j));
            }
        }
    }
    if empty_tiles.len() != 0 {
        let random_tile = rand::thread_rng().gen_range(0..empty_tiles.len());
        let (i, j) = empty_tiles[random_tile];
        board[i][j] = pick_value();
    }
    board
}

fn are_vectors_equal(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for j in 0..a.len() {
        if a[j].len() != b[j].len() {
            return false;
        }
        for i in 0..a[j].len() {
            if a[j][i] != b[j][i] {
                return false;
            }
        }
    }
    true
}

fn transpose_board(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed_board: Vec<Vec<i32>> = Vec::new();
    for i in 0..board.len() {
        let mut row: Vec<i32> = Vec::new();
        for j in 0..board.len() {
            row.push(board[j][i]);
        }
        transposed_board.push(row);
    }
    transposed_board
}

fn has_won(board: &Vec<Vec<i32>>) -> bool {
    for i in 0..board.len() {
        for j in 0..board.len() {
            if board[i][j] == 2048 {
                return true;
            }
        }
    }
    false
}

fn merge_left(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut new_board: Vec<Vec<i32>> = Vec::new();
    for row in board {
        new_board.push(merge_tiles(row));
    }
    new_board
}

fn merge_right(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut merged_board: Vec<Vec<i32>> = Vec::new();
    for row in board {
        let mut r = row;
        r.reverse();
        let mut mt = merge_tiles(r);
        mt.reverse();
        merged_board.push(mt);
    }
    merged_board
}

fn merge_up(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let transposed_board = transpose_board(board);
    let merged_board = merge_left(transposed_board);
    transpose_board(merged_board)
}

fn merge_down(board: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let transposed_board = transpose_board(board);
    let merged_board = merge_right(transposed_board);
    transpose_board(merged_board)
}

fn update_board(board: Vec<Vec<i32>>, direction: &str) -> Vec<Vec<i32>> {
    let mut new_board: Vec<Vec<i32>> = Vec::new();
    match direction {
        "a" => new_board = merge_left(board),
        "d" => new_board = merge_right(board),
        "w" => new_board = merge_up(board),
        "s" => new_board = merge_down(board),
        _ => println!("Invalid direction"),
    }
    new_board
}

fn no_moves_left(board: &Vec<Vec<i32>>) -> bool {
    let mut no_moves_left = true;

    let mlboard = merge_left(board.clone());
    if !are_vectors_equal(&board, &mlboard) {
        no_moves_left = false;
    }

    let mrboard = merge_right(mlboard.clone());
    if !are_vectors_equal(&mlboard, &mrboard) {
        no_moves_left = false;
    }

    let muboard = merge_up(board.clone());
    if !are_vectors_equal(&board, &muboard) {
        no_moves_left = false;
    }

    let mdboard = merge_down(board.clone());
    if !are_vectors_equal(&board, &mdboard) {
        no_moves_left = false;
    }

    no_moves_left
}

fn show_board(board: &Vec<Vec<i32>>) {
    for i in 0..board.len() {
        for j in 0..board.len() {
            print!("{} ", board[i][j]);
        }
        println!();
    }
}

fn main() {
    let mut board = vec![
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
        vec![0, 0, 0, 0],
    ];
    board = add_new_value(board);
    board = add_new_value(board);
    println!("Initial board");
    show_board(&board);
    loop {
        let mut direction = String::new();
        println!("Enter direction: ");
        io::stdin()
            .read_line(&mut direction)
            .expect("Failed to read line");
        let direction = direction.trim();
        let new_board = update_board(board.clone(), direction);
        if !are_vectors_equal(&board, &new_board) {
            board = update_board(board, direction);
            board = add_new_value(board);
            show_board(&board);
        } else {
            println!("Invalid move");
        }
        if has_won(&board.clone()) {
            println!("You won!");
            break;
        }
        if no_moves_left(&board.clone()) {
            println!("Game over!");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_zero() {
        let row = vec![0, 2, 0, 4];
        let filtered_row = filter_zero(row);
        assert_eq!(filtered_row, vec![2, 4]);
    }

    #[test]
    fn test_merge_tiles() {
        let row = vec![2, 2, 4, 4];
        let merged_row = merge_tiles(row);
        assert_eq!(merged_row, vec![4, 8, 0, 0]);
    }

    #[test]
    fn test_merge_left() {
        let mut board = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![4, 4, 8, 8],
            vec![4, 4, 8, 8],
        ];
        let expected_board = vec![
            vec![4, 4, 0, 0],
            vec![4, 4, 0, 0],
            vec![8, 16, 0, 0],
            vec![8, 16, 0, 0],
        ];
        board = merge_left(board);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_merge_right() {
        let mut board = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![4, 4, 8, 8],
            vec![4, 4, 8, 8],
        ];
        let expected_board = vec![
            vec![0, 0, 4, 4],
            vec![0, 0, 4, 4],
            vec![0, 0, 8, 16],
            vec![0, 0, 8, 16],
        ];
        board = merge_right(board);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_merge_up() {
        let mut board = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![4, 4, 8, 8],
            vec![4, 4, 8, 8],
        ];
        let expected_board = vec![
            vec![4, 4, 4, 4],
            vec![8, 8, 16, 16],
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
        ];
        board = merge_up(board);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_merge_down() {
        let mut board = vec![
            vec![2, 2, 2, 2],
            vec![2, 2, 2, 2],
            vec![4, 4, 8, 8],
            vec![4, 4, 8, 8],
        ];
        let expected_board = vec![
            vec![0, 0, 0, 0],
            vec![0, 0, 0, 0],
            vec![4, 4, 4, 4],
            vec![8, 8, 16, 16],
        ];
        board = merge_down(board);
        assert_eq!(board, expected_board);
    }

    #[test]
    fn test_has_won() {
        let board = vec![
            vec![2, 4, 8, 16],
            vec![32, 2, 4, 2],
            vec![4, 8, 2, 8],
            vec![16, 4, 8, 2048],
        ];
        assert_eq!(has_won(&board), true);
    }

    #[test]
    fn test_has_not_won() {
        let board = vec![
            vec![2, 4, 8, 16],
            vec![32, 2, 4, 2],
            vec![4, 8, 2, 8],
            vec![16, 4, 8, 2],
        ];
        assert_eq!(has_won(&board), false);
    }

    #[test]
    fn test_transpose_board() {
        let board = vec![
            vec![2, 4, 8, 16],
            vec![32, 2, 4, 2],
            vec![4, 8, 2, 8],
            vec![16, 4, 8, 2048],
        ];
        let expected_board = vec![
            vec![2, 32, 4, 16],
            vec![4, 2, 8, 4],
            vec![8, 4, 2, 8],
            vec![16, 2, 8, 2048],
        ];
        assert_eq!(transpose_board(board), expected_board);
    }
}
