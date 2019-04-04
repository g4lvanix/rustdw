use rand::prelude::*;
use rand::distributions::Uniform;

use clap::{ArgMatches, arg_enum, value_t, _clap_count_exprs};

use std::fs::File;
use std::io::{BufReader, BufRead, Result};
use std::process;

arg_enum!{
    #[derive(Debug)]
    pub enum Wordlist {
        efflarge,
        effshort1,
        effshort2,
        bip39en,
        bip39es,
        bip39fr,
        bip39it,
   }
}

pub fn run(matches: ArgMatches) -> String {
    let mut rng = thread_rng();

    let length: usize = matches.value_of("length").unwrap()
                        .parse().unwrap_or_else(|e| {
                            println!("Error parsing passphrase length: {}", e);
                            process::exit(1);
                        });

    if matches.is_present("p") {
        generate_pin(&mut rng, length)
    } else if matches.is_present("file") {
        let fname = matches.value_of("file").unwrap();

        let words = read_wordlist(fname).unwrap_or_else(|e| {
            println!("Error reading wordlist: {}", e);
            process::exit(1);
        });
          
        generate_passphrase(&mut rng, length, &words.as_slice())
    } else {
        let wl = value_t!(matches.value_of("wordlist"), Wordlist).unwrap_or(Wordlist::efflarge);

        let words = match wl {
            Wordlist::efflarge => eff_wordlist(include_str!("lists/eff/eff_large_wordlist.txt")),
            Wordlist::effshort1 => eff_wordlist(include_str!("lists/eff/eff_short_wordlist_1.txt")),
            Wordlist::effshort2 => eff_wordlist(include_str!("lists/eff/eff_short_wordlist_2_0.txt")),

            Wordlist::bip39en => bip39_wordlist(include_str!("lists/bip39/english.txt")),
            Wordlist::bip39es => bip39_wordlist(include_str!("lists/bip39/spanish.txt")),
            Wordlist::bip39fr => bip39_wordlist(include_str!("lists/bip39/french.txt")),
            Wordlist::bip39it => bip39_wordlist(include_str!("lists/bip39/italian.txt")),
       };

        generate_passphrase(&mut rng, length, &words)
    }
}

fn read_wordlist(fname: &str) -> Result<Vec<String>> {
    let file = File::open(fname)?;

    Ok(BufReader::new(file).lines()
                .filter_map(|x| x.ok())
                .filter_map(|x| {
                    if let Some(k) = x.split_whitespace().rev().next()
                    {
                        Some(k.to_owned())
                    } else {
                        None
                    }
                })
                .collect())
}

fn generate_pin<R: CryptoRng + RngCore>(mut rng: R, length: usize) -> String {
    let dist = Uniform::from(0..10);
    dist.sample_iter(&mut rng)
        .take(length)
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn generate_passphrase<R, T>(mut rng: R, length: usize, words: &[T]) -> String
where
    R: CryptoRng + RngCore,
    T: AsRef<str>,
{
    let dist = Uniform::from(0..words.len());
    dist.sample_iter(&mut rng)
        .take(length)
        .map(|i| words[i].as_ref())
        .collect::<Vec<&str>>()
        .join(" ") 
}

fn eff_wordlist(string: &'static str) -> Vec<&'static str> {
    string.split_whitespace()
    .filter(|x| x.chars().all(char::is_alphabetic))
    .collect()
}

fn bip39_wordlist(string: &'static str) -> Vec<&'static str> {
    string.split_whitespace()
    .collect()
}
