extern crate reqwest;
extern crate clap;
extern crate dns_lookup;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self,BufReader};
use clap::{Arg, App};
use reqwest::Url;
use dns_lookup::{lookup_host,lookup_addr,LookupError};

fn get_url(url: Url) -> Result<(),reqwest::Error>{
    let res = reqwest::get(url)?;
    print!("{} ",res.url());
    println!("Status: {}", res.status());
    Ok(())
}


fn get_dns(hostname: &str) -> Result<(),LookupError> {
    let ips:Vec<std::net::IpAddr> = lookup_host(hostname)?;
    print!("{} ",hostname);
    for ip in ips{
        print!("{} ",ip);
    }
    println!("");
    Ok(())
}

fn request_url(host : &str,line: &str)-> Result<(),reqwest::Error>{
    let rurl = host.to_string() + line;
    println!("{}", rurl);
    let rurl = Url::parse(&rurl).unwrap();
    get_url(rurl)
}

fn request_dns(host: &str,line: String)-> Result<(),LookupError>{
    let hostname = line + "." + host;
    get_dns(&hostname)
}


fn main() -> Result<(), Box<std::error::Error>> {
    let matches = App::new("rbrute")
                        .version("0.1")
                        .author("Attenuation <ouyangjun1999@gmail.com>")
                        .about("Directory/file & DNS bruting tool written in rust")
                        .arg(Arg::with_name("wordlist")
                            .short("w")
                            .long("wordlist")
                            .value_name("FILE")
                            .help("select a wordlist")
                            .takes_value(true))
                        .arg(Arg::with_name("mode")
                            .short("m")
                            .long("mode")
                            .value_name("mode")
                            .help("Directory/File mode (dir) or DNS mode (dns) (default 'dir')")
                            .takes_value(true)
                            .default_value("url")
                            )
                        .arg(Arg::with_name("url")
                           .help("input a url or domain")
                           .required(true)
                           .index(1))
                        .get_matches();
    let host = matches.value_of("url").expect("error url");
    let mode = matches.value_of("mode").expect("error mode");
    let wordlist = matches.value_of("wordlist").expect("error wordlist");
    let word = File::open(wordlist)?;
    let word = BufReader::new(word);
    for line in word.lines() {
        let line = line.unwrap();
        match mode{
            "url" => {
                request_url(host, &line);
                continue;
            },
            "dns" => {
                request_dns(host, line);
                continue;
            },
            _ => {
                break;
            }
        }
    }
    Ok(())
}