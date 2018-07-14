extern crate scraper;
extern crate reqwest;

use scraper::{ Html, Selector };

fn main() {
    let mut response = reqwest::get("http://www.classicshorts.com/abc/a-d.html").unwrap();
    let document = Html::parse_document(&response.text().unwrap());
    let st_selector = Selector::parse(r#"div[class=storylisting]"#).unwrap();

    for listing in document.select(&st_selector) {
        println!("{:?}", listing.value().attr("onclick"));
    }
}
