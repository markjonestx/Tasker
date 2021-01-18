use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

// I want to stay compatible with task book which uses camel Case.
#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
pub struct Settings {
    taskbookDirectory: String,
    displayCompleteTasks: bool,
    displayProgressOverview: bool,
}

impl Settings {
    pub fn new(settings_file: String) -> Result<Settings, serde_json::Error> {
        if settings_file.is_empty() {
            Ok(Settings {
                taskbookDirectory: "~/.config/tasker".into(),
                displayCompleteTasks: true,
                displayProgressOverview: true,
            })
        } else {
            serde_json::from_str(&settings_file)
        }
    }

    pub fn show_completed(&self) -> bool {
        self.displayCompleteTasks
    }

    pub fn show_progress(&self) -> bool {
        self.displayProgressOverview
    }
}

impl fmt::Display for Settings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(&self).expect("Failed to process the task list!")
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn settings_string() -> String {
        return "{\n\"taskbookDirectory\": \"~\",\n\"displayCompleteTasks\": true,\
            \"displayProgressOverview\": true\n}"
            .into();
    }

    #[test]
    fn test_parse_settings() {
        let settings = Settings::new(settings_string()).unwrap();
        assert!(settings.show_completed());
        assert!(settings.show_progress());
    }

    #[test]
    fn test_parsed_settings_matches_created() {
        let read_settings = Settings::new(settings_string()).unwrap();
        let replicated_settings = Settings::new(read_settings.to_string()).unwrap();
        assert_eq!(
            read_settings.show_progress(),
            replicated_settings.show_progress()
        );
        assert_eq!(
            read_settings.show_completed(),
            replicated_settings.show_completed()
        );
    }
}
