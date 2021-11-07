pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.bytes().rev().map(|u| u - b'0').collect::<Vec<_>>();
        let num2 = num2.bytes().rev().map(|u| u - b'0').collect::<Vec<_>>();
        let mut v = vec![0; num1.len() + num2.len()];
        for (i, &d1) in num1.iter().enumerate() {
            let mut carry = 0;
            for (j, &d2) in num2.iter().enumerate() {
                v[i + j] += d1 * d2 + carry;
                carry = v[i + j] / 10;
                v[i + j] %= 10;
            }
            v[i + num2.len()] += carry;
        }
        while v.last() == Some(&0) && v.len() > 1 {
            v.pop();
        }
        v.iter().rev().map(|&u| (b'0' + u) as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "6",
            Solution::multiply(String::from("2"), String::from("3"))
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "56088",
            Solution::multiply(String::from("123"), String::from("456"))
        );
    }
}
