use console::style;
use dialoguer::{Input, Select};

pub fn get_project_name() -> String {
    Input::new()
        .with_prompt("Enter the name of your project")
        .interact()
        .unwrap()
}

pub fn get_project_selection(project_types: &[crate::project_types::ProjectType]) -> usize {
    let styled_options: Vec<String> = project_types.iter().map(|pt| pt.display_name()).collect();

    Select::new()
        .with_prompt("Select the type of project")
        .items(&styled_options)
        .default(0)
        .interact()
        .unwrap()
}

pub fn print_error(message: &str) {
    eprintln!("{}: {}", style("Error").color256(196).to_string(), message);
}