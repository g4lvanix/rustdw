use rustdw::{run, Wordlist};
use clap::{Arg, App, crate_version};

fn main() {
    let matches = App::new("rustdw")
               .version(crate_version!())
               .long_version(concat!(crate_version!(), "\n\n", include_str!("copyright_long.txt")))
              .author("g4lvanix <elia@dl1ye.com>")
               .long_about(concat!("Create diceware passwords and numeric pins", "\n\n", include_str!("copyright_short.txt")))
                    .arg(Arg::with_name("file")
                         .short("f")
                         .long("file")
                         .help("Name of external diceware wordlist file")
                         .takes_value(true))
                    .arg(Arg::with_name("wordlist")
                         .short("w")
                         .long("wordlist")
                         .help("Use one of the internal wordlists [default: efflarge]")
                         .conflicts_with("file")
                         .takes_value(true)
                         .possible_values(&Wordlist::variants()))
                    .arg(Arg::with_name("length")
                         .short("l")
                         .long("length")
                         .help("Length of the generated password or pin")
                         .required(false)
                         .takes_value(true)
                         .default_value("6"))
                    .arg(Arg::with_name("entropy")
                         .short("H")
                         .long("entropy")
                         .help("Minimum entropy of the generated password or pin in bits")
                         .required(false)
                         .takes_value(true)
                         .conflicts_with("length"))
                    .arg(Arg::with_name("p")
                         .short("p")
                         .long("pin")
                         .help("Generate a numeric pin insted of diceware passphrase")
                         .required(false)
                         .takes_value(false))
               .get_matches();

     let passphrase = run(matches);

     println!("{}", passphrase);
}
