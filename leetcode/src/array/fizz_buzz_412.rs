// https://leetcode.com/problems/fizz-buzz/


struct Solution {}

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        (1..n + 1)
            .into_iter()
            .map(|x| {
                if x == 0 {
                    "0".to_string()
                } else if x % 3 == 0 && x % 5 == 0 {
                    "FizzBuzz".to_string()
                } else if x % 3 == 0 {
                    "Fizz".to_string()
                } else if x % 5 == 0 {
                    "Buzz".to_string()
                } else {
                    x.to_string()
                }
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::fizz_buzz(3), vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::fizz_buzz(5), vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::fizz_buzz(15), vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz",
            "11", "Fizz", "13", "14", "FizzBuzz",
        ]);
    }
}