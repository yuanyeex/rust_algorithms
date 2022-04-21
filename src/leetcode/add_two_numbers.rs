// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = l1;
        let mut list2 = l2;
        let mut carry = 0;
        let mut header = Box::new(ListNode::new(0));
        let mut tail = &mut header;
        while list1.is_some() || list2.is_some() || carry != 0 {
            let sum = if let Some(v1) = list1 {
                list1 = v1.next;
                v1.val
            } else {
                0
            } + if let Some(v2) = list2 {
                list2 = v2.next;
                v2.val
            } else {
                0
            } + carry;

            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }

        header.next
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_numbers() {
        let list1 = vec_to_node_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let list2 = vec_to_node_list(vec![9, 9, 9, 9]);
        let ret = add_two_numbers(list1, list2);

        let ret_vec = node_list_to_vec(ret);
        assert_eq!(ret_vec, vec![8, 9, 9, 9, 0, 0, 0, 1])
    }

    fn vec_to_node_list(nodes: Vec<i32>) -> Option<Box<ListNode>> {
        let mut header = Box::new(ListNode::new(0));
        // &mut header: iter borrows a writable reference from header
        // mut iter: iter can also bind to other writable reference
        let mut iter = &mut header;
        for ele in nodes {
            iter.next = Some(Box::new(ListNode::new(ele)));
            // as_mut, makes mut Option<T> to Option<&mut T>
            iter = iter.next.as_mut().unwrap();
        }

        header.next
    }

    fn node_list_to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut list2 = list;
        let mut vec = Vec::new();
        while list2.is_some() {
            if let Some(v) = list2 {
                list2 = v.next;
                let value = v.val;
                vec.push(value);
            }
        }
        vec
    }
}
