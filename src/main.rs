extern crate reqwest;
extern crate scraper;
extern crate regex;

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Error;
use std::{fs, io};
use std::fs::File;
use std::io::{Read, ErrorKind};
use std::io::Write;
use std::io::BufWriter;
use std::io::BufReader;
use std::fs::OpenOptions;
use serde_json::Value as JsonValue;
use std::path::Path;
use regex::bytes::Regex;


#[derive(Debug, PartialEq, Deserialize)]
struct Pattern {
    common: String,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    payload_type: String,
    payload_text: String,
}

fn parse_json() -> Vec<String> {

    let mut payload_vector = Vec::new();

    let json_file_path = Path::new("/Users/jmcbride/IdeaProjects/scraper-v2/payload.json");
    let file = File::open(json_file_path).expect("file not found");

    let payloads:Vec<Payload> = serde_json::from_reader(file).expect("Error while reading!");

    for payload in payloads {
        payload_vector.push(payload.payload_text.to_string());
    }
    payload_vector
}

fn parse_patterns() -> Vec<String> {

    let mut regex_vector = Vec::new();

    let json_file_path = Path::new("baserules.json");
    let file = File::open(json_file_path).expect("file not found");

    let patterns:Vec<Pattern> = serde_json::from_reader(file).expect("Error while reading!");

    for pattern in patterns {
        regex_vector.push(pattern.common);
    }
    regex_vector
}

fn test_regex() {

    let mut pattern_regexs = parse_patterns();
    let mut validated_payloads = parse_json();

    println!("Total Patterns Tested: {}", validated_payloads.len());

    let mut failed_vec = Vec::new();
    let mut passed_vec = Vec::new();
    let mut counter = 0;

    // nested for to inter each regex from json wit h the payloads scraped

    for regex_item in pattern_regexs.iter() {

        let regex_tester = Regex::new(regex_item).unwrap();

        for code in validated_payloads.iter() {
            if !regex_tester.is_match(code.as_ref()) {
                passed_vec.push(code);
                counter + 1;
            }

            else {
                failed_vec.push(code);
            }

        }
    }

    // sorting the fail vector list and deleting duplicates
    failed_vec.sort();
    failed_vec.dedup();

    let mut matches = validated_payloads.len() - failed_vec.len();

    println!("Total Matches: {}", matches);
    println!("Total Failures: {}", failed_vec.len());

}

fn main() {

    test_regex();
}