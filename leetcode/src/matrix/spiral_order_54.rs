struct Solution {}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];

        let m = matrix.len();
        let n = matrix[0].len();

        let (mut left, mut right, mut top, mut bottom) = (0, (n - 1) as i32, 0, (m - 1) as i32);

        while right >= left && bottom >= top {
            for i in left..=right {
                ans.push(matrix[top as usize][i as usize]);
            }
            top += 1;

            for i in top..=bottom {
                ans.push(matrix[i as usize][right as usize]);
            }
            right -= 1;

            for i in (left..=right).rev() {
                ans.push(matrix[bottom as usize][i as usize]);
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                ans.push(matrix[i as usize][left as usize]);
            }
            left += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result = Solution::spiral_order(matrix);
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let matrix = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let result = Solution::spiral_order(matrix);
        let expected = vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7];
        assert_eq!(result, expected);
    }
}