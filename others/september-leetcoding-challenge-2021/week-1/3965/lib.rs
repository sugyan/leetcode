pub struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let (mut max, mut answer) = (0, '\0');
        for (i, c) in keys_pressed.chars().enumerate() {
            let duration = release_times[i] - if i > 0 { release_times[i - 1] } else { 0 };
            if duration >= max {
                answer = if duration == max { answer.max(c) } else { c };
                max = duration;
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
            'c',
            Solution::slowest_key(vec![9, 29, 49, 50], String::from("cbcd"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            'a',
            Solution::slowest_key(vec![12, 23, 36, 46, 62], String::from("spuda"))
        );
    }
}
