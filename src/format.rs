use chrono::{Date, DateTime, Duration, Local, Utc};
use colored::{ColoredString, Colorize};

const FORMAT_DATE_TODAY: &str = "%A, %d %B %Y (today)";
const FORMAT_DATE_YESTERDAY: &str = "%A, %d %B %Y (yesterday)";
const FORMAT_DATE: &str = "%A, %d %B %Y";
const FORMAT_DATETIME_TODAY: &str = "today at %H:%M:%S";
const FORMAT_DATETIME_YESTERDAY: &str = "yesterday at %H:%M:%S";
const FORMAT_DATETIME: &str = "%A, %d %B %Y at %H:%M:%S";
const FORMAT_DATETIME_SHORT: &str = "%Y-%m-%d %H:%M:%S";
const FORMAT_TIME: &str = "%H:%M:%S";

pub fn time(date: &DateTime<Utc>) -> ColoredString {
    date.with_timezone(&Local).format(FORMAT_TIME).to_string().magenta()
}

pub fn datetime_short(date: &DateTime<Utc>) -> ColoredString {
    date.with_timezone(&Local).format(FORMAT_DATETIME_SHORT).to_string().magenta()
}

pub fn date(date: &Date<Utc>) -> ColoredString {
    let today = Local::today();
    let date = date.with_timezone(&Local);

    let delayed_format = if date == today {
        date.format(FORMAT_DATE_TODAY)
    } else if date == (today - Duration::days(1)) {
        date.format(FORMAT_DATE_YESTERDAY)
    } else {
        date.format(FORMAT_DATE)
    };

    delayed_format.to_string().replace(" 0", " ").green()
}

pub fn datetime(datetime: &DateTime<Utc>) -> ColoredString {
    if datetime.timestamp() == 0 {
        return "the beginning of time".magenta();
    }

    let today = Utc::today();
    let datetime = datetime.with_timezone(&Local);

    let delayed_format = if datetime.date() == today {
        datetime.format(FORMAT_DATETIME_TODAY)
    } else if datetime.date() == (today - Duration::days(1)) {
        datetime.format(FORMAT_DATETIME_YESTERDAY)
    } else {
        datetime.format(FORMAT_DATETIME)
    };

    delayed_format.to_string().magenta()
}

pub fn duration(duration: &Duration) -> ColoredString {
    let days = duration.num_days();
    let sign = if duration.num_seconds().is_negative() { "-" } else { "" };

    let string = if days != 0 {
        format!("{}{:0>2}d {:0>2}h {:0>2}m {:0>2}s", sign, days.abs(), duration.num_hours().abs() % 24, duration.num_minutes().abs() % 60, duration.num_seconds().abs() % 60)
    } else {
        format!("{}{:0>2}h {:0>2}m {:0>2}s", sign, duration.num_hours().abs() % 24, duration.num_minutes().abs() % 60, duration.num_seconds().abs() % 60)
    };

    string.bright_magenta()
}
