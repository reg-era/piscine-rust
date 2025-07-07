use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

pub use Event::*;

use std::fmt;

impl fmt::Display for Notification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let position = match &self.position {
            Position::Top => "Top",
            Position::Bottom => "Bottom",
            Position::Center => "Center",
        };
        let colored_content = self
            .content
            .truecolor(self.color.0, self.color.1, self.color.2);

        write!(f, "({}, {}, {})", position, self.size, colored_content)
    }
}
use std::fmt::Debug;
impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Registration(time) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    format_duration(*time)
                ),
            },
            Appointment(msg) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: msg.to_string(),
            },
            Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_seconds_f32();
    let hours = total_seconds / 3600.;
    let minutes = (total_seconds % 3600.) / 60.;
    let seconds = total_seconds % 60.;

    format!("{}H:{}M:{}S", hours as i32, minutes as i32, seconds as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let remainder = Remainder("Go to the doctor");
        println!("{}", remainder.notify());
        let registration = Registration(Duration::seconds(49094));
        println!("{}", registration.notify());
        let appointment = Appointment("Go to the doctor");
        println!("{}", appointment.notify());
        let holiday = Event::Holiday;
        println!("{}", holiday.notify());
    }
}
