use core::fmt::{self, Display};

use super::{Date, DateTime, Offset, Time};

impl Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // like `2015-11-02`
            Date::YMD { year, month, day } => write!(f, "{:04}-{:02}-{:02}", year, month, day),
            // like `2015-W45-01`
            Date::Week { year, ww, d } => write!(f, "{:04}-{:02}-{:02}", year, ww, d),
            // like `2015-306`
            Date::Ordinal { year, ddd } => write!(f, "{:04}-{:03}", year, ddd),
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(
            f,
            "{:02}:{:02}:{:02}.{}",
            self.hour, self.minute, self.second, self.millisecond,
        )
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `+00:00` or `Z`
        if self.hours == 0 && self.minutes == 0 {
            write!(f, "Z")
        } else {
            write!(f, "{:02}:{:02}", self.hours, self.minutes)
        }
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // like `16:43:16.123+00:00`
        write!(f, "{}T{}", self.date, self.time)?;
        if let Some(offset) = self.offset {
            write!(f, "{}", offset)?;
        }
        Ok(())
    }
}
