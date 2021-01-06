use std::cmp::Eq;
use std::cmp::PartialEq;
use std::string::ToString;
use std::fmt::Debug;

const MINUTES_IN_AN_HOUR: i32 = 60;
const MINUTES_IN_A_DAY: i32 = 24 * MINUTES_IN_AN_HOUR;

fn modulo(x: i32, m: i32) -> i32 {
    ((x % m) + m) % m
}

pub struct Clock {
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = Self::to_minutes(hours, minutes);

        Self::from_minutes(total_minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::from_minutes(self.minutes + minutes)
    }

    pub fn hours(&self) -> i32 {
        self.minutes / MINUTES_IN_AN_HOUR
    }

    pub fn minutes(&self) -> i32 {
        self.minutes % MINUTES_IN_AN_HOUR
    }

    fn to_minutes(hours: i32, minutes: i32) -> i32 {
        return modulo(hours * MINUTES_IN_AN_HOUR + minutes, MINUTES_IN_A_DAY)
    }

    fn from_minutes(minutes: i32) -> Self {
        let positive_non_overflowing_minutes = modulo(minutes, MINUTES_IN_A_DAY);

        Clock {
            minutes: positive_non_overflowing_minutes
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.minutes == other.minutes
    }
}

impl Eq for Clock {}

impl ToString for Clock {
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours(), self.minutes())
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("Clock")
         .field("minutes", &self.minutes)
         .finish()
    }
}
