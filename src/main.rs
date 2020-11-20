mod cmd;

use passage::PasswordStore;
use std::env;
use std::process;

fn main() {
    let store = PasswordStore::new(dirs::home_dir().unwrap().join(".passage"));

    let args: Vec<String> = env::args().collect();

    let result = if args.len() < 2 {
        cmd::list(&store)
    } else {
        let cmd = &args[1][..];
        match cmd {
            "list" => cmd::list(&store),
            "show" => {
                if args.len() > 2 {
                    cmd::show(&store, &args[2])
                } else {
                    cmd::list(&store)
                }
            }
            "init" => cmd::init(&store),
            _ => cmd::show(&store, &args[1]),
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}
