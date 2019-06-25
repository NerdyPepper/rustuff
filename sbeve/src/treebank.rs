use regex::Regex;

fn starting_quotes() -> Vec<(Regex, String)>{
    let list = vec![
        (Regex::new(r#"^\""#).unwrap(), r'``'),
        (Regex::new(r#""#).unwrap())
    ]
}

fn treebank_tokenizer(input: &str) -> {
}
