extern crate clap;
use clap::{Arg, App};

extern crate rand;
use rand::prelude::*;
use rand::distributions::Uniform;

use std::fs;
use std::process;

fn main() {

    let matches = App::new("rustdw")
                          .version("0.1")
                          .author("g4lvanix <elia@dl1ye.com>")
                          .about("Create diceware passwords and numeric pins")
                          .arg(Arg::with_name("file")
                               .short("f")
                               .help("Name of the diceware wordlist file")
                               .takes_value(true)
                               .default_value("eff_large_wordlist.txt"))
                          .arg(Arg::with_name("length")
                               .short("l")
                               .help("Length of the generated password or pin")
                               .required(false)
                               .takes_value(true)
                               .default_value("6"))
                          .arg(Arg::with_name("p")
                               .short("p")
                               .help("Generate a numeric pin insted of diceware passphrase")
                               .required(false)
                               .takes_value(false))
                          .get_matches();

     let mut rng = thread_rng();

     let length: usize = matches.value_of("length").unwrap()
                         .parse().unwrap_or_else(|e| {
                             println!("Error parsing passphrase length: {}", e);
                             process::exit(1);
                         });

     let passphrase = if matches.is_present("p") {
          let dist = Uniform::from(0..10);
          dist.sample_iter(&mut rng)
               .take(length)
               .map(|x| x.to_string())
               .collect::<Vec<String>>()
               .join("")
     } else {
          let fname = matches.value_of("file").unwrap();

          let contents = fs::read_to_string(fname).unwrap_or_else(|e| {
              println!("Error reading wordlist: {}", e);
              process::exit(1);
          });
          
          let words: Vec<&str> = contents.split_whitespace()
                              .filter(|x| {!x.chars().all(char::is_numeric)})
                              .collect();
          
          let dist = Uniform::from(0..words.len());

          dist.sample_iter(&mut rng)
               .take(length)
               .map(|i| words[i])
               .collect::<Vec<&str>>()
               .join(" ")         
     };

     println!("{}", passphrase);
}
