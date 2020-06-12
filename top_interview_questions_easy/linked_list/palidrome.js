/******************************************************************************

Palindrome Linked List

Given a singly linked list, determine if it is a palindrome.

Example 1:

Input: 1->2
Output: false
Example 2:

Input: 1->2->2->1
Output: true
Follow up:
Could you do it in O(n) time and O(1) space?
******************************************************************************/

/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {boolean}
 */
var isPalindrome = function(head) {
    var values_in_order = [];
    var head_dummy = head;
    
    if(head == null){
        return true;
    }
    
    while(true){
        values_in_order.push(head.val);
        if(head_dummy.next !== null){
            head_dummy.val = head_dummy.next.val;
            head_dummy.next = head_dummy.next.next;
        }
        else{
            break;
        }
    }
    
    var values_in_order_rev = values_in_order.slice().reverse();
    
    for(var i = 0; i < values_in_order.length; i++){
        if(values_in_order[i] != values_in_order_rev[i]){
            return false;
        }
    }
    
    return true;
};
