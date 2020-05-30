/********************************************************************************
Best Time to Buy and Sell Stock II

Say you have an array prices for which the ith element is the price of a given
stock on day i.

Design an algorithm to find the maximum profit. You may complete as many
transactions as you like (i.e., buy one and sell one share of the stock multiple
times).

Note: You may not engage in multiple transactions at the same time (i.e., you
must sell the stock before you buy again).

Example 1:

Input: [7,1,5,3,6,4]
Output: 7

Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5),
profit = 5-1 = 4.

Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.

Constraints:

+ 1 <= prices.length <= 3 * 10 ^ 4
+ 0 <= prices[i] <= 10 ^ 4
********************************************************************************/

/// Used a moving averages valleys approach
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut current_profit = 0;
        
        for i in 1..prices.len() {
            if(prices[i] > prices[i-1]){
                current_profit += prices[i] - prices[i - 1];
            }
        }
        
        return current_profit;
    }
}
