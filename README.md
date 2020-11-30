# passage

`passage` is a password store utilizing the [`age`][age] encryption library.

Note that `age` is still considered **beta** software. Until `age` comes out of
beta, `passage` should be considered **beta** software as well.

[age]: https://age-encryption.org/v1

## Motivation

This project is an attempt to improve upon the success of [`pass`][pass] while
prioritizing speed and eschewing the use of PGP [[1][pgp-1], [2][pgp-2]].

[pass]: https://www.passwordstore.org/
[pgp-1]: https://latacora.micro.blog/2019/07/16/the-pgp-problem.html
[pgp-2]: https://blog.filippo.io/giving-up-on-long-term-pgp/

## Installation

Currently, the only supported method of installation is through Cargo.

    $ cargo install --git https://git.sr.ht/~gpanders/passage

Or, to build from a locally cloned copy of this repository,

    $ git clone https://git.sr.ht/~gpanders/passage
    $ cargo install --path passage

## Usage

`passage` is a near drop-in replacement for [`pass`][pass]. If you've used
`pass`, you can simply create an alias

    $ alias pass=passage

and continue to use `pass` as normal.

Initialize a new password store using

    $ passage init

This will create a new `age` private key at `$XDG_DATA_HOME/passage/key.txt`
and create your password store at `$HOME/.passage`.

Add a new password to the password store using

    $ passage insert path/to/item

or

    $ passage add path/to/item

You can retrieve a saved password from the store using

    $ passage show path/to/item

or simply

    $ passage path/to/item

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

You can view all items in your store with

    $ passage list

or

    $ passage show

or just

    $ passage
