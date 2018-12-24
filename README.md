# rustdw - A diceware passhphrase generator written in Rust

This command line tool can generate secure but memorable 
passhphrases based on the [diceware technique](http://world.std.com/~reinhold/diceware.html).
The Electronic Frontier Foundation (EFF) has a great article on [how this technique works](https://www.eff.org/dice).

Besides this tool you need a word list, either the [original diceware list](http://world.std.com/~reinhold/diceware.html) created by Arnold G. Reinhold 
or one of the [lists published by the EFF](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases).

`rustdw` uses a cryptographically secure random number generator provided by the 
[`rand crate`](https://docs.rs/rand) to pick random words from the word list. 
The words are chosen according to a unifom distribution (every word is equally likely to be chosen).

## Installation

After [installing rust](https://www.rust-lang.org/tools/install), clone the repo and build it using cargo:

```
$ git clone github.com/g4lvanix/rustdw
$ cd rustdw
rustdw $ cargo build --release
```

Additionally, please obtain one of the wordlists from the sources mentioned above.

On Linux you can use the `wget` command: 

```
$ wget https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt
```

## Usage 

Run `rustdw --help` for usage info.

### Diceware passphrases 

```
rustdw 0.1
g4lvanix <elia@dl1ye.com>
Create diceware passwords and numeric pins

USAGE:
    rustdw [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -p               Generate a numeric pin insted of diceware passphrase
    -V, --version    Prints version information

OPTIONS:
    -f <file>          Name of the diceware wordlist file [default: eff_large_wordlist.txt]
    -l <length>        Length of the generated password or pin [default: 6]

```

To generate a 5 word long passhphrase with words chosen from EFFs large wordlist:

```
$ ./rustdw -l 5
```

To use a different word list:

```
$ ./rustdw -f eff_short_wordlist_1.txt -l 5
```

### Numeric PINs

`rustdw` can also create numeric passwords for use as a PIN (personal identification number). 

To generate a 4 digit PIN:

```
$ ./rustdw -p -l 4
```

## Notes 

[Relevant xkcd](https://www.xkcd.com/936/)
