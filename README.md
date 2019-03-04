# rustdw - A diceware passhphrase generator written in Rust

This command line tool can generate secure but memorable 
passhphrases based on the [diceware technique](http://world.std.com/~reinhold/diceware.html).
The Electronic Frontier Foundation (EFF) has a great article on [how this technique works](https://www.eff.org/dice).

`rustdw` uses a cryptographically secure random number generator provided by the 
[`rand`](https://docs.rs/rand) crate to pick random words from the word list. 
The words are chosen according to a unifom distribution (every word is equally likely to be chosen).

## Installation

After [installing rust](https://www.rust-lang.org/tools/install), clone the repo and build it using cargo:

```
$ git clone github.com/g4lvanix/rustdw
$ cd rustdw
$ cargo build --release
```

## Usage 

Run `rustdw --help` for usage info.

```
rustdw 0.2
g4lvanix <elia@dl1ye.com>
Create diceware passwords and numeric pins

USAGE:
    rustdw [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -p               Generate a numeric pin insted of diceware passphrase
    -V, --version    Prints version information

OPTIONS:
    -f <file>            Name of external diceware wordlist file
    -l <length>          Length of the generated password or pin [default: 6]
    -w <wordlist>        Use one of the internal wordlists [default: large]  [possible values: large, short1, short2]

```

### Diceware passphrases 

The entropy of the generated passhphrase (assuming selection according to uniform distribution) can be estimated as: 

`S = l x log2(k)` bits

where `k` is the number of words in the wordlist and `l` is the number of chosen words.

#### Included wordlists 

`rustdw` includes three [word lists published by the EFF](https://www.eff.org/deeplinks/2016/07/new-wordlists-random-passphrases). 
Please refer to the linked page for a discussion of tradeoffs in selecting the 
wordlist and explanations on the cryptographic strength of the resulting passphrases.

| Argument | Generated from               | Number of words | Entropy per word (bits) |
| -------- | ---------------------------- | --------------- | ----------------------- |
| `large`  | `eff_large_wordlist.txt`     | 7776            | 12.9                    |
| `short1` | `eff_short_wordlist_1.txt`   | 1296            | 10.3                    |
| `short2` | `eff_short_wordlist_2_0.txt` | 1296            | 10.3                    |

At the time of this writing, using 6 words from the `large` wordlist (77 bits entropy) 
or 7 words from the `short1` wordlist (72 bits entropy) are recommended by the EFF. 

To generate a 7 word long passhphrase with words chosen from EFFs large wordlist:

```
$ ./rustdw -l 7
```

To use the EFF short word list:

```
$ ./rustdw -l 7 -w short1
```

To use the EFF short words with unique prefixes list:

```
$ ./rustdw -l 7 -w short2
```

#### Wordlists from other sources

It is possible to use a word list that is not included in rustdw, for example the
[original diceware list](http://world.std.com/~reinhold/diceware.html) created by Arnold G. Reinhold.
Please note that the list format has to be exactly the same as those published by EFF:

```
11111	abacus
11112	abdomen
    :
    :
```

The external word list can be specified like this

```
$ ./rustdw -f external_wordlist.txt -l 5
```

### Numeric PINs

`rustdw` can also create numeric passwords for use as a PIN (personal identification number). 

To generate a 4 digit PIN:

```
$ ./rustdw -p -l 4
```

## Notes 

[Relevant xkcd](https://www.xkcd.com/936/)
