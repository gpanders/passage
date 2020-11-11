use colored::*;
use std::fs;
use std::fs::DirEntry;
use std::io::Result;
use std::path::Path;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

fn list(root: &Path, depth: usize) -> Result<()> {
    let mut entries = fs::read_dir(root)?
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .collect::<Vec<DirEntry>>();

    entries.sort_by_key(|k| k.file_name());

    for (i, entry) in entries.iter().enumerate() {
        for _ in 1..depth {
            print!("│   ");
        }

        if i == entries.len() - 1 {
            print!("└── ");
        } else {
            print!("├── ");
        }

        let path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();
        if path.is_dir() {
            println!("{}", file_name.blue().bold());
            list(path.as_path(), depth + 1)?;
        } else {
            let file_name = match file_name.strip_suffix(".gpg") {
                Some(e) => e,
                _ => &file_name,
            };

            println!("{}", file_name);
        }
    }

    Ok(())
}

fn main() {
    let store = dirs::home_dir().unwrap().join(".password-store");

    println!("Password Store");
    list(store.as_path(), 1).expect("Failed to list password store");
}
