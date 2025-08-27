use std::process::Command;
use dialoguer::Select;

pub fn create_electron_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npx")
        .arg("create-electron-app@latest")
        .arg(project_name.to_lowercase())
        .current_dir(project_name)
        .output()?;
    println!("Creating Electron project in {}", project_name);
    Ok(())
}

pub fn create_tauri_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template_options = vec![
        "vanilla", "vanilla-ts", "vue", "vue-ts", "react", "react-ts",
        "svelte", "svelte-ts", "solid", "solid-ts", "preact", "preact-ts",
        "lit", "lit-ts", "yew", "leptos", "sycamore", "gtk-rs"
    ];
    
    let template_selection = Select::new()
        .with_prompt("Select Tauri template")
        .items(&template_options)
        .default(0)
        .interact()
        .unwrap();

    let package_manager_options = vec!["npm", "yarn", "pnpm", "bun"];
    let package_manager_selection = Select::new()
        .with_prompt("Select package manager")
        .items(&package_manager_options)
        .default(0)
        .interact()
        .unwrap();

    let mut command = Command::new("npx");
    command
        .arg("create-tauri-app@latest")
        .arg(&project_name.to_lowercase())
        .arg("--template")
        .arg(template_options[template_selection])
        .arg("--package-manager")
        .arg(package_manager_options[package_manager_selection])
        .arg("--yes")
        .current_dir(project_name);

    command.output()?;

    Command::new("npm")
        .arg("install")
        .current_dir(format!("{}/{}", project_name, project_name.to_lowercase()))
        .output()?;

    println!("Creating Tauri project in {}/{}", project_name, project_name.to_lowercase());
    println!("Template: {}", template_options[template_selection]);
    println!("Package Manager: {}", package_manager_options[package_manager_selection]);
    println!("To run the development server:");
    println!("  cd {}/{}\n", project_name, project_name.to_lowercase());
    println!("  For Desktop development, run:");
    println!("  npm run tauri dev\n");
    println!("  For Android development, run:");
    println!("  npm run tauri android init");
    println!("  npm run tauri android dev\n");
    println!("  For iOS development, run:");
    println!("  npm run tauri ios init");
    println!("  npm run tauri ios dev\n");
    
    Ok(())
}