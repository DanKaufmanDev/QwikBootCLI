use std::{env, fs};
use std::env::consts::OS;
use std::process::Command;

mod project_types;
mod commands;
mod utils;

use project_types::ProjectType;
use utils::{get_project_name, get_project_selection, print_error};

fn init() {
    let input = get_project_name();

    let project_types = vec![
        ProjectType::HtmlJavaScript,
        ProjectType::React,
        ProjectType::ReactNative,
        ProjectType::Electron,
        ProjectType::Angular,
        ProjectType::Vue,
        ProjectType::Svelte,
        ProjectType::SvelteKit,
        ProjectType::Vite,
        ProjectType::NextJS,
        ProjectType::NuxtJS,
        ProjectType::ExpressJS,
        ProjectType::NestJS,
        ProjectType::Remix,
        ProjectType::Fastify,
        ProjectType::Go,
        ProjectType::Python,
        ProjectType::Rust,
        ProjectType::Tauri,
    ];

    let selection = get_project_selection(&project_types);

    if let Err(e) = project_types[selection].create_project(&input) {
        print_error(&format!("creating project - {}", e));
        return;
    }
}

fn update_qwik_boot() -> Result<(), Box<dyn std::error::Error>> {
    const GITHUB_RAW_BASE: &str = "https://raw.githubusercontent.com/DanKaufmanDev/QwikBootCLI/main";
    
    match OS {
        "windows" => {
            println!("Fetching Windows installer...");
            let installer_url = format!("{}/install.ps1", GITHUB_RAW_BASE);
            
            let response = reqwest::blocking::get(&installer_url)?;
            if !response.status().is_success() {
                return Err(format!("Failed to fetch installer: HTTP {}", response.status()).into());
            }
            
            let script_content = response.text()?;
            
            let temp_script = "install_qwikboot.ps1";
            std::fs::write(temp_script, script_content)?;
            
            let output = Command::new("powershell")
                .arg("-ExecutionPolicy")
                .arg("Bypass")
                .arg("-File")
                .arg(temp_script)
                .output()?;
            
            let _ = fs::remove_file(temp_script);
            
            if output.status.success() {
                println!("QwikBoot updated successfully!");
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to execute installer: {}", error).into());
            }
        }
        "macos" | "linux" => {
            println!("Fetching Unix installer...");
            let installer_url = format!("{}/install.sh", GITHUB_RAW_BASE);
            
            let response = reqwest::blocking::get(&installer_url)?;
            if !response.status().is_success() {
                return Err(format!("Failed to fetch installer: HTTP {}", response.status()).into());
            }
            
            let script_content = response.text()?;
            
            let temp_script = "install_qwikboot.sh";
            fs::write(temp_script, script_content)?;
            
            Command::new("chmod")
                .arg("+x")
                .arg(temp_script)
                .output()?;
            
            let output = Command::new(format!("./{}", temp_script))
                .output()?;
            
            let _ = fs::remove_file(temp_script);
            
            if output.status.success() {
                println!("QwikBoot updated successfully!");
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                return Err(format!("Failed to execute installer: {}", error).into());
            }
        }
        _ => {
            return Err(format!("Unsupported operating system: {}", OS).into());
        }
    }
    Ok(())
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        init();
    } else {
        let arg_input = args[1].clone();
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        match arg_input.as_str() {
            "--help" | "-h" => {
                println!("Usage:");
                println!("{}", args[0]);
                println!("{}  --template: <template_name> <project_name>", args[0]);
                println!("\nOptions:");
                println!("  --help | -h: Show the list of available commands");
                println!("\n  --version | -v: Show the version of the program");
                println!("  --update | -u: Update the program to the latest version");
                println!("\n  --template: <template_name> <project_name>");
                println!("  --list | --templates: List all available project templates")
                println!("  --plugins | -p <https://github.com/USERNAME/REPO>: Fetch community made templates");
            }
            "--version" | "-v" => {
                println!("QwikBoot: Version {}", VERSION);
            }
            "--update" | "-u" => {
                if let Err(e) = update_qwik_boot() {
                    print_error(&format!("failed to update QwikBoot: {}", e));
                    return;
                }
                println!("QwikBoot updated successfully!");
            }
            "--template" => {
            if args.len() < 4 {
                println!("Usage: {} --template <project_type> <project_name>", args[0]);
                println!("Example: {} --template react my-app", args[0]);
                return;
            }
            
            let template_name = args[2].as_str();
            let project_name = args[3].as_str();
            
            let project_type = match template_name {
                "html" | "html-js" => ProjectType::HtmlJavaScript,
                "react" => ProjectType::React,
                "vue" => ProjectType::Vue,
                "svelte" => ProjectType::Svelte,
                "sveltekit" => ProjectType::SvelteKit,
                "vite" => ProjectType::Vite,
                "nextjs" | "next" => ProjectType::NextJS,
                "nuxtjs" | "nuxt" => ProjectType::NuxtJS,
                "angular" => ProjectType::Angular,
                "express" | "expressjs" => ProjectType::ExpressJS,
                "nest" | "nestjs" => ProjectType::NestJS,
                "remix" => ProjectType::Remix,
                "fastify" => ProjectType::Fastify,
                "reactnative" | "react-native" => ProjectType::ReactNative,
                "electron" => ProjectType::Electron,
                "tauri" => ProjectType::Tauri,
                "go" => ProjectType::Go,
                "python" => ProjectType::Python,
                "rust" => ProjectType::Rust,
                _ => {
                    println!("Unknown template: {}", template_name);
                    println!("Available templates:");
                    println!("  Frontend: html, react, vue, svelte, sveltekit, vite, nextjs, nuxtjs, angular");
                    println!("  Backend: express, nest, remix, fastify");
                    println!("  Mobile: reactnative");
                    println!("  Desktop: electron, tauri");
                    println!("  Languages: go, python, rust");
                    return;
                }
            };

            if let Err(e) = project_type.create_project(project_name) {
                print_error(&format!("creating project - {}", e));
                return;
            }
            
            println!("Project '{}' created successfully with {} template!", project_name, project_type.display_name());
        }

        "--list" | "--templates" => {
            println!("Available project templates:");
            println!();
            println!("  Frontend:");
            println!("  html, html-js     - HTML + JavaScript");
            println!("  react             - React application");
            println!("  vue               - Vue.js application");
            println!("  svelte            - Svelte application");
            println!("  sveltekit         - SvelteKit framework");
            println!("  vite              - Vanilla Vite project");
            println!("  nextjs, next      - Next.js framework");
            println!("  nuxtjs, nuxt      - Nuxt.js framework");
            println!("  angular           - Angular application");
            println!();
            println!("  Backend:");
            println!("  express, expressjs - Express.js framework");
            println!("  nest, nestjs      - NestJS framework");
            println!("  remix             - Remix framework");
            println!("  fastify           - Fastify framework");
            println!();
            println!("  Mobile:");
            println!("  reactnative       - React Native app");
            println!();
            println!("  Desktop:");
            println!("  electron          - Electron application");
            println!("  tauri             - Tauri application");
            println!();
            println!("  Languages:");
            println!("  go                - Go application");
            println!("  python            - Python project");
            println!("  rust              - Rust application");
            println!();
            println!("Usage: {} --template <template_name> <project_name>", args[0]);
            println!("Example: {} --template react my-app", args[0]);
        }
        "--plugins" | "-p" => {
            if args.len() < 3 {
                println!("Usage: {} --plugins <repo_url>", args[0]);
                println!("Example: {} --plugins https://github.com/owner/repo", args[0]);
                return;
            }
           let repo_url = args[2].as_str();
           if let Err(e) = commands::custom::create_template_from_github(repo_url) {
            print_error(&format!("failed to create project from GitHub template: {}", e));
            return;
           }
        }
            _ => {
                println!("Invalid argument: {}", arg_input);
                println!("  use --help | -h: to show the list of available commands");
            }
        }
    }
}