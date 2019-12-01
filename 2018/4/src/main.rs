use failure::Error;
use std::collections::HashMap;
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Guard {
    pub awake: bool,
    pub total_nap_time: u64,
    pub id: u64,
    sleep_cnt_per_min: Vec<u64>,
    nap_start: u64,
}

impl Guard {
    pub fn new(id: u64) -> Guard {
        Guard {
            awake: true,
            id: id,
            sleep_cnt_per_min: vec![0; 60],
            nap_start: 0,
            total_nap_time: 0,
        }
    }

    pub fn sleep(&mut self, minute: u64) {
        self.nap_start = minute;
        self.awake = false;
    }

    pub fn wake(&mut self, minute: u64) {
        if self.awake {
            return;
        }

        // Increment each minute in a nap so we can total the count over all days
        for m in self.nap_start..minute {
            self.sleep_cnt_per_min[m as usize] += 1;
        }
        self.total_nap_time += minute - self.nap_start;
        self.awake = true;
    }

    pub fn nap_cnt_per_min(&self, minute: u64) -> u64 {
        return self.sleep_cnt_per_min[minute as usize];
    }

    pub fn sleepiest_minute(&self) -> (usize, u64) {
        let mut index = 0;
        let mut max = 0;
        for (i, _) in self.sleep_cnt_per_min.iter().enumerate() {
            if self.sleep_cnt_per_min[i] > max {
                max = self.sleep_cnt_per_min[i];
                index = i;
            }
        }

        return (index, max);
    }
}

#[derive(Debug)]
struct Date {
    year: u64,
    month: u64,
    day: u64,
    hour: u64,
    minute: u64,
}

impl Date {
    pub fn new(s: &str) -> Date {
        Date {
            year: s[1..=4].parse().unwrap(),
            month: s[6..=7].parse().unwrap(),
            day: s[9..=10].parse().unwrap(),
            hour: s[12..=13].parse().unwrap(),
            minute: s[15..=16].parse().unwrap(),
        }
    }
}

fn main() -> Result<(), Error> {
    let buf = fs::read_to_string("input.txt")?;
    let mut input: Vec<&str> = buf.lines().collect();
    input.sort();

    let mut guards: HashMap<u64, Guard> = HashMap::new();
    let mut active_guard_id = 0;

    for line in input {
        let date = Date::new(&line[0..=17]);
        let c = line.chars().nth(19).unwrap();
        // The word starting at offset 19 is always 'Guard', 'falls', or 'wakes'
        if c == 'G' {
            // Grab the guard id out of the string and add it if we haven't seen this guard before
            active_guard_id = line
                .split(&['#', ' '][..])
                .filter_map(|s| s.parse::<u64>().ok())
                .collect::<Vec<u64>>()[0];
            guards
                .entry(active_guard_id)
                .or_insert(Guard::new(active_guard_id));
        } else {
            let guard = guards.get_mut(&active_guard_id).unwrap();
            if c == 'f' {
                guard.sleep(date.minute);
            } else if c == 'w' {
                guard.wake(date.minute);
            }
        }
    }

    // Find which guard slept the most. Most of this logic is in the Guard class, so we just
    // need to find the highest nap count
    let mut sleepy_guard = 0;
    let mut longest_nap_time = 0;
    for (_, guard) in &guards {
        if guard.total_nap_time > longest_nap_time {
            sleepy_guard = guard.id;
            longest_nap_time = guard.total_nap_time;
        }
    }

    let guard = &guards[&sleepy_guard];
    let (minute, _) = guard.sleepiest_minute();
    // part 1
    println!(
        "sleepiest guard was #{:?}, sleepy minute is {}, so answer is {}",
        sleepy_guard,
        minute,
        sleepy_guard * minute as u64
    );

    // part 2
    for (_, guard) in guards {
        println!("#{} {:?}", guard.id, guard.sleepiest_minute());
    }

    Ok(())
}

#[test]
fn quick_minute_nap() {
    const GUARD_ID: u64 = 1;
    const SLEEP_TIME: u64 = 10;
    const WAKE_TIME: u64 = 11;

    let mut guard = Guard::new(GUARD_ID);
    assert_eq!(guard.id, GUARD_ID);

    guard.sleep(SLEEP_TIME);
    assert_eq!(guard.awake, false);
    guard.wake(WAKE_TIME);;

    assert_eq!(guard.awake, true);
    assert_eq!(guard.total_nap_time, WAKE_TIME - SLEEP_TIME);
}

#[test]
fn multiple_naps() {
    const GUARD_ID: u64 = 27;
    let mut guard = Guard::new(GUARD_ID);
    assert_eq!(guard.id, GUARD_ID);

    guard.sleep(10);
    guard.wake(15);

    guard.sleep(27);
    guard.wake(31);

    guard.sleep(38);
    guard.wake(52);

    assert_eq!(guard.awake, true);
    assert_eq!(guard.total_nap_time, 23);
}

#[test]
fn count_nap_minute_counts() {
    const GUARD_ID: u64 = 17;
    let mut guard = Guard::new(GUARD_ID);

    guard.sleep(10);
    guard.wake(20);
    guard.sleep(13);
    guard.wake(19);
    guard.sleep(18);
    guard.wake(20);
    guard.sleep(19);
    guard.wake(21);

    assert_eq!(guard.nap_cnt_per_min(10), 1);
    assert_eq!(guard.nap_cnt_per_min(14), 2);
    assert_eq!(guard.nap_cnt_per_min(18), 3);
    assert_eq!(guard.nap_cnt_per_min(19), 3);
    assert_eq!(guard.nap_cnt_per_min(20), 1);
    assert_eq!(guard.sleepiest_minute(), 18);
}

#[test]
fn test_date_constructor() {
    let date = Date::new("[1518-09-25 00:52]");

    assert_eq!(date.year, 1518);
    assert_eq!(date.month, 9);
    assert_eq!(date.day, 25);
    assert_eq!(date.hour, 0);
    assert_eq!(date.minute, 52);
}
