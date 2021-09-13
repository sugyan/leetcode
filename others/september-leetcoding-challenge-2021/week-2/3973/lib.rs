pub struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let counts = |s: &str| {
            s.bytes().fold([0; 26], |mut acc, u| {
                acc[(u - b'a') as usize] += 1;
                acc
            })
        };
        let balloon = counts("balloon");
        counts(&text)
            .iter()
            .zip(balloon.iter())
            .filter_map(|(c, b)| if *b > 0 { Some(c / b) } else { None })
            .min()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            1,
            Solution::max_number_of_balloons(String::from("nlaebolko"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            2,
            Solution::max_number_of_balloons(String::from("loonbalxballpoon"))
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            0,
            Solution::max_number_of_balloons(String::from("leetcode"))
        );
    }
}
