use hyper::header::{ Header, Range };

fn main() {
    let url = "https://google.com";
    let _req = reqwest::get(url).unwrap();
}

fn handler( start: u64, end: u64, url: &str, filename: &str ) {
    let mut headers = Headers::new();
    headers.set(Range::bytes(start, end));

    let req = reqwest::get(url).unwrap();

}
