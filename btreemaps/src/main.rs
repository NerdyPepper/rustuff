use std::collections::BTreeMap;

fn main() {
    let mut artists: BTreeMap<String, Vec<String>> = BTreeMap::new();

    let alb: String = "alb1".into();
    let songs: Vec<String> = ["s1", "s2", "s3"]
        .iter()
        .map(|&x| x.to_string())
        .collect();
    artists.insert(alb, songs);
    for (k, v) in artists {
        println!("{} {:?}", k, v);
    };
}

