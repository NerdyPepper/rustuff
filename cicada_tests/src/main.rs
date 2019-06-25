fn main() {
    let cmd = r#"ls $(pwd)"#;
    let tokens = cicada::cmd_to_tokens(cmd);
    let op = cicada::run(cmd.as_ref());
    println!("{:?}", tokens);
    println!();
    println!("{}", op.stdout);

}
