use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProjectJson {
    name: String,
    #[serde(default)]
    targets: HashMap<String, Target>,
}

#[derive(Debug, Deserialize)]
struct Target {
    #[serde(default)]
    configurations: HashMap<String, serde_json::Value>,
}

fn main() {
    // Step 1: Find the git root
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let git_root = find_git_root(&current_dir);

    match git_root {
        Some(root) => {
            // Step 2: Recursively search for project.json files and parse them
            if let Err(e) = find_and_parse_project_json_files(&root) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!(
                "Error: No .git directory found. Starting from: {}",
                current_dir.display()
            );
            std::process::exit(1);
        }
    }
}

/// Walk up the directory tree to find the .git directory
fn find_git_root(start_path: &Path) -> Option<PathBuf> {
    let mut current = start_path.to_path_buf();

    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Some(current);
        }

        // Move to parent directory
        match current.parent() {
            Some(parent) => current = parent.to_path_buf(),
            None => return None, // Reached filesystem root
        }
    }
}

/// Recursively search for project.json files and parse them
/// Respects .gitignore patterns
fn find_and_parse_project_json_files(root: &Path) -> std::io::Result<()> {
    let mut walker = ignore::WalkBuilder::new(root);
    walker.git_ignore(true).git_exclude(true);

    for result in walker.build() {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // Check if this is a project.json file
                if let Some(file_name) = path.file_name() {
                    if file_name == "project.json" {
                        parse_and_print_project_json(path);
                    }
                }
            }
            Err(err) => {
                eprintln!("Warning: Error walking directory: {}", err);
            }
        }
    }

    Ok(())
}

/// Parse a project.json file and print project:target:configuration combinations
fn parse_and_print_project_json(path: &Path) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Warning: Failed to read {}: {}", path.display(), e);
            return;
        }
    };

    let project: ProjectJson = match serde_json::from_str(&content) {
        Ok(project) => project,
        Err(e) => {
            eprintln!("Warning: Failed to parse JSON in {}: {}", path.display(), e);
            return;
        }
    };

    let project_name = &project.name;

    // Iterate through all targets
    for (target_name, target) in &project.targets {
        // Check if this target has configurations
        if target.configurations.is_empty() {
            // If no configurations, skip this target (or we could print project:target with empty config)
            continue;
        }

        // Iterate through all configurations for this target
        for configuration_name in target.configurations.keys() {
            println!("{}:{}:{}", project_name, target_name, configuration_name);
        }
    }
}
