use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

const MINS_PER_DAY: i32 = 60 * 24;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            minutes: (hours * 60 + minutes).rem_euclid(MINS_PER_DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(0, self.minutes + minutes)
    }

    fn internal_from(clk_str: &str) -> Self {
        let fields = clk_str.split(":").collect::<Vec<&str>>();
        match fields.len() {
            0 => Self { minutes: 0 },
            1 => Self::new(fields[0].parse().unwrap_or(0) * 60, 0),
            _ => Self::new(
                fields[0].parse().unwrap_or(0),
                fields[1].parse().unwrap_or(0),
            ),
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl From<&Clock> for String {
    fn from(clock: &Clock) -> Self {
        let hours = clock.minutes.div_euclid(60);
        let minutes = clock.minutes.rem_euclid(60);
        format!("{:02}:{:02}", hours, minutes)
    }
}
impl From<&str> for Clock {
    fn from(clk_str: &str) -> Self {
        Clock::internal_from(clk_str)
    }
}

impl From<String> for Clock {
    fn from(clk_str: String) -> Self {
        Clock::internal_from(&clk_str[..])
    }
}
