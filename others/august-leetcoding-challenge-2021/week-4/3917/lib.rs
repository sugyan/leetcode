pub struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let n1 = num1
            .trim_end_matches('i')
            .split('+')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        let n2 = num2
            .trim_end_matches('i')
            .split('+')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<_>>();
        format!(
            "{}+{}i",
            n1[0] * n2[0] - n1[1] * n2[1],
            n1[0] * n2[1] + n1[1] * n2[0]
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
