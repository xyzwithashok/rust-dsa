// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }

    pub(crate) fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;
        for i in v {
            let node = Some(Box::new(ListNode::new(i)));
            *tail = node;
            tail = &mut tail.as_mut().unwrap().next;
        }
        head
    }

    pub(crate) fn to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = vec![];
        let mut head = head;
        while let Some(node) = head {
            v.push(node.val);
            head = node.next;
        }
        v
    }

    pub(crate) fn to_string(head: Option<Box<ListNode>>) -> String {
        let mut s = String::new();
        let mut head = head;
        while let Some(node) = head {
            s.push_str(&node.val.to_string());
            s.push_str(" -> ");
            head = node.next;
        }
        s.push_str("None");
        s
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

// impl Iterator for ListNode {
//     type Item = &'a ListNode;
//
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(node) = self.next.as_ref() {
//             Some(node)
//         } else {
//             None
//         }
//     }
// }


struct LinkedList {
    head: Option<Box<ListNode>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList {
            head: None,
        }
    }

    pub(crate) fn from_vec(v: Vec<i32>) -> Self {
        LinkedList {
            head: ListNode::from_vec(v),
        }
    }

    pub(crate) fn to_vec(&self) -> Vec<i32> {
        ListNode::to_vec(self.head.clone())
    }

    pub(crate) fn to_string(&self) -> String {
        ListNode::to_string(self.head.clone())
    }

    pub(crate) fn push(&mut self, val: i32) {
        let mut node = Some(Box::new(ListNode::new(val)));
        let mut head = self.head.take();
        node.as_mut().unwrap().next = head;
        self.head = node;
    }

    pub(crate) fn pop(&mut self) -> Option<i32> {
        let mut head = self.head.take();
        if let Some(mut node) = head {
            self.head = node.next.take();
            Some(node.val)
        } else {
            None
        }
    }

    pub(crate) fn peek(&self) -> Option<i32> {
        self.head.as_ref().map(|node| node.val)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub(crate) fn get_head(&self) -> Option<Box<ListNode>> {
        self.head.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        assert_eq!(list.is_empty(), true);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.to_vec(), vec![3, 2, 1]);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.is_empty(), true);
    }
}
