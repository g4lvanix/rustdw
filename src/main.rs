use rustdw::{run, Wordlist};
use clap::{Arg, App, crate_version};

fn main() {
    let matches = App::new("rustdw")
               .version(crate_version!())
               .long_version(concat!(crate_version!(), "

    Copyright (C) 2019 Yuma Ritterbusch

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>"))
               .author("g4lvanix <elia@dl1ye.com>")
               .long_about("
    Create diceware passwords and numeric pins

    Copyright (C) 2019 Yuma Ritterbusch 
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions; see `rustdw --version` for details.
")
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
