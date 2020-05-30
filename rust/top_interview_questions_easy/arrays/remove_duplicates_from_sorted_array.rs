/*******************************************************************************
Remove Duplicates From Sorted Arrays

Given a sorted array nums, remove the duplicates in-place such that each element
appear only once and return the new length.

Do not allocate extra space for another array, you must do this by modifying the
input array in-place with O(1) extra memory.

Example 1:

Given nums = [1,1,2],
Your function should return length = 2, with the first two elements of nums
being 1 and 2 respectively.

It doesn't matter what you leave beyond the returned length.
********************************************************************************/

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if(nums.len() != 0){   
            let mut i:usize = 0;
            loop{
                if(i == (nums.len() - 1)){
                    break;
                }
                else if(nums[i] == nums[i + 1]){
                    nums.remove(i);
                }
                else{
                    i = i +1;
                }
            }
        
            return nums.len() as i32;
        }
        else {
            return 0;
        }
    }
}
