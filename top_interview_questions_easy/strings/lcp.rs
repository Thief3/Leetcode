/*******************************************************************************

Longest Common Prefix

Write a function to find the longest common prefix string amongst an array of
strings.

If there is no common prefix, return an empty string "".

Example 1:

Input: ["flower","flow","flight"]
Output: "fl"

Example 2:

Input: ["dog","racecar","car"]
Output: ""
Explanation: There is no common prefix among the input strings.

Note:

All given inputs are in lowercase letters a-z.

 *******************************************************************************/

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut count = 0;
        let mut common: String = "".to_string();
        let mut current_letter: char = ' ';
        
        if strs.len() == 0 {
            return "".to_string();
        }
        
        'outer: loop {
            //println!("Looping");
            if count < strs[0].len(){
                current_letter = (strs[0].chars().collect::<Vec<char>>())[count];
                //println!("Current_letter: {}", current_letter);
            }
            for s in &strs{
                let c: Vec<char> = s.chars().collect();
                if count < c.len() && c[count] == current_letter {
                    continue;
                }
                else{
                    //println!("No match. Terminate.");
                    break 'outer;
                }
            }
            common.push(current_letter);
            //println!("Added current_letter to the common string: {:?}", common);
            count = count + 1;
        }
        
        return common;
    }
}
