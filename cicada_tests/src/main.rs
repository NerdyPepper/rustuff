fn main() {
    let cmd = r#"ls "good folder" | grep me > file1"#;
    let tokens: () = cicada::cmd_to_tokens(cmd);
    println!("{:?}", tokens);

}
