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
        .map(PathBuf::from)
        .unwrap_or_else(|_| dirs::home_dir().unwrap().join(".passage"));
    let store = PasswordStore::new(dir);

    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .setting(AppSettings::ArgsNegateSubcommands)
        .setting(AppSettings::VersionlessSubcommands)
        .about("Password store built on the age encryption library")
        .long_about(
            "
passage is a UNIX-style password management tool that allows you to combine password and secrets
management with other programs.

To get started, run 'passage init' to initialize your password store and generate a secret key.
Then use 'passage insert ITEM' and 'passage show ITEM' to insert and retrieve passwords from your
store.

Use 'passage help <SUBCOMMAND>' for more information on how to use each subcommand.
",
        )
        .arg(
            Arg::with_name("clip")
                .short("c")
                .long("clip")
                .requires("item")
                .hidden(true),
        )
        .arg(
            Arg::with_name("item")
                .help("Display password for NAME")
                .value_name("NAME"),
        )
        .subcommand(
            SubCommand::with_name("edit")
                .about("Edit an existing item in the password store")
                .arg(Arg::with_name("item").value_name("NAME").required(true)),
        )
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a password store")
                .long_about(
                    "
If a password store does not already exist, and the -k/--key flag is not provided, a new secret key
will be auto-generated.

With -k/--key, the provided secret key is used for the password store instead. If a secret key
already exists, the store is reencrypted to use the new key.

The public key corresponding to your store's secret key is automatically added to the recipients
list in your store. You can add additional recipients to your store using the -r/--recipient flag.
The argument to this flag should be an age public key.

Examples:

Initialize a new store:

    $ passage init

Initialize a new store or reinitialize an existing store using an existing key:

    $ age-keygen -o key.txt
    $ passage init -k key.txt

Add other recipients to your store:

    $ passage init -r age1294r5jdje2n2jprxj0avqyvmpsujzlmjt5kla728x5eykgd8cc9skkms53
",
                )
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
                .about("Retrieve a password from the store")
                .long_about(
                    "
With no arguments, 'passage show' is equivalent to 'passage ls' or just 'passage'; namely, it lists
the contents of the store.

With an argument, decrypt and display the given item from the store, if it exists. With -c/--clip,
copy the password to the system clipboard instead.
",
                )
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
            SubCommand::with_name("ls")
                .alias("list")
                .about("Display the contents of your password store")
                .long_about(
                    "
Displays the directory tree of the password store.

This command is alternatively called 'list'.
",
                ),
        )
        .subcommand(
            SubCommand::with_name("insert")
                .alias("add")
                .about("Insert a new item into the password store")
                .long_about(
                    "
Create a new item in the password store with the given name. If no argument is given, the user is
prompted for the name of the item to create. If the item already exists in the password store, the
user is prompted to confirm that they wish to overwrite the existing item. To bypass confirmation,
use the -f/--force flag.

The user is then prompted to enter the password for the new item and then asked again to confirm
the password.

This command is alternatively called 'add'.
",
                )
                .arg(Arg::with_name("item").value_name("NAME"))
                .arg(
                    Arg::with_name("force")
                        .help("Don't ask before overwriting an existing item")
                        .short("f")
                        .long("force")
                        .requires("item"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .alias("remove")
                .about("Remove an item from the password store")
                .long_about(
                    "
Remove the given item from the password store, if it exists. If no argument is given, the user is
prompted for the name of the item to remove.

The user is prompted to confirm that they wish to remove the given item. To bypass confirmation,
use the -f/--force flag.

This command is alternatively called 'remove'.
",
                )
                .arg(Arg::with_name("item").value_name("NAME"))
                .arg(
                    Arg::with_name("force")
                        .help("Don't ask for confirmation")
                        .short("f")
                        .long("force")
                        .requires("item"),
                ),
        )
        .subcommand(
            SubCommand::with_name("lock")
                .about("Lock the password store")
                .long_about(
                    "
Locks the password store by encrypting the secret key with a passphrase. While locked, any time
a password is retrieved from the store (using 'passage show'), the passphrase must be entered.

This can be used to provide an additional measure of security when using systems where other
users have root access.
",
                ),
        )
        .subcommand(
            SubCommand::with_name("unlock")
                .about("Unlock the password store")
                .long_about(
                    "
Unlock the password store by decrypting the secret key.
",
                ),
        )
        .subcommand(
            SubCommand::with_name("key")
                .about("Display the password store's key")
                .long_about(
                    "
With no options, display the public key corresponding to the password store's secret key. Use
-s/--secret to display the password store's secret key.
",
                )
                .arg(
                    Arg::with_name("secret")
                        .help("Show secret key")
                        .short("s")
                        .long("secret"),
                ),
        )
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
        ("ls", Some(_)) => cmd::list(store),
        ("lock", Some(_)) => cmd::lock(),
        ("unlock", Some(_)) => cmd::unlock(),
        ("key", Some(sub)) => cmd::key(sub.is_present("secret")),
        ("insert", Some(sub)) => cmd::insert(store, sub.value_of("item"), sub.is_present("force")),
        ("rm", Some(sub)) => cmd::remove(store, sub.value_of("item"), sub.is_present("force")),
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
