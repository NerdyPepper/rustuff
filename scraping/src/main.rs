extern crate reqwest;
extern crate serde_json;

use serde_json::Value;
use reqwest::Response;

fn main() {
    let url = "http://en.wikipedia.org/w/api.php?action=query&format=json&prop=extracts%7Clinks&indexpageids=1&titles=TTY&redirects=1&exlimit=20&explaintext=1&plnamespace=";
    let mut res = reqwest::get(url).unwrap();

    match get_links(&mut res) {
        Ok(x) => println!("{:?}", x),
        Err(_) => panic!("Oopsies"),
    };
}

pub fn query_url_gen(title: &str) -> String {
    // query config
    let mut url = String::from("https://en.wikipedia.org");
    url.push_str("/w/api.php?");
    url.push_str("action=query&");
    url.push_str("format=json&");
    url.push_str("prop=extracts&7Clinks&");
    url.push_str("indexpageids=1&");
    url.push_str("titles=");
    url.push_str(title);
    url.push_str("&");
    url.push_str("redirects=1&");
    url.push_str("explaintext=1");
    url
}

pub fn get_links(res: &mut Response) -> Result<Vec<String>, reqwest::Error> {
    let v: Value = match serde_json::from_str(&res.text()?) {
        Ok(x) => x,
        Err(x) => panic!("Failed to parse json\nReceived error {}", x),
    };

    let pageid = &v["query"]["pageids"][0];
    let pageid_str = match pageid {
        Value::String(id) => id,
        _ => panic!("wut"),
    };

    let mut links = vec![];
    match &v["query"]["pages"][pageid_str]["links"] {
        Value::Array(arr) => {
            for item in arr {
                match item["title"] {
                    Value::String(ref title) => links.push(title.to_string()),
                    _ => links.push(String::from("in da loop"))
                }
            }
        },
        _ => println!("{:#}", &v["query"]["pages"][pageid_str])
    };

    Ok(links)
}
