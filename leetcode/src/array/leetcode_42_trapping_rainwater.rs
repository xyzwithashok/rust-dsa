pub struct Solution {}

impl Solution {
    pub fn trap(self, height: Vec<i32>) -> i32 {
        let max_left: Vec<i32> = height.clone().into_iter()
            .scan(i32::MIN, |max_so_far, x| {
                *max_so_far = (*max_so_far).max(x);
                Some(*max_so_far)
            })
            .collect();

        let max_right: Vec<i32> = height.clone().into_iter().rev()
            .scan(i32::MIN, |max_so_far, x| {
                *max_so_far = (*max_so_far).max(x);
                Some(*max_so_far)
            }).collect::<Vec<i32>>().into_iter().rev().collect();

        let mut water = 0;

        for i in 0..height.len() {
            water += (max_left[i].min(max_right[i]) - height[i]).max(0);
        }

        water
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trap() {
        let s = Solution {};
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        // println!("{:?}", height.clone());
        let result = s.trap(height);
        assert_eq!(result, 6);
    }
}