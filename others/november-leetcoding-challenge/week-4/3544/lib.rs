pub struct Solution {}

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut answer = 0;
        for i in 1..=26 {
            let mut dict = [0; 26];
            let (mut l, mut r) = (0, 0);
            let mut uniq = 0;
            let mut ok = 0;
            while r < s.len() {
                if uniq <= i {
                    let idx = (s[r] - b'a') as usize;
                    if dict[idx] == 0 {
                        uniq += 1;
                    }
                    dict[idx] += 1;
                    if dict[idx] == k {
                        ok += 1;
                    }
                    r += 1;
                } else {
                    let idx = (s[l] - b'a') as usize;
                    if dict[idx] == k {
                        ok -= 1;
                    }
                    dict[idx] -= 1;
                    if dict[idx] == 0 {
                        uniq -= 1;
                    }
                    l += 1;
                }
                if uniq == i && uniq == ok {
                    answer = std::cmp::max(answer, r - l)
                }
            }
        }
        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(3, Solution::longest_substring(String::from("aaabb"), 3));
    }

    #[test]
    fn example_2() {
        assert_eq!(5, Solution::longest_substring(String::from("ababbc"), 2));
    }
}
