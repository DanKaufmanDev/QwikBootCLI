use dialoguer::Select;
use std::process::Command;
use std::fs;

pub fn create_express_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npm")
        .arg("init")
        .arg("-y")
        .arg(project_name.to_lowercase())
        .current_dir(project_name)
        .output()?;

    Command::new("npm")
        .arg("install")
        .arg("express")
        .arg("cors")
        .arg("dotenv")
        .arg("helmet")
        .current_dir(project_name)
        .output()?;

    Command::new("npm")
        .arg("install")
        .arg("--save-dev")
        .arg("nodemon")
        .arg("@types/node")
        .arg("@types/express")
        .current_dir(project_name)
        .output()?;

    Command::new("mkdir")
        .arg("src")
        .current_dir(project_name)
        .output()?;

    fs::write(
        format!("{}/src/index.js", project_name),
        "import express from 'express';\nimport dotenv from 'dotenv';\n\ndotenv.config();\n\nconst app = express();\nconst PORT = process.env.PORT || 3000;\n\napp.get('/', (req, res) => {\n    res.json({ message: 'Hello World' });\n});\n\napp.listen(PORT, () => {\n    console.log(`Server is running on port ${PORT}`);\n});"
    )?;
    
    fs::write(format!("{}/.env", project_name), "PORT=3000\n")?;

    println!("Creating Express project in {}", project_name);
    Ok(())
}

pub fn create_nest_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("npx")
        .arg("@nestjs/cli@latest")
        .arg("new")
        .arg(&project_name.to_lowercase())
        .arg("--package-manager=npm")
        .current_dir(project_name)
        .output()?;
    println!("Creating NestJS project in {}", project_name);
    Ok(())
}

pub fn create_remix_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let language_options = vec!["JavaScript", "TypeScript"];
    let language_selection = Select::new()
        .with_prompt("JavaScript or TypeScript?")
        .items(&language_options)
        .default(0)
        .interact()
        .unwrap();

    Command::new("npx")
        .arg("create-react-router@latest")
        .arg(&project_name.to_lowercase())
        .arg("-y")
        .arg("--template")
        .arg(if language_selection == 1 { "remix-run/react-router-templates/default" } else { "remix-run/react-router-templates/javascript" })
        .current_dir(project_name)
        .output()?;

    println!("Creating Remix project in {}", project_name);
    Ok(())
}

pub fn create_fastify_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let type_selection = Select::new()
        .with_prompt("JavaScript or TypeScript?")
        .item("JavaScript")
        .item("TypeScript")
        .default(0)
        .interact()
        .unwrap();

    if type_selection == 0 {
        Command::new("npx")
            .arg("fastify-cli@latest")
            .arg("generate")
            .arg(&project_name.to_lowercase())
            .arg("--yes")
            .arg("--package-manager=npm")
            .current_dir(project_name)
            .output()?;
    } else {
        Command::new("npx")
            .arg("fastify-cli@latest")
            .arg("generate")
            .arg(&project_name.to_lowercase())
            .arg("--yes")
            .arg("--package-manager=npm")
            .arg("--lang=ts")
            .current_dir(project_name)
            .output()?;
    }
    println!("Creating Fastify project in {}", project_name);
    Ok(())
}