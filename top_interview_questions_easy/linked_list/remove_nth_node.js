/******************************************************************************

Remove n-th node from end of list.

Given a linked list, remove the n-th node from the end of list and return its
head.

Example:

Given linked list: 1->2->3->4->5, and n = 2.

After removing the second node from the end, the linked list becomes 1->2->3->5.
Note:

Given n will always be valid.

Follow up:

Could you do this in one pass?
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
 * @param {number} n
 * @return {ListNode}
 */
var removeNthFromEnd = function(head, n) {
    var return_head = head;
    
        var count = 0;
        while(true){
            if(return_head.next !== null){
                return_head = return_head.next;
                count = count + 1;
            }
            else {
                break;
            }
        }
    
        return_head = head;
        return_head = tester(return_head, n, count);
        return return_head;
    
};

var tester = function(head, n, count) {
    if(count == n - 1){
        if(head.next != null){
            head.val = head.next.val;
            head.next = head.next.next;
        }
        else{
            head = null;
        }
        return head;
    }
    else {
        head.next = tester(head.next, n, count - 1);
        return head;
    }
}
