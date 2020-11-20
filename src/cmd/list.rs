use colored::*;
use passage::{Error, PasswordStore};
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

fn tree(root: &Path, depth: usize, prefix: String) -> io::Result<()> {
    let mut entries = fs::read_dir(root)?
        .filter_map(|e| e.ok())
        .filter(|e| !is_hidden(e))
        .collect::<Vec<DirEntry>>();

    entries.sort_by_key(|k| k.file_name());

    for (i, entry) in entries.iter().enumerate() {
        let is_last = i == entries.len() - 1;
        if is_last {
            print!("{}└── ", prefix);
        } else {
            print!("{}├── ", prefix);
        }

        let path = entry.path();
        let file_name = entry.file_name().into_string().unwrap();

        if path.is_dir() {
            println!("{}", file_name.blue().bold());
            let next_prefix = prefix.clone() + if is_last { "    " } else { "│   " };
            tree(&path, depth + 1, next_prefix)?;
        } else {
            let file_name = match file_name.strip_suffix(".age") {
                Some(e) => e,
                _ => &file_name,
            };

            println!("{}", file_name);
        }
    }

    Ok(())
}

pub fn list(store: &PasswordStore) -> Result<(), Error> {
    if !store.dir.exists() {
        return Err(Error::Other(String::from(
            "Error: password store is empty. Try \"passage init\".",
        )));
    }

    println!("Password Store");
    tree(&store.dir, 1, String::from(""))?;

    Ok(())
}
