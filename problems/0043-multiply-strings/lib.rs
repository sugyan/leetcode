pub struct Solution {}

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut answer = vec![0; num1.len() + num2.len()];
        let v1: Vec<u8> = num1.bytes().rev().map(|b| b as u8 - '0' as u8).collect();
        let v2: Vec<u8> = num2.bytes().rev().map(|b| b as u8 - '0' as u8).collect();
        for (i, d1) in (0..).zip(v1.iter()) {
            let mut carry = 0;
            for (j, d2) in (0..).zip(v2.iter()) {
                let m = d1 * d2 + carry;
                carry = m / 10;
                answer[i + j] += m % 10;
                if answer[i + j] > 9 {
                    carry += 1;
                    answer[i + j] -= 10;
                }
            }
            if carry > 0 {
                answer[i + v2.len()] += carry;
            }
        }
        answer.reverse();
        while answer.len() > 1 && answer[0] == 0 {
            answer.remove(0);
        }
        return answer.iter().map(|u| ('0' as u8 + u) as char).collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "6".to_string(),
            Solution::multiply("2".to_string(), "3".to_string())
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "56088".to_string(),
            Solution::multiply("123".to_string(), "456".to_string())
        );
    }
}
