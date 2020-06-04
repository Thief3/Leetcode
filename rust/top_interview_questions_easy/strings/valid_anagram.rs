/*******************************************************************************

Valid Anagram

Given two strings s and t, write a function to determine if t is an anagram of s.

Example 1:

Input: s = "anagram", t = "nagaram"
Output: true
Example 2:

Input: s = "rat", t = "car"
Output: false
Note:
You may assume the string contains only lowercase alphabets.

Follow up:
What if the inputs contain unicode characters? How would you adapt your solution
to such case?

********************************************************************************/

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_sorted = s.chars().collect::<Vec<char>>();
        let mut t_sorted = t.chars().collect::<Vec<char>>();
        s_sorted.sort();
        t_sorted.sort();
        
        let length = s.len();
        if(s.len() != t.len()){
            return false;
        }
        
        for i in 0..length{
            if(s_sorted[i] == t_sorted[i]){
                continue;
            }
            else {
                return false;
            }
        }
        
        return true;
    }
}
