use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut hours = (hours + (minutes / 60)) % 24;
        let mut minutes = minutes % 60;

        // messing around with Display and From traits
        println!(
            "clock String::from = {}",
            String::from(Clock { hours, minutes })
        );
        println!(
            "clock to_string = {}",
            (Clock { hours, minutes }).to_string()
        );
        println!(
            "clock from str (using Display::fmt): {}",
            Clock::from(String::from(Clock { hours, minutes }))
        );
        // println!("clock from str: {}", Clock::from("23:45"));

        if minutes < 0 {
            minutes = 60 + minutes;
            hours -= 1;
        }
        if hours < 0 {
            hours = 24 + hours;
        }
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }

    fn internal_from(clk_str: &str) -> Self {
        let fields = clk_str.split(":").collect::<Vec<&str>>();
        match fields.len() {
            0 => Clock {
                hours: 0,
                minutes: 0,
            },
            1 => Clock {
                hours: fields[0].parse().unwrap_or(0),
                minutes: 0,
            },
            _ => Clock {
                hours: fields[0].parse().unwrap_or(0),
                minutes: fields[1].parse().unwrap_or(0),
            },
        }
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl From<Clock> for String {
    fn from(clock: Clock) -> Self {
        format!("{:02}:{:02}", clock.hours, clock.minutes)
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
