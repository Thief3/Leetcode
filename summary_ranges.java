/** Summary Ranges
 * 
 * Given a sorted integer array without duplicates, return the summary of its ranges.
 *
 * Example 1:
 * Input:  [0,1,2,4,5,7]
 * Output: ["0->2","4->5","7"]
 * Explanation: 0,1,2 form a continuous range; 4,5 form a continuous range.
 * 
 * Example 2:
 * Input:  [0,2,3,4,6,8,9]
 * Output: ["0","2->4","6","8->9"]
 * Explanation: 2,3,4 form a continuous range; 8,9 form a continuous range.
 */

class Solution {
    public List<String> summaryRanges(int[] nums) {
        if(nums.length == 0){
            return new ArrayList<String>();
        }
        List<String> output = new ArrayList<>();
        int count = 0;
        //int current = nums[0];
        int small = nums[0];
        int current_loc = 0;
        while(current_loc + 1 < nums.length){
            if(nums[current_loc] + 1 == nums[current_loc + 1]){
                current_loc = current_loc + 1;
                count = count + 1;
            }
            else{
                System.out.println(count);
                if(count == 0){
                    output.add(String.valueOf(nums[current_loc]));
                }
                else{
                    output.add(String.valueOf(small)+"->"+String.valueOf(nums[current_loc]));
                }
                count = 0;
                current_loc = current_loc + 1;
                small = nums[current_loc];
            }
            
        }
        // Some duplication here but yolo
        if(count == 0){
            output.add(String.valueOf(nums[nums.length - 1]));
        }
        else{
            output.add(String.valueOf(small)+"->"+String.valueOf(nums[nums.length - 1]));
        }
        return output;
    }
}
