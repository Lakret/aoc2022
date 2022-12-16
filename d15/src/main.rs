use core::panic;
use std::{cmp::Ordering, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn dist(&self, other: Pos) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<'a> From<&'a str> for Pos {
    fn from(s: &'a str) -> Self {
        let mut s = s.split(", ");
        let x = s.next().unwrap().strip_prefix("x=").unwrap().parse().unwrap();
        let y = s.next().unwrap().strip_prefix("y=").unwrap().parse().unwrap();
        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sensor {
    loc: Pos,
    closest_beacon: Pos,
}

impl Sensor {
    /// Returns true if the `sensor` cover `pos`, i.e. if the beacon cannot be located here.
    fn covers(&self, pos: Pos) -> bool {
        let distance_to_closest = self.loc.dist(self.closest_beacon);
        self.loc.dist(pos) <= distance_to_closest
    }

    /// Returns an interval covered by the `sensor` on `y` horizontal line
    /// as an optional `Interval`. If no coverage on `y` exists for this sensor,
    /// returns `None`.
    fn covers_on_y(&self, y: i64) -> Option<Interval> {
        if self.covers(Pos { x: self.loc.x, y }) {
            let mut min_x = self.loc.x - 1;
            while self.covers(Pos { x: min_x, y }) {
                min_x -= 1;
            }
            let mut max_x = self.loc.x + 1;
            while self.covers(Pos { x: max_x, y }) {
                max_x += 1;
            }
            Some(Interval { min_val: min_x + 1, max_val: max_x - 1 })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Interval {
    min_val: i64,
    max_val: i64,
}

impl Interval {
    // Returns combined or non-overlapping intervals.
    // Assumes that `self` is less or equal to `other`.
    fn combine_sorted(self, other: Interval) -> Vec<Interval> {
        if self == other {
            vec![self]
        // touching or overlapping
        } else if self.max_val == other.min_val || (self.max_val >= other.min_val && self.min_val <= other.min_val) {
            vec![Interval { min_val: self.min_val.min(other.min_val), max_val: self.max_val.max(other.max_val) }]
        // no overlap
        } else {
            vec![self, other]
        }
    }

    fn distinct_positions(self) -> i64 {
        if self.max_val == self.min_val {
            1
        } else {
            (self.max_val - self.min_val).abs()
        }
    }
}

impl Ord for Interval {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.min_val < other.min_val {
            Ordering::Less
        } else if self.min_val > other.min_val {
            Ordering::Greater
        } else if self.min_val == other.min_val && self.max_val < other.max_val {
            Ordering::Less
        } else if self.min_val == other.min_val && self.max_val > other.max_val {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for Interval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(path: &str) -> Vec<Sensor> {
    let mut sensors = vec![];
    for line in fs::read_to_string(path).unwrap().trim_end().split("\n") {
        let mut line = line.strip_prefix("Sensor at ").unwrap().split(": closest beacon is at ");
        let loc = line.next().unwrap().into();
        let closest_beacon = line.next().unwrap().into();
        sensors.push(Sensor { loc, closest_beacon })
    }
    sensors
}

fn all_covers_on_y_combined(sensors: &Vec<Sensor>, y: i64) -> Vec<Interval> {
    let mut intervals = vec![];
    for sensor in sensors {
        if let Some(interval) = sensor.covers_on_y(y) {
            intervals.push(interval);
        }
    }
    intervals.sort_unstable();

    let intervals = intervals.iter().fold(vec![intervals[0]], |mut acc, prev| {
        let mut new_last = acc.last().unwrap().combine_sorted(*prev);
        acc.drain((acc.len() - 1)..);
        acc.append(&mut new_last);
        acc
    });

    intervals
}

fn p1(sensors: &Vec<Sensor>, y: i64) -> i64 {
    all_covers_on_y_combined(sensors, y).iter().map(|i| i.distinct_positions()).sum()
}

fn find_beacon_coords(sensors: &Vec<Sensor>, beacon_max_val: i64) -> Option<Pos> {
    for y in 0..(beacon_max_val + 1) {
        if y % 1000 == 0 {
            println!("y = {y}...");
        }
        let cover_xs = all_covers_on_y_combined(sensors, y);
        match &cover_xs[..] {
            [] => panic!("too many options at y = {y}!"),
            [Interval { min_val, max_val }] if *min_val <= 0 && *max_val >= beacon_max_val => continue,
            covers => {
                let mut x = 0;
                for cover in covers {
                    if cover.min_val <= x && cover.max_val >= x {
                        x = cover.max_val + 1
                    } else {
                        return Some(Pos { x, y });
                    }
                }
            }
        }
    }
    None
}

use rayon::prelude::*;

fn find_beacon_coords_par(sensors: &Vec<Sensor>, beacon_max_val: i64) -> Option<Pos> {
    (0..(beacon_max_val + 1)).into_par_iter().find_map_first(|y| {
        if y % 1000 == 0 {
            println!("y = {y}...");
        }
        let cover_xs = all_covers_on_y_combined(sensors, y);
        match &cover_xs[..] {
            [] => panic!("too many options at y = {y}!"),
            [Interval { min_val, max_val }] if *min_val <= 0 && *max_val >= beacon_max_val => None,
            covers => {
                let mut x = 0;
                for cover in covers {
                    if cover.min_val <= x && cover.max_val >= x {
                        x = cover.max_val + 1
                    } else {
                        break;
                    }
                }
                if x <= beacon_max_val {
                    Some(Pos { x, y })
                } else {
                    None
                }
            }
        }
    })
}

fn p2(sensors: &Vec<Sensor>, beacon_max_val: i64) -> i64 {
    match find_beacon_coords_par(sensors, beacon_max_val) {
        Some(Pos { x, y }) => 4000000 * x + y,
        None => panic!("cannot find the answer"),
    }
}

fn main() {
    let sensors = parse_input("../inputs/d15");
    let ans = p2(&sensors, 2000000);
    println!("p2 ans = {ans}.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test() {
        let test_sensors = parse_input("../inputs/d15_test");
        assert_eq!(test_sensors.len(), 14);
        assert_eq!(&test_sensors[0], &Sensor { loc: Pos { x: 2, y: 18 }, closest_beacon: Pos { x: -2, y: 15 } });
        assert_eq!(&test_sensors[13], &Sensor { loc: Pos { x: 20, y: 1 }, closest_beacon: Pos { x: 15, y: 3 } });

        let test_sensors = parse_input("../inputs/d15");
        assert_eq!(test_sensors.len(), 33);
        assert_eq!(
            &test_sensors[0],
            &Sensor { loc: Pos { x: 2899860, y: 3122031 }, closest_beacon: Pos { x: 2701269, y: 3542780 } }
        );
        assert_eq!(
            &test_sensors[32],
            &Sensor { loc: Pos { x: 2797371, y: 3645126 }, closest_beacon: Pos { x: 2701269, y: 3542780 } }
        );
    }

    #[test]
    fn covers_test() {
        let test_sensors = parse_input("../inputs/d15_test");
        assert_eq!(test_sensors[6].covers(Pos { x: 8, y: 16 }), true);
        assert_eq!(test_sensors[6].covers(Pos { x: 8, y: 17 }), false);
        assert_eq!(test_sensors[6].covers(Pos { x: -1, y: 7 }), true);
        assert_eq!(test_sensors[6].covers(Pos { x: -2, y: 7 }), false);
        assert_eq!(test_sensors[6].covers(Pos { x: 17, y: 7 }), true);
        assert_eq!(test_sensors[6].covers(Pos { x: 18, y: 7 }), false);
        assert_eq!(test_sensors[6].covers(Pos { x: 8, y: -2 }), true);
        assert_eq!(test_sensors[6].covers(Pos { x: 8, y: -3 }), false);
        assert_eq!(test_sensors[6].covers(Pos { x: 7, y: -1 }), true);
        assert_eq!(test_sensors[6].covers(Pos { x: 8, y: -1 }), true);
        assert_eq!(test_sensors[6].covers(Pos { x: 6, y: -1 }), false);
    }

    #[test]
    fn covers_on_y_test() {
        let test_sensors = parse_input("../inputs/d15_test");
        assert_eq!(test_sensors[6].covers_on_y(9), Some(Interval { min_val: 1, max_val: 15 }));
        assert_eq!(test_sensors[6].covers_on_y(-2), Some(Interval { min_val: 8, max_val: 8 }));
        assert_eq!(test_sensors[6].covers_on_y(17), None);
    }

    #[test]
    fn combine_sorted_test() {
        let ints = vec![
            Interval { min_val: -2, max_val: 2 },
            Interval { min_val: 2, max_val: 2 },
            Interval { min_val: 2, max_val: 14 },
            Interval { min_val: 12, max_val: 12 },
            Interval { min_val: 14, max_val: 18 },
            Interval { min_val: 16, max_val: 24 },
        ];

        assert_eq!(ints[0].combine_sorted(ints[1]), vec![Interval { min_val: -2, max_val: 2 }]);
        assert_eq!(
            Interval { min_val: -2, max_val: 2 }.combine_sorted(ints[2]),
            vec![Interval { min_val: -2, max_val: 14 }]
        );
        assert_eq!(ints[2].combine_sorted(ints[3]), vec![Interval { min_val: 2, max_val: 14 }]);
        assert_eq!(ints[4].combine_sorted(ints[5]), vec![Interval { min_val: 14, max_val: 24 }]);
        assert_eq!(ints[0].combine_sorted(ints[5]), vec![ints[0], ints[5]]);
    }

    #[test]
    fn p1_test() {
        let test_sensors = parse_input("../inputs/d15_test");
        assert_eq!(p1(&test_sensors, 10), 26);

        let sensors = parse_input("../inputs/d15");
        assert_eq!(p1(&sensors, 2000000), 5240818);
    }

    #[test]
    fn p2_test() {
        let test_sensors = parse_input("../inputs/d15_test");
        assert_eq!(find_beacon_coords(&test_sensors, 10), None);
        assert_eq!(find_beacon_coords(&test_sensors, 20), Some(Pos { x: 14, y: 11 }));
        assert_eq!(p2(&test_sensors, 20), 56000011);

        // TODO:
        // let sensors = parse_input("../inputs/d15");
        // assert_eq!(p2(&sensors, 2000000), 0);
    }
}
