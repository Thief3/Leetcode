/*******************************************************************************

strStr

Implement strStr().

Return the index of the first occurrence of needle in haystack, or -1 if needle
is not part of haystack.

Example 1:

Input: haystack = "hello", needle = "ll"
Output: 2
Example 2:

Input: haystack = "aaaaa", needle = "bba"
Output: -1
Clarification:

What should we return when needle is an empty string? This is a great question
to ask during an interview.

For the purpose of this problem, we will return 0 when needle is an empty string.
This is consistent to C's strstr() and Java's indexOf().

 *******************************************************************************/

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_vec: Vec<char> = haystack.chars().collect();
        let needle_vec: Vec<char> = needle.chars().collect();
        let mut needling: bool = false;
        
        
        if needle == "".to_string() {
            return 0;
        }
        
        if haystack.len() < needle.len(){
            return -1;
        }
        
        'outer: for i in 0..haystack.len(){
            if(haystack_vec[i] == needle_vec[0]){
                needling = true;
                'inner: for j in 0..needle.len(){
                    if(i + j < haystack.len()){
                        if(haystack_vec[i + j] == needle_vec[j]){
                            continue;
                        }
                        else{
                            needling = false;
                            break 'inner;
                        }
                    }
                    else{
                        needling = false;
                        break 'outer;
                    }
                }
                if needling { return i as i32; }
            }
            else{
                continue;
            }
        }
        
        return -1;
    }
}
