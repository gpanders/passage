use std::env;

mod cmd;

fn main() {
    let store = dirs::home_dir().unwrap().join(".password-store");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cmd::list(&store);
    } else {
        let cmd = &args[1][..];
        match cmd {
            "list" => cmd::list(&store),
            "show" => cmd::show(&store, &args[2]),
            "init" => cmd::init(&store),
            _ => cmd::show(&store, &args[1]),
        };
    }
}
