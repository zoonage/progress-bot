use crate::schema::standups;
use crate::schema::users;
use crate::StandupState;
use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};

#[derive(Debug, Queryable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub channel: Option<String>,
    pub reminder: Option<NaiveDateTime>,
    pub real_name: String,
    pub avatar_url: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub real_name: &'a str,
    pub avatar_url: &'a str,
}

#[derive(Debug, Queryable, AsChangeset)]
pub struct Standup {
    pub id: i32,
    pub username: String,
    pub date: NaiveDateTime,
    pub prev_day: Option<String>,
    pub day: Option<String>,
    pub blocker: Option<String>,
}

impl Standup {
    pub fn get_state(&self) -> StandupState {
        if self.prev_day.is_none() {
            StandupState::PrevDay
        } else if self.day.is_none() {
            StandupState::Today
        } else if self.blocker.is_none() {
            StandupState::Blocker
        } else {
            StandupState::Complete
        }
    }

    pub fn add_content(&mut self, content: &str) {
        match self.get_state() {
            StandupState::PrevDay => self.prev_day = Some(content.to_string()),
            StandupState::Today => self.day = Some(content.to_string()),
            StandupState::Blocker => self.blocker = Some(content.to_string()),
            _ => (),
        }
    }

    pub fn get_copy(&self, channel: &Option<String>) -> String {
        match self.get_state() {
            StandupState::PrevDay => {
                ":two: What are you going to be focusing on *today*?".to_string()
            }
            StandupState::Today => ":three: Any blockers impacting your work?".to_string(),
            StandupState::Blocker => {
                let extra = match channel {
                    None => String::from(""),
                    Some(channel) => format!(
                        "Additionally, I've shared the standup notes to <#{}>.",
                        channel
                    ),
                };

                format!(":white_check_mark: *All done here!* {}\n\n Thank you, have a great day and talk to you {}.",
                    extra, "tomorrow"
                )
            }
            StandupState::Complete => {
                "You're done for today, off to work you go now! :nerd_face:".to_string()
            }
        }
    }
}

#[derive(Insertable)]
#[table_name = "standups"]
pub struct NewStandup {
    pub username: String,
    pub date: NaiveDateTime,
}

impl NewStandup {
    pub fn new(username: &str) -> NewStandup {
        let now = Utc::now();
        let d = NaiveDate::from_ymd(now.year(), now.month(), now.day());
        let t = NaiveTime::from_hms_milli(0, 0, 0, 0);
        let today = NaiveDateTime::new(d, t);

        NewStandup {
            username: username.to_string(),
            date: today,
        }
    }
}