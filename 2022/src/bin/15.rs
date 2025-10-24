// Solving https://adventofcode.com/2022/day/15
advent_of_code_2022::solution!(15);

const TEST_ROW: i32 = 10;
const REAL_ROW: i32 = 2_000_000;
const MIN_COORD: i32 = 0;
const MIN_TEST_COORD: i32 = 20;
const MIN_REAL_COORD: i32 = 4_000_000;
const TUNING_CONST: u64 = 4_000_000;

struct Sensor {
    x: i32,
    y: i32,
}

struct Beacon {
    x: i32,
    y: i32,
}

pub fn part_one(input: &str) -> Option<i32> {
    let is_real = input.len() > 1000;
    let sensors = get_input(is_real);
    let (sensors, beacons) = input_to_sensor_ranges(sensors);
    let (min_out, max_out) = get_furthest_out(&sensors);
    let mut count = 0;
    let row = get_row(is_real);
    for col_idx in min_out..max_out {
        if sensor_in_range(&sensors, row, col_idx) && not_beacon(&beacons, row, col_idx) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let is_real = input.len() > 1000;
    let sensors = get_input(is_real);
    let (sensors, _) = input_to_sensor_ranges(sensors);
    let max_coord = get_max_coord(is_real);
    for row_idx in MIN_COORD..max_coord {
        match check_row(&sensors, max_coord, row_idx) {
            None => (),
            Some(x) => {
                return Some((x as u64) * TUNING_CONST + (row_idx as u64));
            },
        }
    }
    None
}

fn get_ranges(sensors: &Vec<(Sensor, i32)>, max_coord: i32, row: i32) -> Vec<(i32, i32)> {
    let mut bad_ranges: Vec<(i32, i32)> = Vec::new();
    for (sensor, r) in sensors {
        let dist_remaining = r - (sensor.y - row).abs();
        if dist_remaining >= 0 {
            let mut start = sensor.x - dist_remaining;
            if start < 0 {
                start = 0;
            }
            let mut end = sensor.x + dist_remaining;
            if end > max_coord {
                end = max_coord;
            }
            bad_ranges.push((start, end));
        }
    }
    bad_ranges
}

fn check_row(sensors: &Vec<(Sensor, i32)>, max_coord: i32, row: i32) -> Option<i32> {
    let mut bad_ranges = get_ranges(sensors, max_coord, row);
    bad_ranges.sort();
    let mut total_range = bad_ranges.get(0).unwrap().clone();
    bad_ranges.remove(0);
    bad_ranges.reverse();
    for idx in (0..bad_ranges.len()).rev() {
        let item = bad_ranges.get(idx).unwrap();
        if (item.0 >= total_range.0 && item.0 <= total_range.1) ||
        (item.1 >= total_range.0 && item.1 <= total_range.1) {
            total_range = (total_range.0.min(item.0), total_range.1.max(item.1));
            bad_ranges.remove(idx);
        }
    }
    if total_range.0 != 0 {
        Some(0)
    } else if total_range.1 != max_coord {
        Some(total_range.1 + 1)
    } else {
        None
    }
}

fn get_max_coord(is_real: bool) -> i32 {
    if is_real {
        MIN_REAL_COORD
    } else {
        MIN_TEST_COORD
    }
}

fn sensor_in_range(sensors: &Vec<(Sensor, i32)>, row: i32, col: i32) -> bool {
    for (s, r) in sensors {
        let dist = (s.x - col).abs() + (s.y - row).abs();
        if dist <= *r {
            return true;
        }
    }
    false
}

fn not_beacon(beacons: &Vec<Beacon>, row: i32, col: i32) -> bool {
    for b in beacons {
        if b.x == col && b.y == row {
            return false;
        }
    }
    true
}

fn get_row(is_real: bool) -> i32 {
    if is_real {
        REAL_ROW
    } else {
        TEST_ROW
    }
}

fn get_furthest_out(sensors: &Vec<(Sensor, i32)>) -> (i32, i32) {
    let mut max = 0;
    let mut min = 100_000_000;
    for (s, r) in sensors {
        if s.x + r > max {
            max = s.x + r;
        }
        if s.x - r < min {
            min = s.x - r;
        }
    }
    (min, max)
}

fn input_to_sensor_ranges(input: Vec<(Sensor, Beacon)>) -> (Vec<(Sensor, i32)>, Vec<Beacon>) {
    let mut sensor_ranges: Vec<(Sensor, i32)> = Vec::new();
    let mut beacons: Vec<Beacon> = Vec::new();
    for (sense, beacon) in input {
        let dist = (sense.x - beacon.x).abs() + (sense.y - beacon.y).abs();
        sensor_ranges.push((sense, dist));
        beacons.push(beacon);
    }
    (sensor_ranges, beacons)
}

fn get_input(is_real: bool) -> Vec<(Sensor, Beacon)> {
    if is_real {
        vec![
            (Sensor { x: 1326566, y: 3575946 }, Beacon { x: 1374835, y: 2000000 }),
            (Sensor { x: 2681168, y: 3951549 }, Beacon { x: 3184941, y: 3924923 }),
            (Sensor { x: 3959984, y: 1095746 }, Beacon { x: 3621412, y: 2239432 }),
            (Sensor { x: 3150886, y: 2479946 }, Beacon { x: 3621412, y: 2239432 }),
            (Sensor { x: 3983027, y: 2972336 }, Beacon { x: 4012908, y: 3083616 }),
            (Sensor { x: 3371601, y: 3853300 }, Beacon { x: 3184941, y: 3924923 }),
            (Sensor { x: 3174612, y: 3992719 }, Beacon { x: 3184941, y: 3924923 }),
            (Sensor { x: 3316368, y: 1503688 }, Beacon { x: 3621412, y: 2239432 }),
            (Sensor { x: 3818181, y: 2331216 }, Beacon { x: 3621412, y: 2239432 }),
            (Sensor { x: 3960526, y: 3229321 }, Beacon { x: 4012908, y: 3083616 }),
            (Sensor { x: 61030, y: 3045273 }, Beacon { x: -467419, y: 2369316 }),
            (Sensor { x: 3635583, y: 3121524 }, Beacon { x: 4012908, y: 3083616 }),
            (Sensor { x: 2813357, y: 5535 }, Beacon { x: 3595763, y: -77322 }),
            (Sensor { x: 382745, y: 1566522 }, Beacon { x: 1374835, y: 2000000 }),
            (Sensor { x: 3585664, y: 538632 }, Beacon { x: 3595763, y: -77322 }),
            (Sensor { x: 3979654, y: 2158646 }, Beacon { x: 3621412, y: 2239432 }),
            (Sensor { x: 3996588, y: 2833167 }, Beacon { x: 4012908, y: 3083616 }),
            (Sensor { x: 3249383, y: 141800 }, Beacon { x: 3595763, y: -77322 }),
            (Sensor { x: 3847114, y: 225529 }, Beacon { x: 3595763, y: -77322 }),
            (Sensor { x: 3668737, y: 3720078 }, Beacon { x: 3184941, y: 3924923 }),
            (Sensor { x: 1761961, y: 680560 }, Beacon { x: 1374835, y: 2000000 }),
            (Sensor { x: 2556636, y: 2213691 }, Beacon { x: 3621412, y: 2239432 }),
            (Sensor { x: 65365, y: 215977 }, Beacon { x: 346716, y: -573228 }),
            (Sensor { x: 709928, y: 2270200 }, Beacon { x: 1374835, y: 2000000 }),
            (Sensor { x: 3673956, y: 2670437 }, Beacon { x: 4029651, y: 2547743 }),
            (Sensor { x: 3250958, y: 3999227 }, Beacon { x: 3184941, y: 3924923 }),
            (Sensor { x: 3009537, y: 3292368 }, Beacon { x: 3184941, y: 3924923 }),
        ]
    } else {
        vec![
            (Sensor { x: 2, y: 18 }, Beacon { x: -2, y: 15 }),
            (Sensor { x: 9, y: 16 }, Beacon { x: 10, y: 16 }),
            (Sensor { x: 13, y: 2 }, Beacon { x: 15, y: 3 }),
            (Sensor { x: 12, y: 14 }, Beacon { x: 10, y: 16 }),
            (Sensor { x: 10, y: 20 }, Beacon { x: 10, y: 16 }),
            (Sensor { x: 14, y: 17 }, Beacon { x: 10, y: 16 }),
            (Sensor { x: 8, y: 7 }, Beacon { x: 2, y: 10 }),
            (Sensor { x: 2, y: 0 }, Beacon { x: 2, y: 10 }),
            (Sensor { x: 0, y: 11 }, Beacon { x: 2, y: 10 }),
            (Sensor { x: 20, y: 14 }, Beacon { x: 25, y: 17 }),
            (Sensor { x: 17, y: 20 }, Beacon { x: 21, y: 22 }),
            (Sensor { x: 16, y: 7 }, Beacon { x: 15, y: 3 }),
            (Sensor { x: 14, y: 3 }, Beacon { x: 15, y: 3 }),
            (Sensor { x: 20, y: 1 }, Beacon { x: 15, y: 3 }),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_one(&input);
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code_2022::template::read_file("examples", DAY);
        let result = part_two(&input);
        assert_eq!(result, Some(56000011));
    }
}
