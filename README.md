# passage

`passage` is a password store utilizing the [`age`][age] encryption library.

Note that `age` is still considered **beta** software. Until `age` comes out of
beta, `passage` should be considered **beta** software as well.

[age]: https://age-encryption.org/v1

# Table of Contents

* [FAQ](#faq)
* [Installation](#installation)
* [Usage](#usage)

## FAQ

### How is passage different from pass?

`passage` is a reimplementation of [`pass`][pass] that eschews the use of PGP
[[1][pgp-1], [2][pgp-2]] in favor of [`age`][age].

[pass]: https://www.passwordstore.org/
[pgp-1]: https://latacora.micro.blog/2019/07/16/the-pgp-problem.html
[pgp-2]: https://blog.filippo.io/giving-up-on-long-term-pgp/

### Is passage secure?

`passage`, like all password managers, makes trade-offs between security and
convenience. Whether or not `passage` meets your security qualifications
depends on your use case.

`passage` uses a secret key that is only accessible by you. This means if you
are on a single-user system or a multi-user system where only you have root
access, no one can access your secret key.

If you are on a multi-user system where other users have root access, **then it
is possible for those users to read your secret key**. You can eliminate this
risk by locking your password store (i.e. encrypting your secret key with a
passphrase) at the cost of having to re-enter your passphrase each time you
wish to retrieve an item from your store.

## Installation

Currently, the only supported method of installation is through Cargo.

    $ cargo install --git https://git.sr.ht/~gpanders/passage

This will install `passage` to `$HOME/.cargo/bin`.

Or, to build from a locally cloned copy of this repository,

    $ git clone https://git.sr.ht/~gpanders/passage
    $ cd passage
    $ make install

This will install `passage` to `/usr/local/bin`. To specify a different install
prefix, set the `prefix` variable in the `make` command, e.g.:

    $ make prefix=/opt/passage install

## Usage

`passage` is a near drop-in replacement for [`pass`][pass]. If you've used
`pass`, you can simply create an alias

    $ alias pass=passage

and continue to use `pass` as normal.

### Initialization

Initialize a new password store using

    $ passage init

This will create a new `age` private key at `$XDG_DATA_HOME/passage/key.txt`
and create your password store at `$HOME/.passage`.

You can add additional recipients to your password store using the
`-r`/`--recipients` option to `init`:

    $ passage init -r age1294r5jdje2n2jprxj0avqyvmpsujzlmjt5kla728x5eykgd8cc9skkms53

You can also use an existing age secret key instead of generating a new one
with the `-k`/`--key` option:

    $ age-keygen -o key.txt
    $ passage init -k key.txt

### Adding and retrieving items

Add a new password to the password store using

    $ passage insert ITEM

or

    $ passage add ITEM

You can retrieve a saved password from the store using

    $ passage show ITEM

or simply

    $ passage ITEM

Use the `-c`/`--clip` flag to copy the password to your system clipboard:

    $ passage -c ITEM
    Copied password for ITEM to clipboard.

### View store contents

You can view all items in your store with

    $ passage list

or

    $ passage show

or just

    $ passage

### Locking and unlocking

You can lock your password store using

    $ passage lock

This will prompt you for a passphrase, which will be used to encrypt your
secret key. When locked, you will need to enter this passphrase anytime you
wish to display a password from the store.

Note that locking the password store will only require a passphrase when
retrieving a password from the store, **not** when inserting a new password
into the store. This is because only the secret key is encrypted with the
passphrase. The public key used to add new items remains unencrypted.

You can unlock a locked password store using

    $ passage unlock

