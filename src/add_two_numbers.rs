// 2. Add Two Numbers
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

use crate::ListNode;
use crate::Solution;
use std::collections::LinkedList;

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
        //
        // Attached new solutions (only work with edition=2018) without using
        // LinkedList. But this version is slower (12ms) than the original one
        // (8ms) and it does not work with edition=2015.
        //
        // Personally, I would use LinkedList for better performance.

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

    pub fn add_two_numbers_no_linkedlist(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut result = Box::new(ListNode::new(0));
        let mut tail_ref = &mut result;

        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        while l1.is_some() && l2.is_some() {
            let ll1 = l1.unwrap();
            let ll2 = l2.unwrap();

            let sum = ll1.val + ll2.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();

            l1 = ll1.next;
            l2 = ll2.next;
        }

        while l1.is_some() {
            let ll1 = l1.unwrap();
            let sum = ll1.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
            l1 = ll1.next;
        }

        while l2.is_some() {
            let ll2 = l2.unwrap();
            let sum = ll2.val + carry;
            carry = sum / 10;
            let n = Box::new(ListNode::new(sum % 10));
            tail_ref.next = Some(n);
            tail_ref = tail_ref.next.as_mut().unwrap();
            l2 = ll2.next;
        }

        if carry != 0 {
            tail_ref.next = Some(Box::new(ListNode::new(carry)));
        }

        result.next
    }
}
