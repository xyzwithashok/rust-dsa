// https://www.geeksforgeeks.org/maximum-value-arri-arrj-j
// expression |arr[i] – arr[j]| + |i – j|
use std;
use std::cmp::{max, min};

pub fn max_value_of_expression(arr: Vec<i32>) -> i32 {
    let n = arr.len();

    let (mut max1, mut max2) = (i32::MIN, i32::MIN);
    let (mut min1, mut min2) = (i32::MAX, i32::MAX);

    for i in 0..n {
        let temp1: i32 = arr[i] + i as i32;
        let temp2: i32 = arr[i] - i as i32;
        max1 = max(temp1, temp2);
        min1 = min(min1, temp1);
        max2 = max(max2, temp2);
        min2 = min(min2, temp2);
    }
    max((max1 - min1), (max2 - min2))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_value() {
        let arr = vec![1, 15, 13, 8];
        // assert_eq!(max_value(arr), 90);
    }
}