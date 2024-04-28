struct Solution {}

/// https://leetcode.com/problems/maximum-of-absolute-value-expression/
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let arr1 = vec![1, 2, 3, 4];
        let arr2 = vec![-1, 4, 5, 6];
        assert_eq!(Solution::max_abs_val_expr(arr1, arr2), 13);
    }

    #[test]
    fn test_2() {
        let arr1 = vec![1, -2, -5, 0];
        let arr2 = vec![0, -2, -1, -7];
        assert_eq!(Solution::max_abs_val_expr(arr1, arr2), 20);
    }
}