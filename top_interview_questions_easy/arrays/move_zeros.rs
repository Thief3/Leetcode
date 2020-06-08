/*******************************************************************************

Move Zeros

Given an array nums, write a function to move all 0's to the end of it while
maintaining the relative order of the non-zero elements.

Example:

Input: [0,1,0,3,12]
Output: [1,3,12,0,0]
Note:

You must do this in-place without making a copy of the array.
Minimize the total number of operations.

 *******************************************************************************/

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut count = 0;
        let mut still_in_loop:bool = true;
        let mut i = 0;
        while still_in_loop{
            if nums[i] == 0 {
                count = count + 1;
            }
            else{
                i = i + 1;
            }
            
            if i + count < nums.len(){
                nums[i] = nums[i + count];
            }
            else {
                still_in_loop = false;
            }
        }
        
        // This bit is fine.
        for i in (nums.len() - count)..nums.len() {
            nums[i] = 0;
        }
    }
}
