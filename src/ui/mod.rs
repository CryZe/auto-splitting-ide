mod central_panel;
mod code_editor;
mod divider;
mod left_side_bar;
mod panel;
mod right_side_bar;
mod side_bar;
mod source_panel;
mod toggle;
mod variables;
mod widgets;

use std::fmt;

pub use central_panel::*;
pub use code_editor::*;
pub use divider::*;
pub use left_side_bar::*;
pub use panel::*;
pub use right_side_bar::*;
pub use side_bar::*;
pub use source_panel::*;
pub use toggle::*;
pub use variables::*;
pub use widgets::*;

struct FmtDuration<T>(T);

impl fmt::Display for FmtDuration<time::Duration> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SECONDS_PER_MINUTE: u64 = 60;
        const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;

        let time = self.0;
        let nanoseconds = time.subsec_nanoseconds();
        let total_seconds = time.whole_seconds();
        let (minus, total_seconds, nanoseconds) = if (total_seconds | nanoseconds as i64) < 0 {
            ("-", (-total_seconds) as u64, (-nanoseconds) as u32)
        } else {
            ("", total_seconds as u64, nanoseconds as u32)
        };
        let seconds = (total_seconds % SECONDS_PER_MINUTE) as u8;
        let minutes = ((total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
        let hours = total_seconds / SECONDS_PER_HOUR;
        if hours != 0 {
            write!(
                f,
                "{minus}{hours}:{minutes:02}:{seconds:02}.{nanoseconds:09}"
            )
        } else {
            write!(f, "{minus}{minutes}:{seconds:02}.{nanoseconds:09}")
        }
    }
}

impl fmt::Display for FmtDuration<std::time::Duration> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const SECONDS_PER_MINUTE: u64 = 60;
        const SECONDS_PER_HOUR: u64 = 60 * SECONDS_PER_MINUTE;

        let time = self.0;
        let nanoseconds = time.subsec_nanos();
        let total_seconds = time.as_secs();
        let seconds = (total_seconds % SECONDS_PER_MINUTE) as u8;
        let minutes = ((total_seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE) as u8;
        let hours = total_seconds / SECONDS_PER_HOUR;
        if hours != 0 {
            write!(f, "{hours}:{minutes:02}:{seconds:02}.{nanoseconds:09}")
        } else {
            write!(f, "{minutes}:{seconds:02}.{nanoseconds:09}")
        }
    }
}

struct FmtTime(time::OffsetDateTime);

impl fmt::Display for FmtTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (h, m, s) = self.0.to_hms();
        write!(f, "{:02}:{:02}:{:02}", h, m, s)
    }
}
