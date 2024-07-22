#![allow(non_snake_case, non_camel_case_types, dead_code)]

/*

   Student name: Nilin Reza
   Student ID: 501214241
   Student moon userid: nreza

*/

use std::collections::HashMap;

fn boggle(board: &[&str], words: &Vec<String>) -> HashMap<String, Vec<(u8, u8)>> {
    let mut found: HashMap<String, Vec<(u8, u8)>> = HashMap::new();

    // Convert the board to a 2D vector of characters
    let board: Vec<Vec<char>> = board.iter().map(|&row| row.chars().collect()).collect();

    for word in words {
        if let Some(coordinates) = find_word(&board, word) {
            found.insert(word.clone(), coordinates);
        }
    }

    found
}

fn find_word(board: &Vec<Vec<char>>, word: &String) -> Option<Vec<(u8, u8)>> {
    for (row_index, row) in board.iter().enumerate() {
        for (col_index, &cell) in row.iter().enumerate() {
            if !word.is_empty() && cell == word.chars().nth(0).unwrap() {
                let path = traverse(
                    board,
                    row_index,
                    col_index,
                    &word[1..],
                    vec![(row_index as u8, col_index as u8)],
                );
                if !path.is_empty() {
                    return Some(path);
                }
            }
        }
    }

    None
}

fn traverse(
    board: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    word: &str,
    path: Vec<(u8, u8)>,
) -> Vec<(u8, u8)> {
    if word.is_empty() {
        return path;
    }

    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for (dx, dy) in &directions {
        let new_row = (row as isize) + dx;
        let new_col = (col as isize) + dy;

        if new_row >= 0
            && new_row < board.len() as isize
            && new_col >= 0
            && new_col < board[0].len() as isize
        {
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            let new_pos = (new_row as u8, new_col as u8);

            if board[new_row][new_col] == word.chars().nth(0).unwrap() && !path.contains(&new_pos) {
                let mut new_path = path.clone();
                new_path.push(new_pos);
                let result = traverse(board, new_row, new_col, &word[1..], new_path);

                if !result.is_empty() {
                    return result;
                }
            }
        }
    }

    vec![]
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;