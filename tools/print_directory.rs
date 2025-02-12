use std::fs;
use std::path::Path;

// cargo run --bin print_directory
fn print_directory_structure(path: &Path, prefix: &str) -> std::io::Result<()> {
    if path.is_dir() {
        // Print current directory name
        if prefix.is_empty() {
            println!("{}", path.file_name().unwrap().to_string_lossy());
        }

        // Get and sort directory entries
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(|e| e.ok())
            .collect();
        entries.sort_by_key(|e| e.path());

        // Process each entry
        let total = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            let is_last = index == total - 1;
            let entry_path = entry.path();
            let name = entry_path.file_name().unwrap().to_string_lossy();

            // Skip hidden files and target directory
            if name.starts_with('.') || name == "target" {
                continue;
            }

            // Determine and print the appropriate prefix
            let new_prefix = if prefix.is_empty() {
                String::from("├── ")
            } else {
                format!("{}{}",
                    if prefix.contains("└") {
                        prefix.replace("└", " ")
                    } else {
                        prefix.replace("├", "│")
                    },
                    if is_last { "└── " } else { "├── " }
                )
            };

            println!("{}{}", new_prefix, name);

            // Recursively print subdirectories
            if entry_path.is_dir() {
                let next_prefix = if prefix.is_empty() {
                    if is_last { String::from("    ") } else { String::from("│   ") }
                } else {
                    format!("{}{}",
                        if prefix.contains("└") {
                            prefix.replace("└", " ")
                        } else {
                            prefix.replace("├", "│")
                        },
                        if is_last { "    " } else { "│   " }
                    )
                };
                print_directory_structure(&entry_path, &next_prefix)?;
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let current_dir = std::env::current_dir()?;
    print_directory_structure(&current_dir, "")?;
    Ok(())
}