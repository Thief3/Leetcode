/**
 * Given an array of linked-lists lists, each linked list is sorted in ascending order.
 * Merge all the linked-lists into one sort linked-list and return it.
 * Example 1:
 * Input: lists = [[1,4,5],[1,3,4],[2,6]]
 * Output: [1,1,2,3,4,4,5,6]
 * Explanation: The linked-lists are:
 * [
 *  1->4->5,
 *  1->3->4,
 *  2->6
 * ]
 * merging them into one sorted list:
 * 1->1->2->3->4->4->5->6
 *
 * Example 2:
 * Input: lists = []
 * Output: []
 * 
 * Example 3:
 * Input: lists = [[]]
 * Output: []
 *
 * Constraints:
 *     k == lists.length
 *     0 <= k <= 10^4
 *     0 <= lists[i].length <= 500
 *     -10^4 <= lists[i][j] <= 10^4
 *     lists[i] is sorted in ascending order.
 *     The sum of lists[i].length won't exceed 10^4.
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
    public ListNode mergeKLists(ListNode[] lists) {
        ListNode output = new ListNode();
        
        if(lists.length == 0){
            return null;
        }
        else if(lists.length > 0){
            output = lists[0];
            for(int i = 1; i < lists.length; i++){
                output = this.mergeTwoLists(output, lists[i]);
            }
        }
        
        return output;
    }
    
    public ListNode mergeTwoLists(ListNode l1, ListNode l2) {
        ListNode dummy = new ListNode();
        ListNode output = dummy;
        while(true){
            if(l1 == null && l2 == null){
                break;
            }
            dummy.next = new ListNode();
            dummy = dummy.next;
            if(l1 == null){
                dummy.val = l2.val;
                l2 = l2.next;
            }
            else if(l2 == null){
                dummy.val = l1.val;
                l1 = l1.next;
            }
            else if(l1.val > l2.val){
                dummy.val = l2.val;
                l2 = l2.next;
            }
            else if(l1.val <= l2.val){
                dummy.val = l1.val;
                l1 = l1.next;
            }
        }
        output = output.next;
        return output;
    }
}
