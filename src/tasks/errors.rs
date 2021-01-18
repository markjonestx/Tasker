use colored::*;

fn error_mark() -> ColoredString {
    "✖".red()
}

pub fn no_id_error() -> String {
    format!(" {} No id was provided in input", error_mark())
}

pub fn too_many_ids_error() -> String {
    format!(" {} More than one id was given as input", error_mark())
}

pub fn no_index(missing_id: String) -> String {
    format!(
        " {} Unable to find item with id: {}",
        error_mark(),
        missing_id.dimmed()
    )
}

pub fn invalid_priority() -> String {
    format!(" {} Priority can only be 1, 2, or 3", error_mark())
}
