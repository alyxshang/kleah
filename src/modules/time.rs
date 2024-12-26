/*
Jade by Alyx Shang.
Licensed under the FSL v1.
*/

/// Using the "Local"
/// structure from the "chrono"
/// crate to retrieve the current 
/// time.
use chrono::offset::Local;

/// A generic structure to
/// hold information on the current
/// local time.
pub struct TimeNow{
    pub year: String,
    pub month: String,
    pub day: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String
}

/// Implementing generic
/// methods for the "TimeNow"
/// structure.
impl TimeNow{

    /// Implementing a "new"
    /// method for the "TimeNow"
    /// structure.
    pub fn new() -> TimeNow {
        let time = Local::now();
        let date = time.date_naive();
        let curr_time = time.time();
        let year: String = format!("{}",date.format("%Y"));
        let month: String = format!("{}",date.format("%m"));
        let day: String = format!("{}",date.format("%d"));
        let hours: String = format!("{}",curr_time.format("%H"));
        let minutes: String = format!("{}",curr_time.format("%M"));
        let seconds: String = format!("{}",curr_time.format("%S"));
        TimeNow {
            year: year,
            month: month,
            day: day,
            hours: hours,
            minutes: minutes,
            seconds: seconds
        }
    }
}

/// Gets the current time 
/// in the format "YYYY-MM-DD/HH:MM:SS".
pub fn get_time() -> String {
    let time_now: TimeNow = TimeNow::new();
    format!(
        "{}-{}-{}/{}:{}:{}",
        time_now.year,
        time_now.month,
        time_now.day,
        time_now.hours,
        time_now.minutes,
        time_now.seconds,
    )
}