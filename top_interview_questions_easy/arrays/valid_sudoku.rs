/*******************************************************************************

Valid Sudoku

Determine if a 9x9 Sudoku board is valid. Only the filled cells need to be
validated according to the following rules:

Each row must contain the digits 1-9 without repetition.
Each column must contain the digits 1-9 without repetition.
Each of the 9 3x3 sub-boxes of the grid must contain the digits 1-9 without
repetition.

A partially filled sudoku which is valid.

The Sudoku board could be partially filled, where empty cells are filled with
the character '.'.

Example 1:

Input:
[
  ["5","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
Output: true
Example 2:

Input:
[
  ["8","3",".",".","7",".",".",".","."],
  ["6",".",".","1","9","5",".",".","."],
  [".","9","8",".",".",".",".","6","."],
  ["8",".",".",".","6",".",".",".","3"],
  ["4",".",".","8",".","3",".",".","1"],
  ["7",".",".",".","2",".",".",".","6"],
  [".","6",".",".",".",".","2","8","."],
  [".",".",".","4","1","9",".",".","5"],
  [".",".",".",".","8",".",".","7","9"]
]
Output: false
Explanation: Same as Example 1, except with the 5 in the top left corner being 
    modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is
    invalid.

Note:

A Sudoku board (partially filled) could be valid but is not necessarily solvable.
Only the filled cells need to be validated according to the mentioned rules.
The given board contain only digits 1-9 and the character '.'.
The given board size is always 9x9.
********************************************************************************/

impl Solution {
    /// From contains_duplicates.rs
    pub fn contains_duplicate(chars: Vec<char>) -> bool {
        let mut t_chars = chars.clone();
        if(t_chars.len() > 1){
            t_chars.sort();
            for i in 0..(t_chars.len() - 1) {
                if (t_chars[i] == t_chars[i + 1]){
                    return true;
                }   
            }
            //println!("{:?}", t_chars);
        }
        return false;
    }
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut board_is_valid:bool = true;
        let mut current_values: Vec<char> = Vec::new();
        
        for i in 0..9 {
            // Horz check.
            current_values = board[i].clone();
            current_values.retain(|&x| x != '.');
            if (Solution::contains_duplicate(current_values)){
                board_is_valid = false;
                break;
            }
            
            current_values = Vec::new();
            for j in 0..9 {
                current_values.push(board[j][i]);
            }
            
            current_values.retain(|&x| x != '.');
            if (Solution::contains_duplicate(current_values)){
                board_is_valid = false;
                break;
            }
        }
        
        for h in 0..3 {
            for w in 0..3{
                current_values = Vec::new();
                for i in 0..3 {
                    for j in 0..3{
                        current_values.push(board[i + 3*h][j + 3*w]);
                        current_values.retain(|&x| x != '.');
                        
                        if (Solution::contains_duplicate(current_values.clone())){
                            board_is_valid = false;
                            break;
                        }
                    }
                }
            }
        }
        //println!("bool: {:?}", board_is_valid);
        return board_is_valid;
    }
}
