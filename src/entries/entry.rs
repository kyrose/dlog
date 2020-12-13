use std::fmt::Display;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Duration, Local, NaiveDateTime, Utc, serde::ts_seconds};

use crate::format;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    /// A POSIX time point defining the start of the time entry.
    #[serde(with = "ts_seconds")]
    pub from: DateTime<Utc>,
    /// A POSIX time point defining the end of the time entry.
    #[serde(with = "ts_seconds")]
    pub to: DateTime<Utc>,
    /// The activity committed during this time entry.
    /// Must not be an empty string unless the time entry is incomplete.
    pub activity: String,
    /// The project the activity was committed in.
    /// Can be an empty string to mean that the activity does not belong
    /// to a project.
    pub project: String,
    /// A variable number of tags that belong to this time entry.
    #[serde(with = "vector_format")]
    pub tags: Vec<String>,
    /// An optional entry comment.
    pub comment: String
}

impl Entry {
    /// Creates a new incomplete time entry.
    /// Incomplete time entries only have a start point.
    pub fn new() -> Entry {
        Entry {
            from: Utc::now(),
            to: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            activity: String::new(),
            project: String::new(),
            tags: Vec::new(),
            comment: String::new()
        }
    }

    /// Returns `true` if this time entry is valid.
    pub fn valid(&self) -> bool {
        let now = Utc::now();

        self.from.timestamp() > 0  // from must not be uninitialized
         && (self.to.timestamp() == 0 || self.to >= self.from)  // to must be uninitialized or greater than from
         && self.from <= now  // from must be smaller than now
         && self.to <= now  // to must be smaller than now
    }

    /// Returns `true` if this time entry is complete, that is
    /// if it has both a start and an end point.
    pub fn complete(&self) -> bool {
        self.valid()
         && self.to.timestamp() > 0
         && !self.activity.is_empty()
    }

    /// Returns `true` if the time entry is uninitialized.
    pub fn null(&self) -> bool {
        self.from.timestamp() == 0
         && self.to.timestamp() == 0
         && self.activity.is_empty()
         && self.project.is_empty()
         && self.tags.is_empty()
         && self.comment.is_empty()
    }

    /// Gets `from` in local time.
    pub fn local_from(&self) -> DateTime<Local> {
        self.from.with_timezone(&Local)
    }

    /// Gets `to` in local time.
    pub fn local_to(&self) -> DateTime<Local> {
        self.to.with_timezone(&Local)
    }

    /// Gets the duration of this time entry.
    pub fn duration(&self) -> Duration {
        if !self.complete() || !self.valid() {
            return Duration::zero();
        }

        self.to - self.from
    }

    /// Gets a colorized string representation of this entry
    /// that can be used as part of a table.
    pub fn tabular(&self) -> String {
        let duration = format::duration(&self.duration());

        let activity_project = if self.project.is_empty() {
            self.activity.cyan().to_string()
        } else {
            format!("{}:{}", self.activity.cyan(), self.project.bright_red())
        };

        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            format!("[{}]", self.tags.iter().map(|tag| tag.bright_yellow().to_string()).collect::<Vec<String>>().join(", "))
        };

        let activity_length = if !self.project.is_empty() {
            ACTIVITY_LENGTH
        } else {
            // We have to subtract exactly 9 here for the output
            // to be properly aligned with rows that have empty projects.
            // There is probably a good reason why, but I don't know what it is.
            ACTIVITY_LENGTH - 9
        };

        format!("{} to {} {} {:>width$}  {}",
            self.local_from().time().to_string().magenta(),
            self.local_to().time().to_string().magenta(),
            format!("({})", duration),
            activity_project,
            tags,
            width = activity_length
        )
    }
}

const ACTIVITY_LENGTH: usize = 56;

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = format::duration(&self.duration());

        let activity_project = if self.project.is_empty() {
            self.activity.cyan().to_string()
        } else {
            format!("{}:{}", self.activity.cyan(), self.project.bright_red())
        };

        let tags = if self.tags.is_empty() {
            String::new()
        } else {
            format!(" {}", self.tags.iter().map(|tag| (String::from("+") + tag).bright_yellow().to_string()).collect::<Vec<String>>().join(" "))
        };

        f.write_fmt(format_args!("{}{} from {} to {} {}",
            activity_project,
            tags,
            self.local_from().time().to_string().magenta(),
            self.local_to().time().to_string().magenta(),
            format!("({})", duration)
        ))
    }
}

mod vector_format {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(vector: &Vec<String>, serializer: S) -> Result<S::Ok, S::Error>
      where S: Serializer
    {
        serializer.serialize_str(&vector.join(","))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
      where D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() {
            Ok(Vec::<String>::new())
        } else {
            Ok(s.split(",").map(|s| s.to_string()).collect::<Vec<String>>())
        }
    }
}