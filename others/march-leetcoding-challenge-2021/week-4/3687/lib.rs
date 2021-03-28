pub struct Solution;

impl Solution {
    pub fn original_digits(s: String) -> String {
        let counts = s.as_bytes().iter().fold([0; 26], |mut acc, x| {
            acc[(x - b'a') as usize] += 1;
            acc
        });
        let mut answer = [0; 10];
        answer[0] = counts[(b'z' - b'a') as usize];
        answer[2] = counts[(b'w' - b'a') as usize];
        answer[4] = counts[(b'u' - b'a') as usize];
        answer[6] = counts[(b'x' - b'a') as usize];
        answer[8] = counts[(b'g' - b'a') as usize];
        answer[3] = counts[(b'h' - b'a') as usize] - answer[8];
        answer[5] = counts[(b'f' - b'a') as usize] - answer[4];
        answer[7] = counts[(b's' - b'a') as usize] - answer[6];
        answer[1] = counts[(b'o' - b'a') as usize] - answer[0] - answer[2] - answer[4];
        answer[9] = counts[(b'i' - b'a') as usize] - answer[5] - answer[6] - answer[8];
        (0..10)
            .flat_map(|i| std::iter::repeat((i as u8 + b'0') as char).take(answer[i]))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("012", Solution::original_digits(String::from("owoztneoer")));
    }

    #[test]
    fn example_2() {
        assert_eq!("45", Solution::original_digits(String::from("fviefuro")));
    }
}
