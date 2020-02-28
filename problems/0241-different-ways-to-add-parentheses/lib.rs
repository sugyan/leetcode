pub struct Solution {}

impl Solution {
    pub fn diff_ways_to_compute(input: String) -> Vec<i32> {
        let mut answer = Vec::new();
        for (i, c) in (0..).zip(input.chars()) {
            if c == '+' || c == '-' || c == '*' {
                let lhs = Solution::diff_ways_to_compute((&input[0..i]).to_string());
                let rhs = Solution::diff_ways_to_compute((&input[i + 1..]).to_string());
                for l in lhs.iter() {
                    for r in rhs.iter() {
                        match c {
                            '+' => answer.push(l + r),
                            '-' => answer.push(l - r),
                            '*' => answer.push(l * r),
                            _ => {}
                        }
                    }
                }
            }
        }
        if answer.is_empty() {
            answer.push(input.parse::<i32>().unwrap());
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::diff_ways_to_compute("2-1-1".to_string());
        ret.sort();
        assert_eq!(vec![0, 2], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::diff_ways_to_compute("2*3-4*5".to_string());
        ret.sort();
        assert_eq!(vec![-34, -14, -10, -10, 10], ret);
    }
}
