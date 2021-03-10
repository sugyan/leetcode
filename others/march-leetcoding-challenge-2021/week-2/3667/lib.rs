pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut answer = String::new();
        let chars = [vec!['I', 'V'], vec!['X', 'L'], vec!['C', 'D'], vec!['M']];
        for i in (0..4).rev() {
            let d = (num / 10_i32.pow(i as u32)) % 10;
            if d % 5 == 4 {
                answer.push(chars[i][0]);
                answer.push(if d == 4 { chars[i][1] } else { chars[i + 1][0] });
            } else {
                if d >= 5 {
                    answer.push(chars[i][1]);
                }
                (0..d % 5).for_each(|_| answer.push(chars[i][0]));
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
        assert_eq!("III", Solution::int_to_roman(3));
    }

    #[test]
    fn example_2() {
        assert_eq!("IV", Solution::int_to_roman(4));
    }

    #[test]
    fn example_3() {
        assert_eq!("IX", Solution::int_to_roman(9));
    }

    #[test]
    fn example_4() {
        assert_eq!("LVIII", Solution::int_to_roman(58));
    }

    #[test]
    fn example_5() {
        assert_eq!("MCMXCIV", Solution::int_to_roman(1994));
    }
}
