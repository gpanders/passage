mod cmd;

use passage::PasswordStore;
use std::env;
use std::process;

fn main() {
    let store = PasswordStore::new(dirs::home_dir().unwrap().join(".passage"));

    let args: Vec<String> = env::args().collect();

    let result = if args.len() < 2 {
        cmd::list(store)
    } else {
        let cmd = &args[1][..];
        let arg = if args.len() > 2 {
            Some(&args[2][..])
        } else {
            None
        };
        match cmd {
            "list" => cmd::list(store),
            "show" => match arg {
                Some(arg) => cmd::show(store, arg),
                None => cmd::list(store),
            },
            "init" => cmd::init(store),
            "insert" => cmd::insert(store, arg),
            "rm" => cmd::remove(store, arg),
            _ => cmd::show(store, &args[1]),
        }
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}
