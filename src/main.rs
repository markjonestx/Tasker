use std::{fs, process};

use global_settings::Settings;
use parse::*;
use tasks::{TaskFlag, TaskList};

mod global_settings;
mod parse;
mod tasks;

use clap::{arg, App, ArgMatches};


fn main() {
    let args: ArgMatches = parse_args();

    parse::create_config_dir();
    let settings: Settings = load_settings_file(get_settings_location());
    let mut task_list: TaskList = load_task_list_file(get_task_list_location());
    let mut archive: TaskList = load_task_list_file(get_archive_location());

    let output = run_program(&settings, &mut task_list, &mut archive, args);
    fs::write(get_settings_location(), settings.to_string()).expect("Failed to write settings!");
    fs::write(get_task_list_location(), task_list.to_string()).expect("Failed to write task list!");
    fs::write(get_archive_location(), archive.to_string()).expect("Failed to write archive!");
    print!("{}\n", output);
}


fn parse_args() -> ArgMatches {
    App::new("Tasker")
        .version("0.1")
        .about("a rust clone of Taskbook")
        .override_usage("$ ts [<options> ...]")
        .arg(arg!(ARCHIVE: -a --archive "Display archived items"))
        .arg(arg!(BEGIN: -b --begin "Start/pause task")
            .takes_value(true)
            .multiple_occurrences(true))
        .arg(arg!(CHECK: -c --check "Check/uncheck task")
            .takes_value(true))
        .arg(arg!(CLEAR: --clear "Delete all checked items"))
        .arg(arg!(COPY: -y --copy "Copy item description")
            .takes_value(true))
        .arg(arg!(DELETE: -d --delete "Delete item")
            .takes_value(true))
        .arg(arg!(EDIT: -e --edit "Edit item description")
            .takes_value(true))
        .arg(arg!(FIND: -f --find "Search for items")
            .takes_value(true))
        .arg(arg!(LIST: -l --list "List items by attributes")
            .takes_value(true))
        .arg(arg!(MOVE: -m --move "Move item between boards")
            .takes_value(true))
        .arg(arg!(NOTE: -n --note "Create note")
            .takes_value(true)
            .multiple_occurrences(true))
        .arg(arg!(PRIORITY: -p --priority "Update priority of task")
            .takes_value(true))
        .arg(arg!(RESTORE: -r --restore "Restore items from archive")
            .takes_value(true))
        .arg(arg!(STAR: -s --star "Star/unstar item")
            .takes_value(true))
        .arg(arg!(TASK: -t --task "Create task")
            .takes_value(true))
        .arg(arg!(TIMELINE: -i --timeline "Display timeline view"))
        .after_help("EXAMPLES:
    $ ts
    $ ts --archive
    $ ts --begin 2 3
    $ ts --check 1 2
    $ ts --clear
    $ ts --copy 1 2 3
    $ ts --delete 4
    $ ts --edit @3 Merge PR #42
    $ ts --find documentation
    $ ts --list pending coding
    $ ts --move @1 cooking
    $ ts --note @coding Actually learn rust
    $ ts --priority @3 2
    $ ts --restore 4
    $ ts --star 2
    $ ts --task @coding @issues Patch issue 32
    $ ts --task @coding Finish something for once
    $ ts --task Make some buttercream
    $ ts --timeline")
        .get_matches()
}

/// # Parses the arguments for the program
// fn parse_args() -> ArgMatches {
//     clap_app!(tasker =>
//         (version: "0.1")
//         (about: "a rust clone of Taskbook")
//         (override_usage: "$ ts [<options> ...]")
//         (@arg ARCHIVE: -a --archive "Display archived items")
//         (@arg BEGIN: -b --begin +takes_value +multiple "Start/pause task")
//         (@arg CHECK: -c --check +takes_value "Check/uncheck task")
//         (@arg CLEAR: --clear "Delete all checked items")
//         (@arg COPY: -y --copy +takes_value "Copy item description")
//         (@arg DELETE: -d --delete +takes_value "Delete item")
//         (@arg EDIT: -e --edit +takes_value "Edit item description")
//         (@arg FIND: -f --find +takes_value "Search for items")
//         (@arg LIST: -l --list +takes_value "List items by attributes")
//         (@arg MOVE: -m --move +takes_value "Move item between boards")
//         (@arg NOTE: -n --note +takes_value +multiple "Create note")
//         (@arg PRIORITY: -p --priority +takes_value "Update priority of task")
//         (@arg RESTORE: -r --restore +takes_value "Restore items from archive")
//         (@arg STAR: -s --start +takes_value "Star/unstar item")
//         (@arg TASK: -t --task +takes_value "Create task")
//         (@arg TIMELINE: -i --timeline "Display timeline view")
//         (after_help: "EXAMPLES:
    // $ ts
    // $ ts --archive
    // $ ts --begin 2 3
    // $ ts --check 1 2
    // $ ts --clear
    // $ ts --copy 1 2 3
    // $ ts --delete 4
    // $ ts --edit @3 Merge PR #42
    // $ ts --find documentation
    // $ ts --list pending coding
    // $ ts --move @1 cooking
    // $ ts --note @coding Actually learn rust
    // $ ts --priority @3 2
    // $ ts --restore 4
    // $ ts --star 2
    // $ ts --task @coding @issues Patch issue 32
    // $ ts --task @coding Finish something for once
    // $ ts --task Make some buttercream
    // $ ts --timeline")
    // )
//     .get_matches()
// }

fn run_program(
    settings: &Settings,
    task_list: &mut TaskList,
    archive: &mut TaskList,
    args: ArgMatches,
) -> String {
    // Error handling
    // BEGIN, CHECK, DELETE, EDIT, MOVE, PRIORITY, RESTORE, STAR
    if let Some(begin) = args.values_of("BEGIN") {
        process_error(task_list.flip_task_flag(begin.collect(), TaskFlag::Begin))
    } else if let Some(check) = args.values_of("CHECK") {
        process_error(task_list.flip_task_flag(check.collect(), TaskFlag::Check))
    } else if let Some(delete) = args.values_of("DELETE") {
        process_error(task_list.move_tasks_between_lists(archive, Some(delete.collect()), false))
    } else if let Some(edit) = args.values_of("EDIT") {
        process_error(task_list.edit(edit.collect()))
    } else if let Some(move_list) = args.values_of("MOVE") {
        process_error(task_list.move_to_board(move_list.collect()))
    } else if let Some(priority) = args.values_of("PRIORITY") {
        process_error(task_list.priority(priority.collect()))
    } else if let Some(restore) = args.values_of("RESTORE") {
        process_error(archive.move_tasks_between_lists(task_list, Some(restore.collect()), true))
    } else if let Some(star) = args.values_of("STAR") {
        process_error(task_list.flip_task_flag(star.collect(), TaskFlag::Star))
    }
    // Regular output
    // FIND, LIST, NOTE, TASK
    // else if args.is_present("FIND") {

    // } else if args.is_present("LIST") {

    // }
    else if let Some(note) = args.values_of("NOTE") {
        task_list.new_entry(note.collect(), true)
    } else if let Some(task) = args.values_of("TASK") {
        task_list.new_entry(task.collect(), false)
    }
    // Special output
    // ARCHIVE, CLEAR, COPY, TIMELINE, none
    else if args.is_present("ARCHIVE") {
        archive.board_view()
    } else if args.is_present("CLEAR") {
        task_list
            .move_tasks_between_lists(archive, None, false)
            .expect("Failed to clear the completed tasks!")
    } else if args.is_present("COPY") {
        "Clipboard isn't supported yet.".into()
    } else if args.is_present("TIMELINE") {
        task_list.board_view()
    } else {
        match settings.show_completed() {
            true => task_list.board_view(),
            false => task_list.board_view(),
        }
    }
}

fn process_error(result: Result<String, String>) -> String {
    match result {
        Ok(value) => value,
        Err(err) => {
            print!("{}", err);
            process::exit(1);
        }
    }
}
