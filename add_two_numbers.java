/**
 * Add Two Numbers
 *
 * You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order and each of their nodes contain a single digit. Add the two numbers and return it as a linked list.
 *
 * You may assume the two numbers do not contain any leading zero, except the number 0 itself.

 * Example:

 * Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
 * Output: 7 -> 0 -> 8
 * Explanation: 342 + 465 = 807.
 *
 * https://leetcode.com/problems/add-two-numbers/
 */

/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */

/**
 * Definition for singly-linked list.
 * public class ListNode {
 *     int val;
 *     ListNode next;
 *     ListNode() {}
 *     ListNode(int val) { this.val = val; }
 *     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
 * }
 */
class Solution {
    public ListNode addTwoNumbers(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode();
        ListNode output = dummy;
        while(true){
            int val = 0;
            int extra = 0;
            
            if(l1 != null && l2 != null){
                val = l1.val + l2.val;
                if(val >= 10){
                    val = val - 10;
                    extra = extra + 1;
                }
                
                l1 = l1.next;
                l2 = l2.next;
                
            }
            else if(l1 != null && l2 == null){
                val = l1.val;
                l1 = l1.next;
            }
            else if(l1 == null && l2 != null){
                val = l2.val;
                l2 = l2.next;
            }
            
            output.val = output.val + val;
            
            if(output.val >= 10){
                output.val = output.val - 10;
                extra = extra + 1;
            }
            

            if(extra != 0 || l1 != null || l2 != null){
                output.next = new ListNode();
                output = output.next;
                if(extra != 0){
                    output.val = extra;
                }
            }
            if(l1 == null && l2 == null) {                
                break;
            }
        }
        return dummy;
    }
}

