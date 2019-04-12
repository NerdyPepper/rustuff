use std::process::Command;

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Kevin K. <kbknapp@gmail.com>")
        .about("Does awesome things")
        .args_from_usage("-c, --config=[FILE] 'Sets a custom config file'
                                         <output> 'Sets an optional output file'
                                         -d... 'Turn debugging information on'")
        .subcommand(SubCommand::with_name("test")
                    .about("does testing things")
                    .arg_from_usage("-l, --list 'lists test values'"))
        .get_matches();
}


