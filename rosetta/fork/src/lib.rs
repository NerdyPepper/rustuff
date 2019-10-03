pub fn fork_test() {
    use nix::unistd::{ fork, ForkResult };
    match fork() {
        Ok(ForkResult::Parent {child, ..}) => {
            println!("PARENT: Hello, World!");
            println!("Child Process Id: {}", child);
        },
        Ok(ForkResult::Child) => { println!("CHILD: Hello, World!"); },
        Err(_) => { println!("ERR: failed to fork."); }
    }
}
