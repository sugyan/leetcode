use std::collections::HashMap;

#[derive(Default)]
pub struct UndergroundSystem {
    checkins: HashMap<i32, (String, i32)>,
    times: HashMap<(String, String), Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.checkins.insert(id, (station_name, t));
    }

    pub fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        if let Some((start_station, start_t)) = self.checkins.get(&id) {
            self.times
                .entry((start_station.clone(), station_name))
                .or_default()
                .push(t - start_t);
        }
    }

    pub fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        if let Some(times) = self.times.get(&(start_station, end_station)) {
            return f64::from(times.iter().sum::<i32>()) / times.len() as f64;
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut obj = UndergroundSystem::new();
        obj.check_in(45, String::from("Leyton"), 3);
        obj.check_in(32, String::from("Paradise"), 8);
        obj.check_in(27, String::from("Leyton"), 10);
        obj.check_out(45, String::from("Waterloo"), 15);
        obj.check_out(27, String::from("Waterloo"), 20);
        obj.check_out(32, String::from("Cambridge"), 22);
        {
            let ret = obj.get_average_time(String::from("Paradise"), String::from("Cambridge"));
            assert!((ret - 14.00000).abs() < 10.0_f64.powi(-5));
        }
        {
            let ret = obj.get_average_time(String::from("Leyton"), String::from("Waterloo"));
            assert!((ret - 11.00000).abs() < 10.0_f64.powi(-5));
        }
        obj.check_in(10, String::from("Leyton"), 24);
        {
            let ret = obj.get_average_time(String::from("Leyton"), String::from("Waterloo"));
            assert!((ret - 11.00000).abs() < 10.0_f64.powi(-5));
        }
        obj.check_out(10, String::from("Waterloo"), 38);
        {
            let ret = obj.get_average_time(String::from("Leyton"), String::from("Waterloo"));
            assert!((ret - 12.00000).abs() < 10.0_f64.powi(-5));
        }
    }

    #[test]
    fn example_2() {
        let mut obj = UndergroundSystem::new();
        obj.check_in(10, String::from("Leyton"), 3);
        obj.check_out(10, String::from("Paradise"), 8);
        {
            let ret = obj.get_average_time(String::from("Leyton"), String::from("Paradise"));
            assert!((ret - 5.00000).abs() < 10.0_f64.powi(-5));
        }
        obj.check_in(5, String::from("Leyton"), 10);
        obj.check_out(5, String::from("Paradise"), 16);
        {
            let ret = obj.get_average_time(String::from("Leyton"), String::from("Paradise"));
            assert!((ret - 5.50000).abs() < 10.0_f64.powi(-5));
        }
        obj.check_in(2, String::from("Leyton"), 21);
        obj.check_out(2, String::from("Paradise"), 30);
        {
            let ret = obj.get_average_time(String::from("Leyton"), String::from("Paradise"));
            assert!((ret - 6.66667).abs() < 10.0_f64.powi(-5));
        }
    }
}
