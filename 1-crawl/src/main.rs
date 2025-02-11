use std::env;
use std::fs;
use std::io;
use std::path::Path;

/// Recursively crawls the directory at the given path and prints the paths of entries.
///
/// # Arguments
///
/// * `path` - A reference to a `Path` that represents the directory to crawl.
/// e.g. cargo run /path/to/directory

fn crawl(path: &Path) -> io::Result<()> {
    // Read the directory entries
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        println!("Entry path: {}", entry.path().display());
        let path = entry.path();
        if path.is_dir() {
            // Recursively crawl subdirectories
            //crawl(&path)?;
        } else if path.is_file() {
            println!("File path: {}", path.display());
        } else {
            println!("Unknown type: {}", path.display());
        }
    }
    Ok(())
}

fn main() {
    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the user provided a path argument
    if args.len() < 2 {
        eprintln!("Usage: {} <path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);

    // Check if the provided path exists
    if !path.exists() {
        eprintln!("{} does not exist", path.display());
        return;
    }

    // Attempt to crawl the directory and handle any errors
    if let Err(e) = crawl(&path) {
        eprintln!("Error while crawling {}: {}", path.display(), e);
    }
}
