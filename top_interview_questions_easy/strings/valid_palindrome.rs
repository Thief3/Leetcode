/*******************************************************************************

Valid Palindrome

Given a string, determine if it is a palindrome, considering only alphanumeric
characters and ignoring cases.

Note:For the purpose of this problem, we define empty string as valid palindrome.

Example 1:

Input: "A man, a plan, a canal: Panama"
Output: true
Example 2:

Input: "race a car"
Output: false
 *******************************************************************************/

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s1: Vec<char> = s.to_lowercase().chars()
            .filter(|c| !c.is_whitespace() && c.is_alphanumeric())
            .collect();
        
        let mut s2: Vec<char> = s1.clone();
        s2.reverse();
        
        for i in 0..s1.len(){
            if(s1[i] != s2[i]){
                return false;
            }
        }
        
        return true;
    }
}
