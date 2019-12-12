pub struct Solution {}

impl Solution {
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut v: Vec<char> = (0..n as u8).map(|i| ('1' as u8 + i) as char).collect();
        let mut k = k;
        let mut m = 1;
        for i in 1..=n {
            m *= i;
        }
        let mut answer: Vec<char> = vec![' '; n as usize];
        for i in 0..n as usize {
            m /= n - (i as i32);
            let pos = ((k - 1) / m) as usize;
            answer[i] = v[pos];
            v.remove(pos);
            k -= m * pos as i32;
        }
        return answer.iter().collect::<String>();
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
