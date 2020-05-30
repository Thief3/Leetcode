/*------------------------------------------------------------------------------
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
*/
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        println!("Prices: {:?}", prices);
        if(prices.len() > 1){
            let mut current_sell: usize = 0;
            let mut current_profit: i32 = 0;
        
            for buy in 0..prices.len() {
                for sell in buy..prices.len(){
                    //println!("All sell: {}", sell);
                    //println!("Profit: {}", prices[sell] - prices[buy]);
                    if(current_profit < prices[sell] - prices[buy]){
                        //println!("Current_sell: {}", sell);
                        //current_buy = buy;
                        current_sell = sell;
                        current_profit = prices[sell] - prices[buy];
                    }
                }
            }
            //println!("Sell at: {}", current_sell);
            //println!("New Prices: {:?}", prices[current_sell..].to_vec());
            if(prices.len() == 2){
                return current_profit;
            }
            else {
                return current_profit + Solution::max_profit(prices[current_sell..].to_vec());
            }
        }
        else {
            return 0;
        }
    }
}
