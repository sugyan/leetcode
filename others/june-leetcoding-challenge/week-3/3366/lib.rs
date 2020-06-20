pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut v: Vec<char> = (0..n as u8).map(|i| (b'1' + i) as char).collect();
        let mut k: i32 = k - 1;
        let mut m: i32 = (1..n).product();
        let mut answer: String = String::with_capacity(n as usize);
        for i in 0..n - 1 {
            answer.push(v.remove((k / m) as usize));
            k %= m;
            m /= n - i - 1;
        }
        answer.push(v[0]);
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!("213", Solution::get_permutation(3, 3));
    }

    #[test]
    fn example_2() {
        assert_eq!("2314", Solution::get_permutation(4, 9));
    }
}
