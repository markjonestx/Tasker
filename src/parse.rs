use super::Settings;
use super::TaskList;

use std::{fs, io, path, process};

use dirs;

/// # Creates the config directory
/// This doesn't attempt to do any _real_ error handling. If the directory doesn't exist it
/// makes the directory, if it does it doesn't do anything. If any other error is raised though
/// it'll panic and crash since it's something we probably can't recover from.
pub fn create_config_dir() {
    let directory: path::PathBuf = get_base_location();

    match fs::create_dir(&directory) {
        Ok(_) => return,
        Err(error) => match error.kind() {
            io::ErrorKind::AlreadyExists => return,
            _ => {
                eprintln!(
                    "Couldn't create base directory at {}, unknown error: {}",
                    directory.to_str().unwrap(),
                    error
                );
                process::exit(1);
            }
        },
    }
}

/// # Searches for the base directory for the program
/// Will return a config directory, or the base of the home directory. If neither can be
/// found though, we abort the program since we can't store data anywhere anyway.
pub fn get_base_location() -> path::PathBuf {
    let mut base_dir = match dirs::config_dir() {
        Some(file_path) => file_path,
        None => match dirs::home_dir() {
            Some(file_path) => file_path,
            None => panic!("Couldn't find a path to write to!",),
        },
    };

    base_dir.push("tasker");
    base_dir
}

/// # Get the path to the settings file
/// This will return the path to where they settings should be stored. This will should work
/// on all platforms, and requires create_config_dir to be called first
pub fn get_settings_location() -> path::PathBuf {
    let mut config: path::PathBuf = get_base_location();
    config.push("settings");
    config.set_extension("json");
    config
}

/// # Creates the Settings struct from the settings file
/// This will load in the settings from the file, handle any errors, and crash if it can't
/// be fixed.
pub fn load_settings_file(path_to_settings: path::PathBuf) -> Settings {
    // Parse the settings file
    let settings_contents = match fs::read_to_string(&path_to_settings) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::new(),
            _ => {
                eprintln!(
                    "Failed to open the settings file at {}!",
                    path_to_settings.to_str().unwrap()
                );
                process::exit(1);
            }
        },
    };

    // Load the global settings
    match Settings::new(settings_contents) {
        Ok(parsed_settings) => parsed_settings,
        Err(_) => handle_broken_settings(path_to_settings),
    }
}

/// # Asks the user about broken settings
/// The settings file isn't complicated. It contains 2 booleans that can easily be replaced
/// or set again, so if the file is damaged in some way shape or form we really won't lose
/// anything by replacing the file.
///
/// So what this does is it asks the user if they would like for the settings to go ahead and
/// be cleared. If they do, we replace it, if not we'll go ahead and just crash as normal.
pub fn handle_broken_settings(path_to_settings: path::PathBuf) -> Settings {
    println!("Settings file is damaged, replace with defaults? [Y/n]");

    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user's input!");

    let overwrite = match user_input.to_lowercase().chars().nth(0).unwrap() {
        'y' => true,
        _ => false,
    };

    if overwrite {
        eprintln!(
            "{} is damaged! Please fix before calling tasker again",
            path_to_settings.to_str().unwrap()
        );
        process::exit(1);
    }

    Settings::new("".into()).unwrap()
}

/// # Get the path to the task list
/// This will return the path to where they task list should be stored. This will should work
/// on all platforms, and requires create_config_dir to be called first
pub fn get_task_list_location() -> path::PathBuf {
    let mut config: path::PathBuf = get_base_location();
    config.push("storage");
    config.set_extension("json");
    config
}

/// # Get the path to the task list
/// This will return the path to where they task list should be stored. This will should work
/// on all platforms, and requires create_config_dir to be called first
pub fn get_archive_location() -> path::PathBuf {
    let mut config: path::PathBuf = get_base_location();
    config.push("archive");
    config.set_extension("json");
    config
}

/// # Creates the task list from
/// Loads in the data from the
pub fn load_task_list_file(path_to_task_list: path::PathBuf) -> TaskList {
    // Parse the task storage file
    let task_contents = match fs::read_to_string(path_to_task_list) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::new(),
            _ => {
                eprintln!("Failed to open the tasklist! Error {:?}", error);
                process::exit(1);
            }
        },
    };

    // Load the task list
    match TaskList::new(task_contents) {
        Ok(tasks) => tasks,
        Err(error) => {
            eprintln!("Failed to parse the tasklist! Error {:?}", error);
            process::exit(1);
        }
    }
}
