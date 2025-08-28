use std::process::Command;

pub fn create_template_from_github(repo_url: &str)  -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching template from {}", repo_url);
    let output = Command::new("git")
        .arg("clone")
        .arg(repo_url)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to clone repository: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    Ok(())
}