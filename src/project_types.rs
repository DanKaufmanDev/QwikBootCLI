use console::style;
use dialoguer::Select;
use std::path::Path;
use std::process::Command;

use crate::commands::frontend::*;
use crate::commands::backend::*;
use crate::commands::mobile::*;
use crate::commands::desktop::*;
use crate::commands::languages::*;

#[derive(Debug, Clone)]
pub enum ProjectType {
    HtmlJavaScript,
    React,
    ReactNative,
    Electron,
    Angular,
    Vue,
    Svelte,
    SvelteKit,
    Vite,
    NextJS,
    NuxtJS,
    ExpressJS,
    NestJS,
    Remix,
    Fastify,
    Go,
    Python,
    Rust,
    Tauri,
}

impl ProjectType {
    pub fn display_name(&self) -> String {
        match self {
            ProjectType::HtmlJavaScript => {
                style("HTML").color256(208).to_string() + " + " + &style("JavaScript").color256(185).to_string()
            }
            ProjectType::React => style("React").color256(80).to_string(),
            ProjectType::ReactNative => style("React Native").color256(80).to_string(),
            ProjectType::Electron => style("Electron").color256(6).to_string(),
            ProjectType::Angular => style("Angular").color256(196).to_string(),
            ProjectType::Vue => style("Vue").color256(35).to_string(),
            ProjectType::Svelte => style("Svelte").color256(202).to_string(),
            ProjectType::SvelteKit => {style("Svelte").color256(202).to_string() + &style("Kit").color256(225).to_string()},
            ProjectType::Vite => {style("V").color256(69).to_string() + &style("it").color256(191).to_string() + &style("e").color256(171).to_string()},
            ProjectType::NextJS => style("Next").color256(0).to_string(),
            ProjectType::NuxtJS => style("Nuxt").color256(42).to_string(),
            ProjectType::ExpressJS => style("Express").color256(249).to_string(),
            ProjectType::NestJS => style("Nest").color256(124).to_string(),
            ProjectType::Remix => style("Remix").color256(225).to_string(),
            ProjectType::Fastify => style("Fastify").color256(225).to_string(),
            ProjectType::Go => style("Go").color256(6).to_string(),
            ProjectType::Python => {style("Pyt").color256(27).to_string() + &style("hon").color256(191).to_string()},
            ProjectType::Rust => style("Rust").color256(208).to_string(),
            ProjectType::Tauri => style("Tauri").color256(225).to_string(),
        }
    }

    pub fn check_project_exists(project_name: &str) -> bool {
        Path::new(project_name).exists()
    }

    pub fn create_project(&self, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        if Self::check_project_exists(project_name) {
            println!(
                "{}: project already exists in the current directory",
                &project_name
            );
            let selection = Select::new()
                .with_prompt("Do you want to overwrite the project?")
                .item("No")
                .item("Yes")
                .default(0)
                .interact()
                .unwrap();

            if selection == 0 {
                println!("{}", style("Exiting...").color256(196).bold());
                return Ok(());
            } else {
                Command::new("rm").arg("-rf").arg(project_name).output()?;
            }
        }

        Command::new("mkdir").arg(project_name).output()?;

        match self {
            ProjectType::HtmlJavaScript => create_html_js_project(project_name),
            ProjectType::React => create_vite_project(project_name, "react"),
            ProjectType::Vue => create_vite_project(project_name, "vue"),
            ProjectType::Svelte => create_vite_project(project_name, "svelte"),
            ProjectType::SvelteKit => create_svelte_kit_project(project_name),
            ProjectType::Vite => create_vite_project(project_name, "vanilla"),
            ProjectType::NextJS => create_next_project(project_name),
            ProjectType::NuxtJS => create_nuxt_project(project_name),
            ProjectType::Angular => create_angular_project(project_name),
            
            ProjectType::ExpressJS => create_express_project(project_name),
            ProjectType::NestJS => create_nest_project(project_name),
            ProjectType::Remix => create_remix_project(project_name),
            ProjectType::Fastify => create_fastify_project(project_name),
            
            ProjectType::ReactNative => create_react_native_project(project_name),
            
            ProjectType::Electron => create_electron_project(project_name),
            ProjectType::Tauri => create_tauri_project(project_name),
            
            ProjectType::Go => create_go_project(project_name),
            ProjectType::Python => create_python_project(project_name),
            ProjectType::Rust => create_rust_project(project_name),
        }
    }
}