
// Definition for singly-linked list.
 class ListNode {
     val: number
     next: ListNode | null
     constructor(val?: number, next?: ListNode | null) {
         this.val = (val===undefined ? 0 : val)
         this.next = (next===undefined ? null : next)
     }
 }


class Solution {
    protected nodes: number[];
    constructor(head: ListNode | null) {
        let temp_node: ListNode | null = head;
        this.nodes = [];
        while (temp_node){
            this.nodes.push(temp_node.val)
            temp_node = temp_node.next;
        }
    }

    getRandom(): number {
        const size = this.nodes.length;
        const rand_idx = Math.floor(Math.random() * size);
        return this.nodes[rand_idx];
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * var obj = new Solution(head)
 * var param_1 = obj.getRandom()
 */
