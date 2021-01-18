use serde::{Deserialize, Serialize};

use chrono::Local;
use std::fmt;

use colored::*;

pub fn check_mark() -> ColoredString {
    "✓".green()
}

#[derive(Clone, Copy)]
pub enum TaskFlag {
    Begin,
    Check,
    Star,
}

// These are camelCase because I want compatibility with task book
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    _id: u64,
    _date: String,
    _timestamp: i64,
    description: String,
    isStarred: bool,
    boards: Vec<String>,
    _isTask: bool,
    isComplete: Option<bool>,
    inProgress: Option<bool>,
    priority: Option<u8>,
}

impl Task {
    pub fn new(description: String, mut boards: Vec<String>, id: u64, is_note: bool) -> Task {
        let complete = match is_note {
            true => None,
            false => Some(false),
        };

        let progress = match is_note {
            true => None,
            false => Some(false),
        };

        let priority = match is_note {
            true => None,
            false => Some(1),
        };

        if boards.is_empty() {
            boards.push("My Board".into());
        }

        let timestamp = Local::now().timestamp_millis();
        let date = Local::now().format("%a %b %e %Y").to_string();

        Task {
            _id: id,
            _date: date,
            _timestamp: timestamp,
            description: description,
            isStarred: false,
            boards: boards,
            _isTask: !is_note,
            isComplete: complete,
            inProgress: progress,
            priority: priority,
        }
    }

    pub fn get_boards(&self) -> Vec<String> {
        return self.boards.clone();
    }

    pub fn set_boards(&mut self, boards: Vec<String>) {
        self.boards = boards;
    }

    pub fn get_id(&self) -> u64 {
        return self._id;
    }

    pub fn set_id(&mut self, id: u64) {
        self._id = id;
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn is_note(&self) -> bool {
        return !self._isTask;
    }

    pub fn is_complete(&self) -> bool {
        match self.isComplete {
            Some(value) => value,
            None => false,
        }
    }

    pub fn flip_flag(&mut self, flag: TaskFlag) -> Option<bool> {
        match flag {
            TaskFlag::Begin => {
                match self.inProgress {
                    Some(progress) => match progress {
                        true => self.inProgress = Some(false),
                        false => {
                            self.inProgress = Some(true);
                            self.isComplete = Some(false);
                        }
                    },
                    None => (),
                };
                self.inProgress
            }
            TaskFlag::Check => {
                match self.isComplete {
                    Some(complete) => match complete {
                        true => self.isComplete = Some(false),
                        false => {
                            self.isComplete = Some(true);
                            self.inProgress = Some(false);
                        }
                    },
                    None => (),
                }
                self.isComplete
            }
            TaskFlag::Star => {
                self.isStarred = !self.isStarred;
                Some(self.isStarred)
            }
        }
    }

    pub fn set_priority(&mut self, priority: u8) {
        self.priority = Some(priority);
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut description = match self.priority {
            Some(value) => match value {
                1 => format!("{}", self.description),
                2 => format!("{} (!)", self.description.yellow()),
                3 => format!("{} (!!)", self.description.red()),
                _ => panic!("Task {} has invalid priority", self._id),
            },
            None => format!("{}", self.description),
        };

        let mut status = "☐".cyan();
        match self.inProgress {
            Some(value) => match value {
                true => {
                    status = "∴".blue();
                }
                false => (),
            },
            None => (),
        };

        match self.isComplete {
            Some(value) => match value {
                true => {
                    status = "✓".green();
                    description = format!("{}", self.description.dimmed());
                }
                false => (),
            },
            None => (),
        };

        if self.is_note() {
            status = "●".blue();
        }

        let started;
        if self.isStarred {
            started = "٭".yellow();
        } else {
            started = "".yellow();
        }

        let millisecond_difference = Local::now().timestamp_millis() - self._timestamp;
        let days = millisecond_difference / 86400000;
        let days_since = if days > 0 {
            format!("{}d ", days).dimmed()
        } else {
            "".dimmed()
        };

        // let seconds_since = match SystemTime::now().duration_since(SystemTime::from(self._timestamp)) {
        //     Ok(value) => value.as_secs(),
        //     Err(_) => {
        //         eprintln!("SystemTime has gone backwards!");
        //         0
        //     }
        // };

        // let days_since = ()

        let num = format!("{}.", self._id).dimmed();
        write!(
            f,
            "{} {} {} {}{}",
            num, status, description, days_since, started
        )
    }
}
