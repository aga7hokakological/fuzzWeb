extern crate clap;

use clap::{App, Arg};
use std::ffi::OsString;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[derive(Debug, PartialEq)]
struct Fuzz {
    url: String,
    file: String,
}

impl Fuzz {
    fn new() -> Self {
        Self::new_from(std::env::args_os().into_iter()).unwrap_or_else(|e| e.exit())
    }

    fn new_from<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // basic app information
        let app = App::new("fuzz")
            .version("1.0")
            .about("Does awesome things")
            .author("Saurabh Mandavkar")
            .arg(Arg::with_name("url")
                .long("url") // allow --name
                .short("u") // allow -n
                .takes_value(true)
                .help("Enter url to fuzz")
                .required(true))
            .arg(Arg::with_name("file")
                .long("file") // allow --name
                .short("f") // allow -n
                .takes_value(true)
                .help("Enter fuzzz file")
                .required(true))
            .get_matches();

        // Extract the actual name
        let url1 = app
            .value_of("url")
            .expect("This can't be None, we said it was required");

        let file1 = app
            .value_of("file")
            .expect("This can't be None, we said it was required");

        Ok(Fuzz {
            url: url1.to_string(),
            file: file1.to_string(),
        })
    }
}

fn main() {
    let fuzzweb = Fuzz::new();

    println!("{}", fuzzweb.url);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_args() {
        HelloArgs::new_from(["exename"].iter()).unwrap_err();
    }

    #[test]
    fn test_incomplete_name() {
        HelloArgs::new_from(["exename", "--name"].iter()).unwrap_err();
    }

    #[test]
    fn test_complete_name() {
        assert_eq!(
            HelloArgs::new_from(["exename", "--name", "Hello"].iter()).unwrap(),
            HelloArgs { name: "Hello".to_string() }
        );
    }

    #[test]
    fn test_short_name() {
        assert_eq!(
            HelloArgs::new_from(["exename", "-n", "Hello"].iter()).unwrap(),
            HelloArgs { name: "Hello".to_string() }
        );
    }

    /* This property will fail, can you guess why?
    #[quickcheck]
    fn prop_any_name(name: String) {
        assert_eq!(
            HelloArgs::new_from(["exename", "-n", &name].iter()).unwrap(),
            HelloArgs { name }
        );
    }
    */

    #[quickcheck]
    fn prop_never_panics(args: Vec<String>) {
        let _ignored = HelloArgs::new_from(args.iter());
    }
}