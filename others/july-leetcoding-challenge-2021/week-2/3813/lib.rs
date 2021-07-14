pub struct Solution;

impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let count = str.as_bytes().iter().fold([0; 26], |mut acc, &x| {
            acc[(x - b'a') as usize] += 1;
            acc
        });
        let mut answer = String::new();
        for &u in order.as_bytes() {
            for _ in 0..count[(u - b'a') as usize] {
                answer.push(u as char);
            }
        }
        for u in b'a'..=b'z' {
            if !order.contains(u as char) {
                for _ in 0..count[(u - b'a') as usize] {
                    answer.push(u as char);
                }
            }
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "cbad",
            Solution::custom_sort_string(String::from("cba"), String::from("abcd"))
        );
    }
}
