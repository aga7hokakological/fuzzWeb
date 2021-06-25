use reqwest::{Url, Client};
use futures::future;
use clap::{App, Arg};

use error_chain::error_chain;

use std::thread;
use std::vec::Vec;
use std::io::{self, prelude::*};
use std::path::Path;
use std::fs::File;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn fuzz(urls: Vec<Url>) -> Result<()> {
    let client = Client::new();

    // let res = reqwest::get(target).await?;

    let res = future::join_all(urls.into_iter().map(|url| {
        let client = &client;
        async move {
            let resp = client.get(url).send().await;
            // println!("{:?}", resp);
            resp.unwrap()
        }
    }))
    .await;

    // println!("{:?}", res.status());

    for r in res {
        // match r {
        //     Ok(r) => println!("{}", r.status()),
        //     Err(e) => eprintln!("{}", e),
        // }

        println!("{:?}: {:?}", r.url().as_str(), r.status());
    }

    Ok(())
}

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path> {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

#[tokio::main]
async fn main() -> Result<()> {

    let app = App::new("fuzzWeb")
            .version("1.0")
            .about("Does awesome things")
            .author("Saurabh Mandavkar")
            .args_from_usage(
                "
                -u, --url=[TARGET_URL] 'set your target URL(required)'
                -w, --wordlist=[PATH_TO_WORDLIST] 'set your wordlist(required)'
                -t, --timeout=[SECONDS] 'set the timeout time in seconds Default(15)'
                "
            )
            .arg(
                Arg::with_name("v")
                .short("v")
                .long("verbose")
                .help("show verbose output")
            )
            .get_matches();

    // Extract the actual name
    let target = app.value_of("url").unwrap();
    let wordlist = app.value_of("wordlist").unwrap();
    let _timeout = 10;
    
    let mut new_vec = Vec::new();
    
    let url = Url::parse(&target).unwrap();

    if let Ok(lines) = read_file(&wordlist) {
        for line in lines {
            if let Ok(fuzzword) = line {
                let url_to_fuzz = url.join(&fuzzword);
                let target_url = url_to_fuzz.unwrap();

                new_vec.push(target_url);
            }
        }
    }

    thread::spawn(|| {
        fuzz(new_vec);
    }).join().expect("Thread panicked");

    Ok(())
}