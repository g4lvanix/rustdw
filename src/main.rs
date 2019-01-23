extern crate rustdw;
use rustdw::*;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("rustdw")
               .version("0.2")
               .author("g4lvanix <elia@dl1ye.com>")
               .about("Create diceware passwords and numeric pins")
                    .arg(Arg::with_name("file")
                         .short("f")
                         .help("Name of external diceware wordlist file")
                         .takes_value(true))
                    .arg(Arg::with_name("wordlist")
                         .short("w")
                         .help("Use one of the internal wordlists [default: large]")
                         .conflicts_with("file")
                         .takes_value(true)
                         .possible_values(&Wordlist::variants()))
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

     let passphrase = run(matches);

     println!("{}", passphrase);
}
