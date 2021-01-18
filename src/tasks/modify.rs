use colored::*;

use super::*;
use errors;

use std::str;

impl TaskList {
    fn get_tasks_by_ids(&mut self, ids: Vec<&str>) -> Result<Vec<Task>, String> {
        let mut results: Vec<Task> = Vec::new();

        for id in &ids {
            match self.tasks.remove(&id.to_string()) {
                Some(value) => results.push(value),
                None => return Err(errors::no_index(id.to_string())),
            }
        }
        Ok(results)
    }

    fn get_special_ids_from_args(&self, input: Vec<&str>) -> (Vec<String>, String) {
        let mut special_ids: Vec<String> = Vec::new();
        let mut new_sentence: Vec<String> = Vec::new();

        for word in input {
            if word.starts_with('@') {
                special_ids.push(word.into());
            } else {
                new_sentence.push(word.into());
            }
        }

        (special_ids, new_sentence.join(" "))
    }

    fn get_task_id_from_input(&self, input: Vec<&str>) -> Result<(String, Vec<String>), String> {
        let mut special_id: String = String::new();
        let mut new_sentence: Vec<String> = Vec::new();

        for word in input {
            if word.starts_with('@') && special_id.is_empty() {
                special_id = word.into();
            } else if word.starts_with('@') && !special_id.is_empty() {
                return Err(errors::too_many_ids_error());
            } else {
                new_sentence.push(word.into());
            }
        }

        if special_id.is_empty() {
            return Err(errors::no_id_error());
        }

        let id: String = str::replace(special_id.as_str(), "@", "");

        Ok((id, new_sentence))
    }

    pub fn flip_task_flag(&mut self, ids: Vec<&str>, flag: TaskFlag) -> Result<String, String> {
        let mut marked_tasks: Vec<String> = Vec::new();
        let mut unmarked_tasks: Vec<String> = Vec::new();

        let mut found_tasks: Vec<Task> = self.get_tasks_by_ids(ids)?;

        while !found_tasks.is_empty() {
            let mut task = found_tasks.pop().unwrap();
            match task.flip_flag(flag) {
                Some(value) => match value {
                    true => marked_tasks.push(task.get_id().to_string()),
                    false => unmarked_tasks.push(task.get_id().to_string()),
                },
                None => (),
            };
            self.tasks.insert(task.get_id().to_string(), task);
        }

        let string_of_marked = marked_tasks.join(", ").dimmed();
        let string_of_unmarked = unmarked_tasks.join(", ").dimmed();

        let output = if !marked_tasks.is_empty() {
            match flag {
                TaskFlag::Begin => {
                    format!(" {} Started task(s): {}\n", check_mark(), string_of_marked)
                }
                TaskFlag::Check => {
                    format!(" {} Checked task(s): {}\n", check_mark(), string_of_marked)
                }
                TaskFlag::Star => format!(" {} Star task(s): {}\n", check_mark(), string_of_marked),
            }
        } else {
            String::new()
        };

        if !unmarked_tasks.is_empty() {
            match flag {
                TaskFlag::Begin => Ok(format!(
                    "{}\n {} Paused task(s) : {}",
                    output,
                    check_mark(),
                    string_of_unmarked
                )),
                TaskFlag::Check => Ok(format!(
                    "{}\n {} Unchecked task(s): {}",
                    output,
                    check_mark(),
                    string_of_unmarked
                )),
                TaskFlag::Star => Ok(format!(
                    "{}\n {} Unstarred task(s): {}\n",
                    output,
                    check_mark(),
                    string_of_unmarked
                )),
            }
        } else {
            Ok(output)
        }
    }

    pub fn move_tasks_between_lists(
        &mut self,
        other_list: &mut TaskList,
        ids: Option<Vec<&str>>,
        restore: bool,
    ) -> Result<String, String> {
        // If we're not provided any to move, we're just going to
        // assume that we're moving all the completed tasks over.
        let holder = self.tasks.clone();
        let ids = match ids {
            Some(ids) => ids,
            None => {
                let mut to_delete: Vec<&str> = Vec::new();
                for (id, task) in holder.iter() {
                    match task.is_complete() {
                        true => to_delete.push(id),
                        false => (),
                    }
                }
                to_delete
            }
        };

        let mut found_tasks: Vec<Task> = Vec::new();

        for id in &ids {
            match self.tasks.remove(&id.to_string()) {
                Some(value) => found_tasks.push(value),
                None => return Err(errors::no_index(id.to_string())),
            }
        }

        for task in found_tasks.iter_mut() {
            let new_id: u64 = other_list.get_new_id();
            task.set_id(new_id);
            other_list.tasks.insert(new_id.to_string(), task.clone());
        }

        let moved_ids = ids.join(", ").dimmed();

        if restore {
            Ok(format!(" {} Deleted item(s): {}", check_mark(), moved_ids))
        } else {
            Ok(format!(" {} Restore item(s): {}", check_mark(), moved_ids))
        }
    }

    fn get_new_id(&self) -> u64 {
        let mut potential_id: u64 = 0;

        for (id, _) in &self.tasks {
            if !potential_id.to_string().eq(id) {
                return potential_id;
            }
            potential_id += 1;
        }

        potential_id
    }

    pub fn edit(&mut self, input: Vec<&str>) -> Result<String, String> {
        let (id, words): (String, Vec<String>) = self.get_task_id_from_input(input)?;

        let sentence: String = words.join(" ");

        if self.tasks.contains_key(&id) {
            self.tasks.get_mut(&id).unwrap().set_description(sentence);
            Ok(format!(
                " {} Updated description of item: {}",
                check_mark(),
                id.dimmed()
            ))
        } else {
            Err(errors::no_index(id))
        }
    }

    pub fn move_to_board(&mut self, input: Vec<&str>) -> Result<String, String> {
        let (id, words): (String, Vec<String>) = self.get_task_id_from_input(input)?;

        let mut new_boards: Vec<String> = Vec::new();
        for word in words {
            new_boards.push(format!("@{}", word));
        }

        if self.tasks.contains_key(&id) {
            self.tasks.get_mut(&id).unwrap().set_boards(new_boards);
            Ok(format!(
                " {} Updated description of item: {}",
                check_mark(),
                id.dimmed()
            ))
        } else {
            Err(errors::no_index(id))
        }
    }

    pub fn priority(&mut self, input: Vec<&str>) -> Result<String, String> {
        let (id, words): (String, Vec<String>) = self.get_task_id_from_input(input)?;

        if words.len() > 1 || words.is_empty() {
            return Err(errors::invalid_priority());
        }

        let priority: u8 = match str::parse::<u8>(&words[0]) {
            Ok(v) => v,
            Err(_) => return Err(errors::invalid_priority()),
        };

        let priority_text = match priority {
            1 => "normal".green(),
            2 => "medium".yellow(),
            3 => "high".red(),
            _ => return Err(errors::invalid_priority()),
        };

        if self.tasks.contains_key(&id) {
            self.tasks.get_mut(&id).unwrap().set_priority(priority);
            Ok(format!(
                " {} Updated priority of task: {} to {}",
                check_mark(),
                id.dimmed(),
                priority_text
            ))
        } else {
            Err(errors::no_index(id))
        }
    }

    pub fn new_entry(&mut self, input: Vec<&str>, is_note: bool) -> String {
        let (boards, sentence): (Vec<String>, String) = self.get_special_ids_from_args(input);
        let id = self.get_new_id();
        let id_string = id.to_string();

        let new_entry = Task::new(sentence, boards, id, is_note);
        self.tasks.insert(id_string.clone(), new_entry);

        if is_note {
            format!(" {} Created note: {}", check_mark(), id_string.dimmed())
        } else {
            format!(" {} Created task: {}", check_mark(), id_string.dimmed())
        }
    }
}
