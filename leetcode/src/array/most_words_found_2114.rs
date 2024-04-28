use std::cmp::max;

struct Solution {}

impl Solution {
    pub fn most_words_found(sentences: Vec<String>) -> i32 {
        sentences.into_iter().map(|sentence| {
            let words = sentence.split_whitespace().collect::<Vec<&str>>();
            words.len() as i32
        }).fold(0, max)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let sentences = vec![
            "alice and bob love leetcode".to_string(),
            "i think so too".to_string(),
            "this is great thanks very much".to_string(),
        ];
        assert_eq!(Solution::most_words_found(sentences), 6);
    }

    #[test]
    fn test_2() {
        let sentences = vec![
            "please wait".to_string(),
            "continue to fight".to_string(),
            "continue to win".to_string(),
        ];
        assert_eq!(Solution::most_words_found(sentences), 3);
    }
}