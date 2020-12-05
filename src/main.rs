use clap::{App, AppSettings, Arg, SubCommand};
use std::env;
use std::path::PathBuf;
use std::process;

mod cmd;
mod crypt;
mod error;
mod input;
mod key;
mod store;

use store::PasswordStore;

fn main() {
    let dir = env::var("PASSAGE_STORE_DIR")
        .map(|s| PathBuf::from(s))
        .unwrap_or(dirs::home_dir().unwrap().join(".passage"));
    let store = PasswordStore::new(dir);

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
            SubCommand::with_name("edit")
                .about("Edit an existing item in the password store")
                .arg(Arg::with_name("item").value_name("NAME").required(true)),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a password store with a new key")
                .arg(
                    Arg::with_name("recipient")
                        .help("Add an additional recipient to the password store")
                        .short("r")
                        .long("recipient")
                        .multiple(true)
                        .number_of_values(1)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("key")
                        .help("Initialize store with an existing secret key")
                        .short("k")
                        .long("key")
                        .takes_value(true),
                ),
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
                .arg(Arg::with_name("item").value_name("NAME")),
        )
        .subcommand(
            SubCommand::with_name("remove")
                .about("Remove an item from the password store")
                .alias("rm")
                .arg(Arg::with_name("item").value_name("NAME")),
        )
        .subcommand(
            SubCommand::with_name("lock")
                .about("Lock the password store by encrypting the secret key with a passphrase"),
        )
        .subcommand(
            SubCommand::with_name("unlock")
                .about("Unlock the password store by decrypting the secret key"),
        )
        .subcommand(SubCommand::with_name("pubkey").about("Display password store public key"))
        .get_matches();

    let result = match matches.subcommand() {
        ("show", Some(sub)) => match sub.value_of("item") {
            Some(item) => cmd::show(store, item, sub.is_present("clip")),
            None => cmd::list(store),
        },
        ("edit", Some(sub)) => cmd::edit(store, sub.value_of("item")),
        ("init", Some(sub)) => {
            let recipients = sub
                .values_of("recipient")
                .map(|v| v.map(|s| s.parse()).filter_map(|r| r.ok()).collect());
            cmd::init(store, recipients, sub.value_of("key").map(|s| s.to_owned()))
        }
        ("list", Some(_)) => cmd::list(store),
        ("lock", Some(_)) => cmd::lock(),
        ("unlock", Some(_)) => cmd::unlock(),
        ("pubkey", Some(_)) => cmd::pubkey(),
        ("insert", Some(sub)) => cmd::insert(store, sub.value_of("item")),
        ("remove", Some(sub)) => cmd::remove(store, sub.value_of("item")),
        ("", None) => match matches.value_of("item") {
            Some(item) => cmd::show(store, item, matches.is_present("clip")),
            None => cmd::list(store),
        },
        _ => unreachable!(),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
