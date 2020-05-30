/*******************************************************************************
Single Number

Given a non-empty array of integers, every element appears twice except for one.
Find that single one.

Note:

Your algorithm should have a linear runtime complexity. Could you implement it
without using extra memory?

Example 1:

Input: [2,2,1]
Output: 1
Example 2:

Input: [4,1,2,1,2]
Output: 4

 ********************************************************************************/

/// Think this has linear complexity? The hashmap for loop is O(1) as it should
/// always have one element, and our first loop is a single pass. We do use
/// alot of memory in the worst case tho.

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut num_map = std::collections::HashMap::new();
        
        for i in 0..nums.len(){
            if(num_map.get(&nums[i]) != None){
                num_map.remove(&nums[i]);
            }
            else {
                num_map.insert(nums[i], 1);
            }
        }
        
        let mut ret:i32 = 0;
        for (k, v) in num_map.iter() {
            ret = *k;
        }
        
        return ret;
    }
}
