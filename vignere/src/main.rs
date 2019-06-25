use std::collections::HashMap;

fn main() {
    println!("{}", encrypt("ATTACKATDAWN", "LEMON"));
    println!("{}", decrypt("GCYCZFMLYLEIM", "AYUSH"));
}

fn encrypt(plaintext: &str, key: &str) -> String {
    let alphs: HashMap<_, _> = (b'A'..b'Z')
        .map(|c| c as char)
        .enumerate()
        .collect();

    let p_bytes: Vec<u8> = plaintext.as_bytes().iter().map(|b| b - 65).collect();
    let k_bytes: Vec<u8> = key.as_bytes().iter().map(|b| b - 65).collect();
    let mut ciphertext = vec![];

    for i in 0..p_bytes.len() {
        ciphertext.push( (p_bytes[i] + k_bytes[i % k_bytes.len()]) % 26 );
    }

    ciphertext.iter()
        .map(|byte| alphs.get(&(*byte as usize)).unwrap())
        .collect::<String>()
}

fn decrypt(ciphertext: &str, key: &str) -> String {
    let alphs: HashMap<_, _> = (b'A'..b'Z')
        .map(|c| c as char)
        .enumerate()
        .collect();

    let c_bytes: Vec<u8> = ciphertext.as_bytes().iter().map(|b| b - 65).collect();
    let k_bytes: Vec<u8> = key.as_bytes().iter().map(|b| b - 65).collect();
    let mut plaintext = vec![];

    for i in 0..c_bytes.len() {
        plaintext.push( (c_bytes[i] + 26 - k_bytes[i % k_bytes.len()]) % 26 );
    }

    plaintext.iter()
        .map(|byte| alphs.get(&(*byte as usize)).unwrap())
        .collect::<String>()
}

