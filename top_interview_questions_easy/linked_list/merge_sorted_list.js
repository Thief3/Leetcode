/******************************************************************************

Merge Two Sorted Lists

Merge two sorted linked lists and return it as a new sorted list. The new list
should be made by splicing together the nodes of the first two lists.

Example:

Input: 1->2->4, 1->3->4
Output: 1->1->2->3->4->4
******************************************************************************/

/**
 * Definition for singly-linked list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} l1
 * @param {ListNode} l2
 * @return {ListNode}
 */
var mergeTwoLists = function(l1, l2) {
    if(l1 == null && l2 == null){
        return null;
    }
    else if(l1 == null && l2 != null){
        return l2
    }
    else if(l1 != null && l2 == null){
        return l1;
    }
    
    var thead = [];
    while(true){
        console.log(thead);
        if(l1.val != null || l2.val != null){
            if(l2.val == null || l1.val !== null && l1.val < l2.val){
                thead.push(l1.val);
                if(l1.next != null){
                    l1.val = l1.next.val;
                    l1.next = l1.next.next;
                }
                else{
                    l1.val = null;
                }
            }
        
            else if (l1.val == null || l2.val !== null && l1.val >= l2.val){
                thead.push(l2.val);
                if(l2.next != null){
                    l2.val = l2.next.val;
                    l2.next = l2.next.next;
                }
                else{
                    l2.val = null;
                }
            }
        }
        else {
            break;
        }
    }
    
    return build_list(thead);
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
}
