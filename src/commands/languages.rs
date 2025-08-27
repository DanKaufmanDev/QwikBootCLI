use std::process::Command;
use std::fs;

pub fn create_go_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("go")
        .arg("mod")
        .arg("init")
        .arg(&project_name.to_lowercase())
        .current_dir(project_name)
        .output()?;

    let main_content = r#"package main

import (
    "fmt"
    "log"
    "net/http"
)

func main() {
    http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "Hello, World!")
    })

    fmt.Println("Server starting on port 8080...")
    log.Fatal(http.ListenAndServe(":8080", nil))
}"#;
    
    fs::write(format!("{}/main.go", project_name), main_content)?;

    let readme_content = format!(
        r#"# {}

A Go web server project.

## Run the server

```bash
go run main.go
```

The server will start on http://localhost:8080
"#,
        project_name
    );
    
    fs::write(format!("{}/README.md", project_name), readme_content)?;

    println!("Creating Go project in {}", project_name);
    println!("To run the server:");
    println!("  cd {}", project_name);
    println!("  go run main.go");

    Ok(())
}

pub fn create_python_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("uv")
        .arg("init")
        .current_dir(project_name)
        .output()?;
    println!("Creating Python project in {}", project_name);
    Ok(())
}

pub fn create_rust_project(project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("cargo")
        .arg("init")
        .current_dir(project_name)
        .output()?;
    println!("Creating Rust project in {}", project_name);
    Ok(())
}