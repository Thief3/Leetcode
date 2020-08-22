/** Median Of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively.
 * Return the median of the two sorted arrays.
 * Follow up: The overall run time complexity should be O(log (m+n)).
 *
 * Example 1:
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 *
 * Example 2:
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 *
 * Example 3:
 * Input: nums1 = [0,0], nums2 = [0,0]
 * Output: 0.00000
 *
 * Example 4:
 * Input: nums1 = [], nums2 = [1]
 * Output: 1.00000
 *
 * Example 5:
 * Input: nums1 = [2], nums2 = []
 * Output: 2.00000
 *
 * Constraints:
 *     nums1,length == m
 *     nums2,length == n
 *     0 <= m <= 1000
 *     0 <= n <= 1000
 *     1 <= m + n <= 2000
 *
 * https://leetcode.com/problems/median-of-two-sorted-arrays/
 */

class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        int[] merged = new int[nums1.length + nums2.length];
        int count1 = 0;
        int count2 = 0;
        
        while(true){
            if(nums1.length == count1 && nums2.length == count2){
                break;
            }
            else if(nums1.length == count1){
                merged[count1 + count2] = nums2[count2];
                count2 = count2 + 1;
            }
            else if(nums2.length == count2){
                merged[count1 + count2] = nums1[count1];
                count1 = count1 + 1;
            }
            else if(nums1[count1] >= nums2[count2]){
                merged[count1 + count2] = nums2[count2];
                count2 = count2 + 1;
            }
            else if(nums1[count1] < nums2[count2]){
                merged[count1 + count2] = nums1[count1];
                count1 = count1 + 1;
            }
        }
        if(merged.length % 2 == 0){
            return ((float)merged[merged.length/2 - 1] + (float)merged[merged.length/2])/2;
        }
        else {
            return merged[(merged.length - 1)/2];
        }
    }
}


