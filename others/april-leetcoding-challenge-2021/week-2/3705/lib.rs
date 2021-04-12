pub struct Solution;

impl Solution {
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        (0..n)
            .map(|i| {
                1 + if (1..=k).contains(&i) {
                    if i % 2 == 0 {
                        i / 2
                    } else {
                        k - i / 2
                    }
                } else {
                    i
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(vec![1, 2, 3], Solution::construct_array(3, 1));
    }

    #[test]
    fn example_2() {
        assert_eq!(vec![1, 3, 2], Solution::construct_array(3, 2));
    }
}
