use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        let mut bh = BinaryHeap::new();
        courses.sort_by_cached_key(|v| v[1]);
        let mut total = 0;
        for course in &courses {
            if total + course[0] <= course[1] {
                total += course[0];
                bh.push(course[0]);
            } else if let Some(max) = bh.pop() {
                let min = max.min(course[0]);
                total += min - max;
                bh.push(min);
            }
        }
        bh.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            3,
            Solution::schedule_course(vec![
                vec![100, 200],
                vec![200, 1300],
                vec![1000, 1250],
                vec![2000, 3200]
            ])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(1, Solution::schedule_course(vec![vec![1, 2]]));
    }

    #[test]
    fn example_3() {
        assert_eq!(0, Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]));
    }
}
