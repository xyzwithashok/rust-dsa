struct Solution {}

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix: Vec<Vec<i32>> = vec![vec![0; n as usize]; n as usize];

        let (mut left, mut right, mut top, mut bottom) = (0i32, n - 1, 0i32, n - 1);
        let mut num = 1;

        while num <= n * n {
            for i in left..=right {
                matrix[top as usize][i as usize] = num;
                num += 1;
            }
            top += 1;

            for i in top..=bottom {
                matrix[i as usize][right as usize] = num;
                num += 1;
            }
            right -= 1;

            for i in (left..=right).rev() {
                matrix[bottom as usize][i as usize] = num;
                num += 1;
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                matrix[i as usize][left as usize] = num;
                num += 1;
            }
            left += 1;
        }

        matrix
    }
}


#[cfg(test)]
mod tests {
    use crate::matrix::matrix::Matrix;
    use super::*;

    #[test]
    fn test_1() {
        let n = 3;
        let result = Solution::generate_matrix(n);
        let expected = vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_2() {
        let n = 1;
        let result = Solution::generate_matrix(n);
        let expected = vec![vec![1]];
        assert_eq!(result, expected);
    }
}