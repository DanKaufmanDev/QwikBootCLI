use std::env;

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

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 1 {
        init();
    } else {
        let arg_input = args[1].clone();
        const VERSION: &str = "0.1.0-beta";
        match arg_input.as_str() {
            "--help" | "-h" => {
                println!("Usage: {} <project_name>", args[0]);
                println!("Options:");
                println!("  --version: Show the version of the program");
            }
            "--version" | "-v" => {
                println!("QwikBoot: Version {}", VERSION);
            }
            _ => {
                println!("Invalid argument: {}", arg_input);
                println!("  --help: Show this help message");
            }
        }
    }
}