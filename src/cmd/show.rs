use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn show(store: &Path, item: &str) {
    let item_path = String::from(store.join(item).to_str().unwrap()) + ".gpg";

    let output = Command::new("gpg")
        .arg("--decrypt")
        .arg(item_path)
        .output()
        .unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
}
