/******************************************************************************

Count and Say

The count-and-say sequence is the sequence of integers with the first five terms
as following:

1.     1
2.     11
3.     21
4.     1211
5.     111221

1 is read off as "one 1" or 11.
11 is read off as "two 1s" or 21.
21 is read off as "one 2, then one 1" or 1211.

Given an integer n where 1 ≤ n ≤ 30, generate the nth term of the count-and-say
sequence. You can do so recursively, in other words from the previous member read
off the digits, counting the number of digits in groups of the same digit.

Note: Each term of the sequence of integers will be represented as a string.

 

Example 1:

Input: 1
Output: "1"
Explanation: This is the base case.

Example 2:

Input: 4
Output: "1211"
Explanation: For n = 3 the term was "21" in which we have two groups "2" and "1",
"2" can be read as "12" which means frequency = 1 and value = 2, the same way "1"
is read as "11", so the answer is the concatenation of "12" and "11" which is
"1211".
 *******************************************************************************/

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut current_count: u32 = 0;
        let mut current_num: u32 = 0;
        
        let mut result: String = String::from("1");
        let mut count = 1;
        
        while count < n {
            let old_result: Vec<char> = result.chars().collect();
            result = String::from("");
            current_count = 0;
            current_num = (match old_result[0].to_digit(10){Some(n) => n, None => 0});
            for i in 0..old_result.len() {
                if cur == current_num {
                    current_count = current_count + 1;
                }
                else {
                    result.push(match std::char::from_digit(current_count, 10){
                        Some(n) => n,
                        None => '0'});
                    result.push(match std::char::from_digit(current_num, 10){
                        Some(n) => n,
                        None => '0'});
                
                    current_count = 1;
                    current_num = cur;
                }
            }
            result.push(match std::char::from_digit(current_count, 10){
                    Some(n) => n,
                    None => '0'});
            result.push(match std::char::from_digit(current_num, 10){
                    Some(n) => n,
                    None => '0'});
            count = count + 1;
        }
        
        return result;
    }
}
