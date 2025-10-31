use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    // Step 1: Find the git root
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let git_root = find_git_root(&current_dir);
    
    match git_root {
        Some(root) => {
            println!("Found git root: {}", root.display());
            
            // Step 2: Recursively search for project.json files
            find_project_json_files(&root);
        }
        None => {
            eprintln!("Error: No .git directory found. Starting from: {}", current_dir.display());
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

/// Recursively search for project.json files starting from the given directory
fn find_project_json_files(root: &Path) {
    if let Err(e) = walk_directory(root, root) {
        eprintln!("Error walking directory: {}", e);
        std::process::exit(1);
    }
}

/// Recursively walk a directory and print paths to project.json files
fn walk_directory(current: &Path, root: &Path) -> std::io::Result<()> {
    // Check if current directory is readable
    let entries = fs::read_dir(current)?;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Check if it's a directory
        if path.is_dir() {
            // Skip .git directory and other hidden directories that start with .
            if let Some(name) = path.file_name() {
                if name.to_string_lossy().starts_with('.') {
                    continue;
                }
            }
            
            // Recursively walk subdirectories
            walk_directory(&path, root)?;
        }
        
        // Check if the current entry is a project.json file
        if let Some(file_name) = path.file_name() {
            if file_name == "project.json" {
                // Print the path relative to the root
                if let Ok(relative_path) = path.strip_prefix(root) {
                    println!("{}", relative_path.display());
                } else {
                    println!("{}", path.display());
                }
            }
        }
    }
    
    Ok(())
}
