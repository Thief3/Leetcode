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
/** OLD
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
*/

// Still too slow, but faster still...
class Solution {
    //public static int[] count_list = new int[1];
    
    public static int jump(int[] nums) {
        if(nums.length == 1){
            return 0;
        }
        int[] count_list = new int[nums.length];
        for(int i = 0; i < nums.length; i++){
            count_list[i] = -1;
        }
        for(int i = nums.length - 1; i > 0; i--){
            count_list[i] = recursion_jumping(nums, i, 0, count_list);
            //System.out.print(i);System.out.print(" : ");System.out.println(count_list[i]);
        }
        return recursion_jumping(nums, 0, 0, count_list);
    }
    public static int recursion_jumping(int[] nums, int current_loc, int turns, int[] count_list){
        if(current_loc >= nums.length - 1){
            return turns;
        }
        
        else if (nums[current_loc] >= 1){
            int low = 100000;
            for(int i = 1; i <= nums[current_loc] && current_loc + i < nums.length; i++){
                int val = low;
                if(count_list[current_loc + i] == -1){
                    val = recursion_jumping(nums, current_loc + i, turns + 1, count_list);
                    count_list[current_loc + i] = val;
                }
                else if(count_list[current_loc + i] == 300000){
                    val = 300000;
                }
                else if(nums[current_loc] + current_loc < nums.length - 1 && nums[current_loc + i] == 300000){
                    return 300000;
                }
                else{
                    val = recursion_jumping(nums, current_loc + i, turns + 1, count_list);
                }

                if(val < low){
                    low = val;
                }
            }
            return low;
        }
        else{
            return 200000;
        }
    }
}
//[5,6,4,4,6,9,4,4,7,4,4,8,2,6,8,1,5,9,6,5,2,7,9,7,9,6,9,4,1,6,8,8,4,4,2,0,3,8,5]
