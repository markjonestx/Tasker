use std::collections::BTreeMap;

use colored::*;

use super::*;

impl TaskList {
    pub fn board_view(&self) -> String {
        let mut output: String = String::new();
        let mut boards: BTreeMap<String, (u16, u16)> = BTreeMap::new();

        // Create a set of boards connected to the tuple of (completed, total)
        for (_, ref task) in &self.tasks {
            for board_name in &task.get_boards() {
                // Since not everything is a task, not everything can be completed,
                // so we set the value to 1 if complete and 0 for all other cases
                let is_complete: u16 = task.is_complete().into();
                let key_check = boards.clone();
                // Update the tuple or create a tuple if haven't encountered this board yet
                if key_check.contains_key(board_name) {
                    let total_tuple = key_check.get(board_name).unwrap();
                    boards.insert(
                        board_name.clone(),
                        (total_tuple.0 + is_complete, total_tuple.1 + 1),
                    );
                } else {
                    boards.insert(board_name.clone(), (is_complete, 1));
                }
            }
        }

        // Process the default board first
        if boards.contains_key("My Board") {
            let (completed, total) = boards.get("My Board").unwrap();

            let board_name = "My Board".underline();
            let progress = format!("[{}/{}]", completed, total).dimmed();

            output = format!(" {} {}", board_name, progress);

            for (_, task) in &self.tasks {
                if task.get_boards().contains(&"My Board".to_string()) {
                    output = format!("{}\n    {}", output, task);
                }
            }
            output = format!("{}\n", output);
            boards.remove("My Board");
        };

        // Process over the rest of the boards
        for (ref board, (completed, total)) in boards {
            let progress = format!("[{}/{}]", completed, total).dimmed();

            output = format!("{} {} {}", output, board.underline(), progress.dimmed());
            for (_, task) in &self.tasks {
                if task.get_boards().contains(board) {
                    output = format!("{}\n    {}", output, task);
                }
            }
            output = format!("{}\n", output);
        }

        output
    }
}
