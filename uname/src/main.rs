use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

extern crate rand;
use rand::prelude::*;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    println!("{}", parse(&input[..]));

}

fn parse( input: &str ) -> String {
    let mut output = String::from(input);
    let keys = vec!['%', 'n', 'v', 'a'];
    let mut counts = vec![];
    for key in keys.iter() {
        let fspec = format!("%{}", key);
        counts.push( input.matches(&fspec[..]).count() );
    }

    let zipper: Vec<_> = keys.iter().zip(counts.iter()).collect();
    for (key, count) in zipper {
        match *key {

            'n' => {
                for _ in 0..*count {
                    let noun = gen_noun();
                    output = output.replacen("%n", &noun[..], 1);
                }
            }

            'v' => {
                for _ in 0..*count {
                    let verb = gen_verb();
                    output = output.replacen("%v", &verb[..], 1);
                }
            }

            'a' => {
                for _ in 0..*count {
                    let adj = gen_adj();
                    output = output.replacen("%a", &adj[..], 1);
                }
            }

            '%' => {
                for _ in 0..*count {
                    output = output.replacen("%%", "%", 1);
                }
            }

            _ => {}

        }
    }
    output
}

fn gen_noun() -> String {
    let mut rng = rand::thread_rng();
    let noun_file = File::open("../assets/nouns.txt").unwrap();
    let mut nouns = vec![];
    for line in BufReader::new(noun_file).lines() {
        nouns.push(line.unwrap());
    }
    format!("{}", &nouns[rng.gen_range(0, nouns.len())])
}

fn gen_verb() -> String {
    let mut rng = rand::thread_rng();
    let verb_file = File::open("../assets/verbs.txt").unwrap();
    let mut verbs = vec![];
    for line in BufReader::new(verb_file).lines() {
        verbs.push(line.unwrap());
    }
    format!("{}", &verbs[rng.gen_range(0, verbs.len())])
}

fn gen_adj() -> String {
    let mut rng = rand::thread_rng();
    let adj_file = File::open("../assets/adjs.txt").unwrap();
    let mut adjs = vec![];
    for line in BufReader::new(adj_file).lines() {
        adjs.push(line.unwrap());
    }
    format!("{}", &adjs[rng.gen_range(0, adjs.len())])
}
