/*******************************************************************************

Reverse Integer

Given a 32-bit signed integer, reverse digits of an integer.

Example 1:

Input: 123
Output: 321
Example 2:

Input: -123
Output: -321
Example 3:

Input: 120
Output: 21

Note:

Assume we are dealing with an environment which could only store integers
within the 32-bit signed integer range: [−231,  231 − 1]. For the purpose of
this problem, assume that your function returns 0 when the reversed integer
overflows.

 *******************************************************************************/

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut x_str = std::collections::VecDeque::new();
        
        for i in x.to_string().chars() {
            x_str.push_back(i);
        }
        
        let mut is_negative: bool = false;
        if(x_str[0] == '-'){
            is_negative = true;
            x_str.pop_front();
        }
        
        let x_str_len = x_str.len();
        
        for i in 0..x_str_len/2{
            let spare: char = x_str[i];
            x_str[i] = x_str[x_str_len - 1 - i];
            x_str[x_str_len - 1 - i] = spare;
        }
        
        let mut zeros: bool = true;
        while zeros {
            if(x_str[0] == '0' && x_str.len() > 1){
                x_str.pop_front();
            }
            else {
                zeros = false;
            }
        }
        
        if(is_negative){
            x_str.push_front('-');
        }
        
        return (x_str.iter().map(|x| *x)).collect::<String>().parse::<i32>().unwrap_or(0);
    }
}
