/*******************************************************************************

String to Integer

Implement atoi which converts a string to an integer.

The function first discards as many whitespace characters as necessary until the
first non-whitespace character is found. Then, starting from this character,
takes an optional initial plus or minus sign followed by as many numerical digits
as possible, and interprets them as a numerical value.

The string can contain additional characters after those that form the integral
number, which are ignored and have no effect on the behavior of this function.

If the first sequence of non-whitespace characters in str is not a valid integral
number, or if no such sequence exists because either str is empty or it contains
only whitespace characters, no conversion is performed.

If no valid conversion could be performed, a zero value is returned.

Note:

Only the space character ' ' is considered as whitespace character.
Assume we are dealing with an environment which could only store integers
within the 32-bit signed integer range: [−231,  231 − 1]. If the numerical value
is out of the range of representable values, INT_MAX (231 − 1) or INT_MIN (−231)
is returned.

Example 1:

Input: "42"
Output: 42

Example 2:

Input: "   -42"
Output: -42
Explanation: The first non-whitespace character is '-', which is the minus sign.
Then take as many numerical digits as possible, which gets 42.

Example 3:

Input: "4193 with words"
Output: 4193
Explanation: Conversion stops at digit '3' as the next character is not a
numerical digit.

Example 4:

Input: "words and 987"
Output: 0
Explanation: The first non-whitespace character is 'w', which is not a numerical 
digit or a +/- sign. Therefore no valid conversion could be
performed.

Example 5:

Input: "-91283472332"
Output: -2147483648
Explanation: The number "-91283472332" is out of the range of a 32-bit signed
integer.
Thefore INT_MIN (−231) is returned.


 *******************************************************************************/
use std::collections::VecDeque;

impl Solution {
    pub fn my_atoi(str: String) -> i32 { 

        let mut str_vec: VecDeque<char> = str.chars().collect();
        let mut my_vec: VecDeque<char> =  VecDeque::new();
        let mut is_neg: bool = false;
        let mut has_started: bool = false;
        let mut is_overflow: bool = false;
        let mut num: i32 = 0;
 
        loop {
            if(str_vec.len() <= 0){
                break;
            }
            else if((str_vec[0] == '+' || str_vec[0] == '-') && has_started){
                break;
            }
            else if(str_vec[0] == '-'){
                is_neg = true;
                str_vec.pop_front();
                has_started = true;
            }
            else if(str_vec[0] == '+'){
                str_vec.pop_front();
                has_started = true;
            }
            else if(str_vec[0] == ' ' && !has_started){
                str_vec.pop_front();
            }
            else if(!str_vec[0].is_numeric() || (str_vec[0] == ' ' && has_started)){
                break;
            }
            else if (str_vec[0].is_numeric()){
                my_vec.push_back(str_vec[0]);
                str_vec.pop_front();
                has_started = true;
            }
        }
        
        for i in 0..my_vec.len(){
            let dig = match my_vec[i] {
                '1' => 1 as i32,
                '2' => 2 as i32,
                '3' => 3 as i32,
                '4' => 4 as i32,
                '5' => 5 as i32,
                '6' => 6 as i32,
                '7' => 7 as i32,
                '8' => 8 as i32,
                '9' => 9 as i32,
                '0' => 0 as i32,
                _ => return 0 as i32 // Shouldn't be possible.
            };
            
            if dig == 0 {continue;}
            
            match (10_i32).checked_pow((my_vec.len() - 1 - i) as u32) {
                Some(n) =>
                    match dig.checked_mul(10_i32.pow((my_vec.len() - 1 -i) as u32)) {
                        Some(m) => 
                            match num.checked_add(dig*10_i32.pow((my_vec.len() - 1 - i) as u32)){
                                Some(l) => num = num + dig*10_i32.pow((my_vec.len() - 1 -i) as u32),
                                None => is_overflow = true
                        },
                        None => is_overflow = true
                },
                None => is_overflow = true
            };
            
            if(is_overflow){
                if(is_neg){
                    return std::i32::MIN;
                }
                else{
                    return std::i32::MAX;
                }
            }
        }
        
        if(is_neg){
            num = num * -1;
        }

        return num;
    }
}
