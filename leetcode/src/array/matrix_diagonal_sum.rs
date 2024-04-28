pub struct Solution {}

impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let n = mat.len();
        (0..n).into_iter().fold(
            0, |sum, x| {
                let mut s = sum + mat[x][x];
                if x != n - 1 - x {
                    s += mat[x][n - 1 - x]
                }
                s
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mat = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        assert_eq!(Solution::diagonal_sum(mat), 25);
    }
}