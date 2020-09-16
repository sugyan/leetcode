#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 2],
}

pub struct Solution {}

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut trie: Trie = Default::default();
        for &num in nums.iter() {
            let mut node = &mut trie;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                node = node.children[bit].get_or_insert_with(Default::default);
            }
        }
        let mut answer = 0;
        for &num in nums.iter() {
            let mut max = 0;
            let mut node = &trie;
            for i in (0..31).rev() {
                let bit = ((num >> i) & 1) as usize;
                if let Some(n) = &node.children[1 - bit] {
                    max |= 1 << i;
                    node = n;
                } else {
                    node = &node.children[bit].as_ref().unwrap();
                }
            }
            answer = std::cmp::max(answer, max);
        }
        answer
    }
    // (Bit manipulation)
    // pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    //     use std::collections::HashSet;
    //     let (mut max, mut mask) = (0, 0);
    //     for i in (0..31).rev() {
    //         mask |= 1 << i;
    //         let mut hs: HashSet<i32> = HashSet::with_capacity(nums.len());
    //         for &num in nums.iter() {
    //             hs.insert(num & mask);
    //         }
    //         let tmp = max | (1 << i);
    //         for prefix in hs.iter() {
    //             if hs.contains(&(prefix ^ tmp)) {
    //                 max = tmp;
    //                 break;
    //             }
    //         }
    //     }
    //     max
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(28, Solution::find_maximum_xor(vec![3, 10, 5, 25, 2, 8]));
    }
}
