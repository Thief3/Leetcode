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


 ********************************************************************************/

use std::collections::VecDeque;
use std::convert::TryFrom;

impl Solution {
    pub fn my_atoi(str: String) -> i32 { 
        let mut is_neg: bool = false;
        let mut num: i32 = 0;
        let mut str_vec: VecDeque<char> = str.clone().chars().collect();//.filter(|c| !c.is_whitespace()).collect();
        
        loop{
            if (str_vec.len() == 0){
                return 0;
            }
            else if(str_vec[0] != ' '){
                break;
            }
            else{
                //println!("Happen?");
                str_vec.pop_front();
            }
            
        }
        
        //println!("{:?}", str_vec);
        
        if (str_vec[0] == '-'){
            is_neg = true;
            str_vec.pop_front();
        }
        else if (str_vec[0] == '+'){
            str_vec.pop_front();
        }
        
        for i in (0..str_vec.len()).rev() {
            if !str_vec[i].is_numeric() {
                str_vec.pop_back();
            }
            else{
                break;
            }
        }
        
        //println!("{:?}", str_vec);
        
        for i in 0..str_vec.len() {
            if(!str_vec[i].is_numeric() && num > 0){
                num = num/(10_i32.pow((str_vec.len() -i) as u32));
                break;
            }
            else if(!str_vec[i].is_numeric()){
                return 0;
            }
            
            let dig = match (str_vec[i].to_digit(10)){
                // Technically I shouldn't really cast like this, I could use a match for the chars to ints but long.
                Some(number) => number as i32,
                None => return 0 as i32 //<- This should never happen.
            };
            println!("Digit: {}", dig);
            println!("Add: {}", dig*(10_i32.pow((str_vec.len() - 1 - i) as u32)));
            match 10_i32.checked_pow((str_vec.len() - 1 - i) as u32) {
                Some(v) => 
                    match num.checked_add(dig*(10_i32.pow((str_vec.len() - 1 - i) as u32))){
                        Some(v) => {
                            //println!("{}", v);
                            num = v},
                        None => {
                            //println!("overflow?");
                            //println!("{}", std::i32::MIN);
                            if(is_neg){
                                return std::i32::MIN;
                            }
                            else{
                                return std::i32::MAX;
                            }
                        }
                    },
                None => {
                    println!("overflow?");
                    //println!("{}", std::i32::MIN);
                    if(is_neg){
                        return std::i32::MIN;
                    }
                    else{
                        return std::i32::MAX;
                    }
                }
            };
            
        }
 
        if(is_neg){
            num = num * (-1);
        }
        
        return num as i32;
    }
}
