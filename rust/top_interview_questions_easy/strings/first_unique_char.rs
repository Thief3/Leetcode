/*******************************************************************************

First Unique Character in String

Given a string, find the first non-repeating character in it and return it's
index. If it doesn't exist, return -1.

Examples:

s = "leetcode"
return 0.

s = "loveleetcode",
return 2.
 

Note: You may assume the string contain only lowercase English letters.
********************************************************************************/

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {       
        let mut first = Vec::new();
        let mut second = Vec::new();
        
        for i in s.chars(){
            if(first.contains(&i)){
                second.push(i);
                first.push(i); //<- Defo not effecient.
            }
            else{
                first.push(i);
            }
        }
        
        for i in 0..first.len(){
            if(!second.contains(&first[i])){
                //println!("{0}, {1}", i, &first[i]);
                return i as i32;
            }
        }
        
        return -1;
    }
}
