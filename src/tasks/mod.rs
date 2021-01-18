pub use task::TaskFlag;
use task::*;

pub use modify::*;
pub use view::*;

use std::collections::BTreeMap;
use std::fmt;

use serde::{Deserialize, Serialize};

mod errors;
mod modify;
mod task;
mod view;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskList {
    tasks: BTreeMap<String, Task>,
}

impl TaskList {
    pub fn new(task_json: String) -> Result<TaskList, serde_json::Error> {
        if task_json.is_empty() {
            Ok(TaskList {
                tasks: BTreeMap::new(),
            })
        } else {
            match serde_json::from_str(&task_json) {
                Ok(parsed) => Ok(TaskList { tasks: parsed }),
                Err(error) => Err(error),
            }
        }
    }
}

impl fmt::Display for TaskList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretty =
            serde_json::to_string_pretty(&self.tasks).expect("Failed to process the task list!");
        write!(f, "{}", pretty)
    }
}
