mod cmd;

use clap::{App, AppSettings, Arg, SubCommand};
use passage::PasswordStore;
use std::process;

fn main() {
    let store = PasswordStore::new(dirs::home_dir().unwrap().join(".passage"));

    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .arg(
            Arg::with_name("clip")
                .short("c")
                .long("clip")
                .requires("item")
                .hidden(true),
        )
        .arg(Arg::with_name("item").value_name("NAME"))
        .subcommand(
            SubCommand::with_name("init").about("Initialize a password store with a new key"),
        )
        .subcommand(
            SubCommand::with_name("show")
                .about("Decrypt and print a password")
                .arg(Arg::with_name("item").value_name("NAME"))
                .arg(
                    Arg::with_name("clip")
                        .help("Copy password to the system clipboard")
                        .short("c")
                        .long("clip")
                        .requires("item"),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List passwords")
                .alias("ls")
                .arg(Arg::with_name("subfolder")),
        )
        .subcommand(
            SubCommand::with_name("insert")
                .about("Insert a new item into the password store")
                .alias("add")
                .arg(Arg::with_name("item").value_name("NAME").required(true)),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove an item from the password store")
                .alias("rm")
                .arg(Arg::with_name("item").value_name("NAME").required(true)),
        )
        .subcommand(
            SubCommand::with_name("lock")
                .about("Lock the password store by encrypting the secret key with a passphrase"),
        )
        .subcommand(
            SubCommand::with_name("unlock")
                .about("Unlock the password store by decrypting the secret key"),
        )
        .get_matches();

    let result = match matches.subcommand() {
        ("show", Some(sub)) => match sub.value_of("item") {
            Some(item) => cmd::show(store, item, sub.is_present("clip")),
            None => cmd::list(store),
        },
        ("init", Some(_)) => cmd::init(store),
        ("list", Some(_)) => cmd::list(store),
        ("lock", Some(_)) => cmd::lock(),
        ("unlock", Some(_)) => cmd::unlock(),
        ("insert", Some(sub)) => cmd::insert(store, sub.value_of("item").unwrap()),
        ("remove", Some(sub)) => cmd::remove(store, sub.value_of("item").unwrap()),
        ("", None) => match matches.value_of("item") {
            Some(item) => cmd::show(store, item, matches.is_present("clip")),
            None => cmd::list(store),
        },
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
        process::exit(1);
    }
}
