pub struct Solution;

impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let v = num.bytes().map(|u| (u - b'0') as i64).collect::<Vec<_>>();
        let mut answer = Vec::new();
        Self::helper("", &v, 0, 0, target, &mut answer);
        answer
    }
    fn helper(
        prev: &str,
        remain: &[i64],
        val: i64,
        last: i64,
        target: i32,
        answer: &mut Vec<String>,
    ) {
        if remain.is_empty() {
            if val == target as i64 {
                answer.push(prev[1..].to_string());
            }
            return;
        }
        let mut n = 0;
        for (i, &r) in remain.iter().enumerate() {
            n = n * 10 + r;
            Self::helper(
                &(prev.to_string() + "+" + &n.to_string()),
                &remain[i + 1..],
                val + n,
                n,
                target,
                answer,
            );
            if !prev.is_empty() {
                Self::helper(
                    &(prev.to_string() + "-" + &n.to_string()),
                    &remain[i + 1..],
                    val - n,
                    -n,
                    target,
                    answer,
                );
                Self::helper(
                    &(prev.to_string() + "*" + &n.to_string()),
                    &remain[i + 1..],
                    val - last + last * n,
                    last * n,
                    target,
                    answer,
                );
            }
            if remain[0] == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut ret = Solution::add_operators(String::from("123"), 6);
        ret.sort();
        assert_eq!(vec!["1*2*3", "1+2+3"], ret);
    }

    #[test]
    fn example_2() {
        let mut ret = Solution::add_operators(String::from("232"), 8);
        ret.sort();
        assert_eq!(vec!["2*3+2", "2+3*2"], ret);
    }

    #[test]
    fn example_3() {
        let mut ret = Solution::add_operators(String::from("105"), 5);
        ret.sort();
        assert_eq!(vec!["1*0+5", "10-5"], ret);
    }

    #[test]
    fn example_4() {
        let mut ret = Solution::add_operators(String::from("00"), 0);
        ret.sort();
        assert_eq!(vec!["0*0", "0+0", "0-0"], ret);
    }

    #[test]
    fn example_5() {
        assert_eq!(
            Vec::<String>::new(),
            Solution::add_operators(String::from("3456237490"), 9191)
        );
    }
}
