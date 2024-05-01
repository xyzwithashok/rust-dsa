use crate::linked_list::linked_list::ListNode;

struct Solution;

// TODO: Implement the solution
impl Solution {
    pub fn spiral_matrix(m: i32, n: i32, head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![-1; n as usize]; m as usize];
        let mut head = head;
        let (mut left, mut right, mut top, mut bottom) = (0, n - 1, 0, n - 1);

        let mut x = 0;  // row
        let mut y = 0; // column
        let mut direction = 0;  // 0: right, 1: down, 2: left, 3: up
        let mut step = 0;  // steps in current direction

        while head.is_some() {
            let node = head.unwrap();
            head = node.next;
            let val = node.val;
            ans[x as usize][y as usize] = val;
            step += 1;
            match direction {
                0 => {
                    if step == right {
                        step = 0;
                        direction = 1;
                        top += 1;
                    }
                }
                1 => {
                    if step == bottom {
                        step = 0;
                        direction = 2;
                        right -= 1;
                    }
                }
                2 => {
                    if step == left {
                        step = 0;
                        direction = 3;
                        bottom -= 1;
                    }
                }
                3 => {
                    if step == top {
                        step = 0;
                        direction = 0;
                        left += 1;
                    }
                }
                _ => {}
            }
            match direction {
                0 => y += 1,
                1 => x += 1,
                2 => y -= 1,
                3 => x -= 1,
                _ => {}
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_matrix() {
        let m = 3;
        let n = 3;
        let head = ListNode::from_vec(vec![3, 0, 2, 6, 8, 1, 7, 9, 4, 2, 5, 5, 0]);
        let result = Solution::spiral_matrix(m, n, head);
        let expected = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(result, expected);
    }
}