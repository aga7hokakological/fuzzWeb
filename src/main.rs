extern crate clap;

// use reqwest::IntoUrl;
use reqwest::Url;

// use url::Url;

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::result::Result as OtherResult;
use clap::{App, Arg};
use std::ffi::OsString;

use error_chain::error_chain;

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

    fn new_from<I, T>(_args: I) -> OtherResult<Self, clap::Error>
    where
        I: Iterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // basic app information
        let app = App::new("fuzzWeb")
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

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

async fn fuzzer(url: Url) -> Result<()>
// where U: std::convert::AsRef<>, 
{
    let res = reqwest::get(url).await?;
    println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());

    // let body = res.text().await?;
    // println!("Body:\n{}", body);
    Ok(())
}

fn main() {
    let fuzzweb = Fuzz::new();

    // println!("{}", fuzzweb.url);

    let url = Url::parse(&fuzzweb.url).unwrap();
    // println!("{:?}", url.host_str().unwrap());

    // fuzzer(url);

    if let Ok(lines) = read_lines(&fuzzweb.file) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(fuzzword) = line {
                // println!("{}", fuzzword);
                let url_to_fuzz = url.join(&fuzzword);
                // println!("{:?}", url_to_fuzz.unwrap());
                let url1 = url_to_fuzz.unwrap();
                println!("{}", url1);
                fuzzer(url1);
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_no_args() {
        Fuzz::new_from(["exename"].iter()).unwrap_err();
    }

    #[test]
    fn test_incomplete_name() {
        Fuzz::new_from(["exename", "--name"].iter()).unwrap_err();
    }

    #[test]
    fn test_complete_name() {
        assert_eq!(
            Fuzz::new_from(["exename", "--name", "Hello"].iter()).unwrap(),
            Fuzz { url: "Hello".to_string(), file: "Yo".to_string() }
        );
    }

    #[test]
    fn test_short_name() {
        assert_eq!(
            Fuzz::new_from(["exename", "-n", "Hello"].iter()).unwrap(),
            Fuzz { name: "Hello".to_string(), file: "Yo".to_string()  }
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
        let _ignored = Fuzz::new_from(args.iter());
    }
}