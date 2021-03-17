use rand::{rngs::ThreadRng, Rng};

#[derive(Default)]
pub struct Solution {
    radius: f64,
    x_center: f64,
    y_center: f64,
    rng: ThreadRng,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {
    pub fn new(radius: f64, x_center: f64, y_center: f64) -> Self {
        Self {
            radius,
            x_center,
            y_center,
            ..Self::default()
        }
    }

    pub fn rand_point(&mut self) -> Vec<f64> {
        let r = self.rng.gen::<f64>().sqrt() * self.radius;
        let theta = self.rng.gen::<f64>() * 2.0 * std::f64::consts::PI;
        [
            self.x_center + r * theta.cos(),
            self.y_center + r * theta.sin(),
        ]
        .to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let radius = 1.0;
        let (x_center, y_center) = (0.0, 0.0);
        let mut obj = Solution::new(radius, x_center, y_center);
        for _ in 0..100 {
            let ret: Vec<f64> = obj.rand_point();
            assert!(((ret[0] - x_center).powi(2) + (ret[1] - y_center).powi(2)).sqrt() < radius)
        }
    }

    #[test]
    fn example_2() {
        let radius = 10.0;
        let (x_center, y_center) = (5.0, -7.5);
        let mut obj = Solution::new(radius, x_center, y_center);
        for _ in 0..100 {
            let ret: Vec<f64> = obj.rand_point();
            assert!(((ret[0] - x_center).powi(2) + (ret[1] - y_center).powi(2)).sqrt() < radius)
        }
    }
}
