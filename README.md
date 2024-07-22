# Boggle Solver in Rust

This is a Boggle solver implemented in Rust. It finds all possible words on a given Boggle board.

## How it works

The solver uses a depth-first search algorithm to traverse the Boggle board. It starts from each cell of the board and explores neighboring cells to find valid words.

## Code Overview

The main function is `boggle(board: &[&str], words: &Vec<String>) -> HashMap<String, Vec<(u8, u8)>>`. This function takes a Boggle board and a vector of words as input, and returns a HashMap of words found on the board along with their coordinates.

Here's a brief overview of the helper functions:

- `find_word(board: &Vec<Vec<char>>, word: &String) -> Option<Vec<(u8, u8)>>`: This function finds a word on the board. It returns the coordinates of the word if found, or None if not.
- `traverse(board: &Vec<Vec<char>>, row: usize, col: usize, word: &str, path: Vec<(u8, u8)>) -> Vec<(u8, u8)>`: This function traverses the board from a given cell. It explores all valid directions and returns the path if the word is found.

