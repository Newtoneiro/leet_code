// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry_over = 0;
        let mut n1 = l1;
        let mut n2 = l2;

        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut current = dummy_head.as_mut(); // Mutable reference to the dummy head

        loop {
            match (&n1, &n2) {
                (Some(val1), Some(val2)) => {
                    let sum: i32 = val1.val + val2.val + carry_over;
                    if sum > 9 {
                        carry_over = 1;
                    } else {
                        carry_over = 0;
                    }
                    if let Some(curr) = current {
                        curr.next = Some(Box::new(ListNode::new(sum % 10)));
                        current = curr.next.as_mut();
                    }
                    n1 = n1.unwrap().next;
                    n2 = n2.unwrap().next;
                },
                (Some(val1), None) => {
                    let sum: i32 = val1.val + carry_over;
                    if sum > 9 {
                        carry_over = 1;
                    } else {
                        carry_over = 0;
                    }
                    if let Some(curr) = current {
                        curr.next = Some(Box::new(ListNode::new(sum % 10)));
                        current = curr.next.as_mut();
                    }
                    n1 = n1.unwrap().next;
                },
                (None, Some(val2)) => {
                    let sum: i32 = val2.val + carry_over;
                    if sum > 9 {
                        carry_over = 1;
                    } else {
                        carry_over = 0;
                    }
                    if let Some(curr) = current {
                        curr.next = Some(Box::new(ListNode::new(sum % 10)));
                        current = curr.next.as_mut();
                    }
                    n2 = n2.unwrap().next;
                },
                (None, None) => {
                    if carry_over == 1 {
                        if let Some(curr) = current {
                            curr.next = Some(Box::new(ListNode::new(1)));
                            current = curr.next.as_mut();
                        }
                    }
                    break;
                }
            };
        }

        dummy_head.unwrap().next
    }
}