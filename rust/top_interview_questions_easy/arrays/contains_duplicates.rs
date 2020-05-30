/*******************************************************************************

Array Contains Duplicates.

Given an array of integers, find if the array contains any duplicates.

Your function should return true if any value appears at least twice in the
array, and it should return false if every element is distinct.

Example 1:

Input: [1,2,3,1]
Output: true
Example 2:

Input: [1,2,3,4]
Output: false

********************************************************************************/

/// Brute force method doesn't work with leetcode due to complexitiy.
impl Solution {
    pub fn contains_duplicate(mut nums: Vec<i32>) -> bool {
        if(nums.len() > 1){
            nums.sort();
            for i in 0..(nums.len() - 1) {
                if (nums[i] == nums[i + 1]){
                    return true;
                }   
            }
        }
        return false;
    }
}
