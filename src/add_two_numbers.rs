// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use std::collections::LinkedList;
use ListNode;
use Solution;

fn list_to_linkedlist(l: Option<Box<ListNode>>) -> LinkedList<i32> {
    let mut result = LinkedList::new();
    let mut curr = l;

    while curr != None {
        let inner = curr.unwrap();
        result.push_back(inner.val);
        curr = inner.next;
    }

    result
}

fn linkedlist_to_list(mut ll: LinkedList<i32>) -> Option<Box<ListNode>> {
    let mut tail = None;

    while ll.front().is_some() {
        let v = *ll.front().unwrap();
        let node = ListNode { val: v, next: tail };
        tail = Some(Box::new(node));
        ll.pop_front();
    }

    tail
}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // We have to construct the result ListNode list from its back
        // resulting in inevitable intermediate representation of a list
        //     -- here I use std::collections::LinkedList

        let mut ll1 = list_to_linkedlist(l1);
        let mut ll2 = list_to_linkedlist(l2);

        let mut carry = 0;

        let mut result_ll = LinkedList::new();

        while ll1.front().is_some() && ll2.front().is_some() {
            let sum = ll1.front().unwrap() + ll2.front().unwrap() + carry;
            result_ll.push_front(sum % 10);
            carry = sum / 10;
            ll1.pop_front();
            ll2.pop_front();
        }

        while ll1.front().is_some() {
            let sum = ll1.front().unwrap() + carry;
            result_ll.push_front(sum % 10);
            carry = sum / 10;
            ll1.pop_front();
        }

        while ll2.front().is_some() {
            let sum = ll2.front().unwrap() + carry;
            result_ll.push_front(sum % 10);
            carry = sum / 10;
            ll2.pop_front();
        }

        if carry != 0 {
            result_ll.push_front(carry);
        }

        linkedlist_to_list(result_ll)
    }
}
