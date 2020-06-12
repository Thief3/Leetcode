/******************************************************************************

Merge Sorted Arrays

Given two sorted integer arrays nums1 and nums2, merge nums2 into nums1 as one
sorted array.

Note:

The number of elements initialized in nums1 and nums2 are m and n respectively.
You may assume that nums1 has enough space (size that is greater or equal to
m + n) to hold additional elements from nums2.

Example:

Input:
nums1 = [1,2,3,0,0,0], m = 3
nums2 = [2,5,6],       n = 3

Output: [1,2,2,3,5,6]
******************************************************************************/

/**
 * @param {number[]} nums1
 * @param {number} m
 * @param {number[]} nums2
 * @param {number} n
 * @return {void} Do not return anything, modify nums1 in-place instead.
 */
var merge = function(nums1, m, nums2, n) {
    var count = 0;
    if(m == 0){
        for(var i = 0; i < n; i++){
            nums1[i] = nums2[i];
        }
    }
    else{
        for(var i = nums1.length - 1; i > m - 1; i--){
            nums1[i] = "n";
        }
        // Edge case where second array has element before the first
        if(nums1[0] >= nums2[count]){
            console.log(nums1);
            for(var j = nums1.length - 1; j > 0; j--){
                nums1[j] = nums1[j - 1];
            }
            nums1[0] = nums2[count];
            count = count + 1;
        }
        
        for(var i = 0; i < nums1.length; i++){
            console.log(i, " : ", nums1);
            if(nums1[i] <= nums2[count] && (nums1[i + 1] >= nums2[count] || nums1[i + 1] == "n")){
                for(var j = nums1.length - 1; j > i; j--){
                    nums1[j] = nums1[j - 1];
                }
                nums1[i + 1] = nums2[count];
                count = count + 1;
            }
        }
    }
};
