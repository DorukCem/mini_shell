use std::{
    io::{self, Write},
    process::{Command, ExitCode},
};

use nix::{
    libc::execvp,
    sys::wait::waitpid,
    unistd::{fork, write, ForkResult},
};

const BUILTINS: [&str; 2] = ["cd", "ls"];

fn main() -> ExitCode {
    init();

    main_loop();

    return ExitCode::SUCCESS;
}

fn init() {}
fn main_loop() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error getting line from standart input");

        let args: Vec<_> = line.split_whitespace().collect();

        let status: i32 = execute(args);

        println!("{line}")
    }
}

fn execute(args: Vec<&str>) -> i32 {
    if let Some(head) = args.first() {
        if BUILTINS.contains(head) {
            execute_built_in(args)
        } else {
            launch(args)
        }
    } else {
        return 1;
    }
}

fn execute_built_in(args: Vec<&str>) -> i32 {
    todo!()
}

fn launch(args: Vec<&str>) -> i32 {
    let mut child = Command::new(args[0])
        .args(&args[1..])
        .spawn()
        .expect("failed to spawn the target process"); // TODO insead of panic print to standart output

    let _result = child.wait().unwrap(); // TODO insead of panic print to standart output

    return 1;
}
