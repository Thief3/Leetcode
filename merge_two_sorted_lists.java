/**
 * Merge Two Sorted Lists
 * Merge two sorted linked lists and return it as a new sorted list. The new list should be made by splicing together the nodes of the first two lists.
 *
 * Example:
 * Input: 1->2->4, 1->3->4
 * Output: 1->1->2->3->4->4
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

