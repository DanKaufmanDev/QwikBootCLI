use std::process::Command;

pub fn create_react_native_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npx")
        .arg("create-expo-app@latest")
        .arg("--yes")
        .arg(project_name.to_lowercase())
        .current_dir(project_name)
        .output()?;
    println!("Creating React Native project in {}", project_name);
    Ok(())
}