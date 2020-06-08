/*******************************************************************************
Plus One

Given a non-empty array of digits representing a non-negative integer, plus one
to the integer.

The digits are stored such that the most significant digit is at the head of the
list, and each element in the array contain a single digit.

You may assume the integer does not contain any leading zero, except the number
0 itself.

Example 1:

Input: [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Example 2:

Input: [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
********************************************************************************/

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        if(digits.len() != 0){
            let mut i = digits.len() - 1;
            while i >= 0 {
                digits[i] = digits[i] + 1;
                if (digits[i] >= 10 && i != 0){
                    digits[i-1] = digits[i-1] + digits[i] - 10;
                    digits[i] = 0;
                }
                else if (digits[i] >= 10 && i == 0){
                    digits.insert(0, digits[i] - 10); 
                    digits[1] = 0;
                    i = 1;
                }
                else {
                    break;
                }
                i = i - 1;
            }
        }
        else {
            digits.push(1);
        }
        
        return digits;
    }
        
}
