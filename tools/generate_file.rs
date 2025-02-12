use std::fs;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::env;
use std::error::Error;
use std::collections::HashSet;

// cargo run --bin generate_file --exclude "directory"
/*
cargo run --bin generate_file
cargo run --bin generate_file /path/to/your/rustgl
cargo run --bin generate_file /path/to/your/rustgl --exclude examples,assets
*/
// Default ignored file extensions (same as before)
static DEFAULT_IGNORED_EXTENSIONS: &[&str] = &[
    "lock", "gitignore", "gitattributes", "md", "txt", "toml", "json", "yaml", "yml", "LICENSE",
    "png", "jpg", "jpeg", "svg", "ico", "bmp", "gif", "woff", "woff2", "ttf", "eot", "otf",
    "dll", "exe", "o", "obj", "lib", "a", "so", "dylib", "pdb", "exp", "manifest", "idb", "ipdb",
    "ncb", "opt", "pgd", "pgi", "rc", "res", "rs.bk", "rs.orig", "sbr", "tlb", "tli", "ilk", "log",
    "bak", "tmp", "swp", "swo", "DS_Store", "vscode", "idea", "project", "ipr", "iws", "iml",
    "UserRootDir", "in", "out", "orig", "rej", "patch", "~", "cache", "idx", "part", "crdownload",
    "thumbnails", "parcel-cache", "npm-debug.log", "yarn-error.log", "pnpm-debug.log", ".pnp.cjs",
    ".pnp.npm", "cargo-metadata.json", "Cargo.lock"
];

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Determine the project root directory
    let root_dir = match env::args().nth(1) {
        Some(dir) => PathBuf::from(dir),
        None => env::current_dir()?,
    };

    // 2. Construct the path to the 'src' directory
    let src_dir = root_dir.join("src");

    // 3. Check if 'src' directory exists
    if !src_dir.exists() || !src_dir.is_dir() {
        eprintln!("Error: 'src' directory not found in '{}'", root_dir.display());
        return Ok(());
    }

    // 4. Parse excluded folders argument
    let mut excluded_folders: HashSet<String> = HashSet::new();
    let mut args = env::args().skip(1);

    if env::args().nth(1).is_some() && !env::args().nth(1).unwrap().starts_with("--") {
        args.next(); // Skip root directory arg if provided
    }

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--exclude" => {
                if let Some(folders_str) = args.next() {
                    excluded_folders = folders_str.split(',').map(|s| s.to_string()).collect();
                } else {
                    eprintln!("Error: --exclude option requires a comma-separated list of folder names.");
                    return Ok(());
                }
            }
            _ => {
                eprintln!("Warning: Unknown option '{}'", arg);
            }
        }
    }

    // 5. Create the output file
    let output_file_name = "project_contents.txt";
    let output_path = Path::new(output_file_name);
    let mut output_file = fs::File::create(output_path)?;

    println!("Processing directory: {}", src_dir.display());
    println!("Excluding folders (from root): {:?}", excluded_folders);
    println!("Output will be written to: {}", output_path.display());

    // 6. Traverse and process directory, starting from 'src'
    process_directory(&src_dir, &root_dir, &src_dir, &mut output_file, &excluded_folders)?; // Pass src_dir as src_root

    println!("Successfully wrote project contents to {}", output_path.display());
    Ok(())
}

fn process_directory(dir: &PathBuf, root_dir: &PathBuf, src_root_dir: &PathBuf, output_file: &mut fs::File, excluded_folders: &HashSet<String>) -> Result<(), Box<dyn Error>> {
    for entry_result in fs::read_dir(dir)? {
        let entry = entry_result?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        if file_name_str.starts_with(".") {
            continue;
        }

        if path.is_dir() {
            let relative_path_from_root = path.strip_prefix(root_dir).unwrap_or(&path);
            let folder_name_from_root = relative_path_from_root.components().next().and_then(|comp| comp.as_os_str().to_str());

            if let Some(folder_name) = folder_name_from_root {
                 if excluded_folders.contains(folder_name) {
                    println!("Skipping excluded directory (from root): {}", path.display());
                    continue;
                }
            }
            process_directory(&path, root_dir, src_root_dir, output_file, excluded_folders)?; // Pass src_root_dir along
        } else if path.is_file() {
            if file_name_str.starts_with(".") {
                continue;
            }
            if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
                process_file(&path, src_root_dir, &path, output_file)?; // Use src_root_dir for process_file
            } else {
                if let Some(ext) = path.extension().and_then(|ext| ext.to_str()) {
                    if DEFAULT_IGNORED_EXTENSIONS.contains(&ext) {
                        continue;
                    }
                }
            }
        }
    }
    Ok(())
}

fn process_file(file_path: &PathBuf, src_root_dir: &PathBuf, absolute_file_path: &PathBuf, output_file: &mut fs::File) -> Result<(), Box<dyn Error>> {
    // Calculate relative path from src_dir now
    let relative_path = file_path.strip_prefix(src_root_dir)?;
    let content = fs::read_to_string(absolute_file_path)?;

    writeln!(output_file, "--- File: {} ---", relative_path.display())?; // Relative to src
    writeln!(output_file, "{}", content)?;
    writeln!(output_file, "---\n")?;

    Ok(())
}