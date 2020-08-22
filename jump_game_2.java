/** Jump Game II
 * Given an array of non-negative integers, you are initially positioned at the first index of the array.
 * Each element in the array represents your maximum jump length at that position.
 * Your goal is to reach the last index in the minimum number of jumps.
 * 
 * Example:
 * Input: [2,3,1,1,4]
 * Output: 2
 * Explanation: The minimum number of jumps to reach the last index is 2.
 *  Jump 1 step from index 0 to 1, then 3 steps to the last index.
 *
 * Note:
 * You can assume that you can always reach the last index.
 *
 * https://leetcode.com/problems/jump-game-ii/
 */

class Solution {
    public int jump(int[] nums) {
        if(nums.length == 1){
            return 0;
        }
        return recursion_jumping(nums, 0, 0);
    }
    public int recursion_jumping(int[] nums, int current_loc, int turns){
        if(current_loc >= nums.length - 1){
            return turns;
        }
        else if (nums[current_loc] >= 1){
            int low = 100000000;
            for(int i = 1; i <= nums[current_loc] && current_loc + i < nums.length; i++){
                int val = recursion_jumping(nums, current_loc + i, turns + 1);

                if(val < low){
                    low = val;
                }
            }
            return low;
        }
        else{
            return 20000000;
        }
    }
}
