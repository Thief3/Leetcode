/*******************************************************************************

Two Sum

Given an array of integers, return indices of the two numbers such that they add
up to a specific target.

You may assume that each input would have exactly one solution, and you may not
use the same element twice.

Example:

Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].

 *******************************************************************************/

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut v:Vec<i32> = Vec::new();
        'outer: for i in 0..nums.len(){
            for j in 0..nums.len(){
                if ((nums[i] + nums[j]) == target) && i != j{
                    v.push(i as i32);
                    v.push(j as i32);
                    break 'outer;
                }
            }
        }
        return v;
    }
}
