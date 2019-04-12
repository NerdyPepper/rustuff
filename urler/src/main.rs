use url::percent_encoding::{ utf8_percent_encode, DEFAULT_ENCODE_SET };

fn main() {
    let text = "garçon";
    let formatted = utf8_percent_encode(&text, DEFAULT_ENCODE_SET);
    println!("{}\n{}\n{}", text, formatted, formatted.to_string());
}
