use core::panic;
use std::{cmp::Ordering, fs, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    fn covered_area_boundary(&self) -> Vec<Line> {
        let distance_to_closest = self.loc.dist(self.closest_beacon);
        let top = Pos { x: self.loc.x, y: self.loc.y - distance_to_closest };
        let bottom = Pos { x: self.loc.x, y: self.loc.y + distance_to_closest };
        let left = Pos { x: self.loc.x - distance_to_closest, y: self.loc.y };
        let right = Pos { x: self.loc.x + distance_to_closest, y: self.loc.y };

        vec![
            Line { p1: top, p2: right },
            Line { p1: right, p2: bottom },
            Line { p1: bottom, p2: left },
            Line { p1: left, p2: top },
        ]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    p1: Pos,
    p2: Pos,
}

impl Line {
    // See https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection#Given_two_points_on_each_line_segment
    fn intersection(&self, other: Line) -> Option<Pos> {
        let denominator =
            (self.p1.x - self.p2.x) * (other.p1.y - other.p2.y) - (self.p1.y - self.p2.y) * (other.p1.x - other.p2.x);
        if denominator == 0 {
            None
        } else {
            let l1_part = self.p1.x * self.p2.y - self.p1.y * self.p2.x;
            let l2_part = other.p1.x * other.p2.y - other.p1.y * other.p2.x;
            let x = (other.p1.x - other.p2.x)
                .checked_mul(l1_part)?
                .checked_sub((self.p1.x - self.p2.x).checked_mul(l2_part)?)?;
            let y = ((other.p1.y - other.p2.y).checked_mul(l1_part)?)
                .checked_sub((self.p1.y - self.p2.y).checked_mul(l2_part)?)?;

            Some(Pos { x: x / denominator, y: y / denominator })
        }
    }

    fn slope(&self) -> i64 {
        (self.p2.y - self.p1.y) as i64 / (self.p2.x - self.p1.x) as i64
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
    let (pos_slope, neg_slope): (Vec<_>, Vec<_>) =
        sensors.iter().flat_map(|x| x.covered_area_boundary()).partition(|x| x.slope() == 1);

    let mut intersections = vec![];
    for l1 in &pos_slope {
        for l2 in &neg_slope {
            if let Some(intersection) = l1.intersection(*l2) {
                if intersection.x >= -5
                    && intersection.y >= -5
                    && intersection.x <= beacon_max_val + 10
                    && intersection.y <= beacon_max_val + 10
                {
                    intersections.push(intersection);
                }
            }
        }
    }

    for &i1 in &intersections {
        for &i2 in &intersections {
            let candidates = if i1.y == i2.y && (i1.x - i2.x).abs() == 2 {
                let x = if i1.x > i2.x { i2.x + 1 } else { i1.x + 1 };
                vec![Pos { x, y: i1.y }]
            } else if i1.x == i2.x && (i1.y - i2.y).abs() == 2 {
                let y = if i1.y > i2.y { i2.y + 1 } else { i1.y + 1 };
                vec![Pos { x: i1.x, y }]
            } else if i1.dist(i2) == 2 {
                vec![Pos { x: i1.x, y: i2.y }, Pos { x: i2.x, y: i1.y }]
            } else {
                vec![]
            };

            for candidate in candidates {
                if candidate.x >= 0
                    && candidate.y >= 0
                    && candidate.x <= beacon_max_val
                    && candidate.y <= beacon_max_val
                    && !(sensors.iter().any(|sensor| sensor.covers(candidate)))
                {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

fn p2(sensors: &Vec<Sensor>, beacon_max_val: i64) -> i64 {
    match find_beacon_coords(sensors, beacon_max_val) {
        Some(Pos { x, y }) => 4000000 * x + y,
        None => panic!("cannot find the answer"),
    }
}

fn main() {
    let sensors = parse_input("../inputs/d15");

    let now = Instant::now();
    let ans = p1(&sensors, 2000000);
    let duration = now.elapsed();
    println!("p1 ans = {ans} [{duration:?}]");

    let now = Instant::now();
    let ans = p2(&sensors, 4000000);
    let duration = now.elapsed();
    println!("p2 ans = {ans} [{duration:?}]");
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

        let sensors = parse_input("../inputs/d15");
        assert_eq!(p2(&sensors, 4000000), 13213086906101);
    }

    #[test]
    fn covered_area_boundary_test() {
        let test_sensors = parse_input("../inputs/d15_test");
        assert_eq!(
            test_sensors[6].covered_area_boundary(),
            vec![
                Line { p1: Pos { x: 8, y: -2 }, p2: Pos { x: 17, y: 7 } },
                Line { p1: Pos { x: 17, y: 7 }, p2: Pos { x: 8, y: 16 } },
                Line { p1: Pos { x: 8, y: 16 }, p2: Pos { x: -1, y: 7 } },
                Line { p1: Pos { x: -1, y: 7 }, p2: Pos { x: 8, y: -2 } }
            ]
        );
    }

    #[test]
    fn line_intersection_test() {
        let line1 = Line { p1: Pos { x: -1, y: 0 }, p2: Pos { x: 1, y: 2 } };
        let line2 = Line { p1: Pos { x: 0, y: 0 }, p2: Pos { x: 3, y: 3 } };
        assert_eq!(line1.intersection(line2), None);

        let line1 = Line { p1: Pos { x: 8, y: -2 }, p2: Pos { x: 17, y: 7 } };
        let line2 = Line { p1: Pos { x: 14, y: -2 }, p2: Pos { x: 8, y: 4 } };
        assert_eq!(line1.intersection(line2), Some(Pos { x: 11, y: 1 }));
    }
}
