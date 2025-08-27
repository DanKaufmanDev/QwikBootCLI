use dialoguer::Select;
use std::process::Command;

pub fn create_html_js_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let files = vec!["README.md", "index.html", "style.css", "script.js"];

    for file in files {
        Command::new("touch")
            .arg(file)
            .current_dir(project_name)
            .output()?;
    }

    Command::new("git")
        .arg("init")
        .current_dir(project_name)
        .output()?;

    println!("Creating HTML + JavaScript project in {}", project_name);
    Ok(())
}

pub fn create_vite_project(project_name: &str, template: &str,) -> Result<(), Box<dyn std::error::Error>> {
    let options = vec!["JavaScript", "TypeScript"];
    let selection = Select::new()
        .with_prompt("Select the type of project")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();
    
    if selection == 0 {
        Command::new("npm")
            .arg("create")
            .arg("vite@latest")
            .arg(&project_name.to_lowercase())
            .arg("--")
            .arg("--template")
            .arg(template)
            .current_dir(project_name)
            .output()?;
    } else {
        Command::new("npm")
            .arg("create")
            .arg("vite@latest")
            .arg(&project_name.to_lowercase())
            .arg("--")
            .arg("--template")
            .arg(format!("{}-ts", template))
            .current_dir(project_name)
            .output()?;
    }
    
    let project_path = format!("{}/{}", project_name, project_name.to_lowercase());
    println!("Creating {} project in {}\n", template, project_path);

    Command::new("npm")
        .arg("install")
        .current_dir(&project_path)
        .output()?;

    println!("Running: npm install in {}\n", project_path);
    println!("To start the development server:");
    println!("  cd {}", project_path);
    println!("  npm run dev");

    Ok(())
}

pub fn create_svelte_kit_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let template_options = vec!["minimal", "library", "demo"];
    let template_selection = Select::new()
        .with_prompt("Select SvelteKit template")
        .items(&template_options)
        .default(0)
        .interact()
        .unwrap();

    let typescript_options = vec!["JavaScript", "TypeScript"];
    let ts_selection = Select::new()
        .with_prompt("Select language")
        .items(&typescript_options)
        .default(0)
        .interact()
        .unwrap();

    let mut binding = Command::new("npx");
    let mut command = binding
        .arg("sv")
        .arg("create")
        .arg(&project_name.to_lowercase())
        .arg("--template")
        .arg(template_options[template_selection])
        .arg("--no-add-ons")
        .arg("--install")
        .arg("npm")
        .current_dir(project_name);
        
        if ts_selection == 1 {
           command = command.arg("--types")
            .arg("ts");
        } else {
            command = command.arg("--no-types");
        };

        command.output()?;

    let project_path = format!("{}/{}", project_name, project_name.to_lowercase());
    
    Command::new("npm")
        .arg("install")
        .current_dir(&project_path)
        .output()?;
    
    println!("Creating SvelteKit project in {}", project_path);
    println!("Template: {}", template_options[template_selection]);
    println!("Language: {}", typescript_options[ts_selection]);
    println!("To start the development server:");
    println!("  cd {}", project_path);
    println!("  npm run dev");
    
    Ok(())
}

// Add other frontend project functions...
pub fn create_next_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let options = vec!["JavaScript", "TypeScript"];
    let selection = Select::new()
        .with_prompt("Select the type of project")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    let template = if selection == 0 { "js" } else { "ts" };

    let tailwind = Select::new()
        .with_prompt("Do you want to use Tailwind CSS?")
        .item("Yes")
        .item("No")
        .default(0)
        .interact()
        .unwrap();

    if tailwind == 1 {
        Command::new("npx")
            .arg("create-next-app@latest")
            .arg(project_name.to_lowercase())
            .arg("--yes")
            .arg(format!("--{}", &template))
            .arg("--eslint")
            .arg("--app")
            .arg("--src-dir")
            .current_dir(project_name)
            .output()?;
        println!("Creating Next.js project in {}", project_name);
        Ok(())
    } else {
        Command::new("npx")
            .arg("create-next-app@latest")
            .arg(project_name.to_lowercase())
            .arg("--yes")
            .arg(format!("--{}", &template))
            .arg("--tailwind")
            .arg("--eslint")
            .arg("--app")
            .arg("--src-dir")
            .current_dir(project_name)
            .output()?;
        println!("Creating Next.js project in {}", project_name);
        Ok(())
    }
}

pub fn create_nuxt_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npx")
        .arg("nuxi@latest")
        .arg("init")
        .arg(&project_name.to_lowercase())
        .current_dir(project_name)
        .output()?;

    println!(
        "Creating Nuxt 3 project in {}/{}",
        project_name,
        project_name.to_lowercase()
    );
    Command::new("npm")
        .arg("install")
        .current_dir(format!("{}/{}", project_name, &project_name.to_lowercase()))
        .output()?;
    println!("To start the development server:");
    println!("  cd {}/{}", project_name, project_name.to_lowercase());
    println!("  npm run dev");

    Ok(())
}

pub fn create_angular_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let style_options = vec!["CSS", "SCSS", "SASS"];
    let style_selection = Select::new()
        .with_prompt("Select the type of style")
        .items(&style_options)
        .default(0)
        .interact()
        .unwrap();

    Command::new("npx")
        .arg("@angular/cli@latest")
        .arg("new")
        .arg(project_name.to_lowercase())
        .arg("--style")
        .arg(&style_options[style_selection].to_lowercase())
        .arg("--routing")
        .arg("--standalone")
        .arg("--package-manager=npm")
        .current_dir(project_name)
        .output()?;
    println!(
        "Creating Angular + {} project in {}",
        &style_options[style_selection], project_name
    );
    Ok(())
}