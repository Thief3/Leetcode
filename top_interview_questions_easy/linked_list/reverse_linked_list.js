/******************************************************************************

Reverse Linked List

Reverse a singly linked list.

Example:

Input: 1->2->3->4->5->NULL
Output: 5->4->3->2->1->NULL
Follow up:

A linked list can be reversed either iteratively or recursively. Could you
implement both?
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
 * @return {ListNode}
 */
var reverseList = function(head) {
    var values_in_order = [];
    var head_dummy = head;
    
    console.log(head);
    if(head == null){
        return null; //ListNode(null, null);
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
    
    var return_head = build_list(values_in_order.reverse());
    
    return return_head;
};

var build_list = function(arr) {
    if(arr.length == 0 ){
        return null;
    }
    else{
        return ListNode(arr[0], build_list(arr.slice(1)));
    }
};

// Had to implament this manually for whatever reason.
var ListNode = function(val, next) {
    var ln = new Object;
    ln.val = (val===undefined ? 0 : val)
    ln.next = (next===undefined ? null : next)
    return ln;
};
