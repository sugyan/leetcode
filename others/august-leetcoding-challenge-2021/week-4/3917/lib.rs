pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let n = [num1, num2]
            .iter()
            .map(|s| {
                s.trim_end_matches('i')
                    .split('+')
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        format!(
            "{}+{}i",
            n[0][0] * n[1][0] - n[0][1] * n[1][1],
            n[0][0] * n[1][1] + n[0][1] * n[1][0]
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "0+2i",
            Solution::complex_number_multiply(String::from("1+1i"), String::from("1+1i"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "0+-2i",
            Solution::complex_number_multiply(String::from("1+-1i"), String::from("1+-1i"))
        );
    }
}
