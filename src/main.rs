use colored::*;
use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Result, Write};
use std::path::Path;
use std::process::Command;

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map_or(false, |s| s.starts_with('.'))
}

fn tree(root: &Path, depth: usize, prefix: String) -> Result<()> {
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
            let file_name = match file_name.strip_suffix(".gpg") {
                Some(e) => e,
                _ => &file_name,
            };

            println!("{}", file_name);
        }
    }

    Ok(())
}

fn cmd_show(store: &Path, item: &str) {
    let item_path = String::from(store.join(item).to_str().unwrap()) + ".gpg";

    let output = Command::new("gpg")
        .arg("--decrypt")
        .arg(item_path)
        .output()
        .unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
}

fn cmd_list(store: &Path) {
    println!("Password Store");
    tree(&store, 1, String::from("")).expect("Failed to list password store");
}

fn main() {
    let store = dirs::home_dir().unwrap().join(".password-store");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cmd_list(&store);
    } else {
        let item = &args[1];
        cmd_show(&store, item);
    }
}
