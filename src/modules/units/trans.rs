/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// A structurefor encapsulating
/// an actor's keys.
pub struct ActorKeys{
    pub private: String,
    pub public: String
}

/// A structurefor encapsulating
/// info about foreign actors.
pub struct ForeignActor {
    pub host: String,
    pub user: String
}

/// A generic structure to
/// hold information on the current
/// local time.
pub struct TimeNow{
    pub year: String,
    pub month: String,
    pub day: String,
    pub hours: String,
    pub minutes: String,
    pub seconds: String,
    pub millis: String
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
        let millis: String = format!("{}",curr_time.format("%f"));
        TimeNow {
            year,
            month,
            day,
            hours,
            minutes,
            seconds,
            millis
        }
    }
    
    /// Implementing a generic function
    /// to return a string representation
    /// of this structure.
    pub fn to_string(&self) -> String {
        format!(
            "{}{}{}{}{}{}{}",
            &self.year, 
            &self.month, 
            &self.day, 
            &self.hours, 
            &self.minutes, 
            &self.seconds,
            &self.millis
        )
    }
}
