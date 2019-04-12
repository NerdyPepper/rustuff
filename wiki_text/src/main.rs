extern crate clap;
extern crate reqwest;
extern crate serde_json;

use reqwest::Url;
use serde_json::Value;

fn main() {
    println!("Hello, world!");
    let mut title = "Dynamic_programming".to_string();
    println!("{}", query_url_gen(&title[..]));

    let mut res = reqwest::get(query_url_gen(&title[..])).unwrap();
    let v: Value = res.json().expect("Failed to parse json");

    println!("{}", get_extract(&v).unwrap());
}

fn query_url_gen(title: &str) -> Url {
    Url::parse_with_params(
        &("https://en.wikipedia.org/w/api.php"),
        &[
            ("action", "query"),
            ("format", "json"),
            ("prop", "extracts|links"),
            ("indexpageids", "1"),
            ("titles", &urlencoding::encode(&title.replace(" ", "_"))),
            ("redirects", "1"),
            ("pllimit", "100"),
            ("explaintext", "1"),
        ],
    ).unwrap()
}

fn get_extract(v: &Value) -> Result<String, reqwest::Error> {
    let pageid = &v["query"]["pageids"][0];
    let pageid_str = match pageid {
        Value::String(id) => id,
        _ => "-1",
    };

    match &v["query"]["pages"][pageid_str]["extract"] {
        Value::String(extract) => {
            // format to plain text
            let extract = extract.replace("\\\\", "\\");

            Ok(extract.to_string())
        }
        // ignore non strings
        _ => Ok("This page does not exist anymore".to_string()),
    }
}
