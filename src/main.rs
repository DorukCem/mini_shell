use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, ExitCode},
};

const BUILTINS: [&str; 3] = ["cd", "ls", "exit"];

fn main() -> ExitCode {
    init();

    main_loop();

    return ExitCode::SUCCESS;
}

fn init() {}
fn main_loop() {
    loop {
        let wd = std::env::current_dir().expect("Error getting current working directory");
        print!("{} > ", wd.to_str().unwrap());
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Error getting line from standart input");

        let args = parse_args(line);

        let status: i32 = execute(args);
        if status == 0 {
            break;
        }
    }
}

fn execute(args: Args) -> i32 {
    if args.command != "" {
        if BUILTINS.contains(&args.command.as_str()) {
            execute_built_in(args)
        } else {
            launch(args)
        }
    } else {
        return 1;
    }
}

fn launch(args: Args) -> i32 {
    let mut child = Command::new(args.command)
        .args(args.args)
        .spawn()
        .expect("failed to spawn the target process"); // TODO insead of panic print to standart output

    let _result = child.wait().unwrap(); // TODO insead of panic print to standart output

    return 1;
}

fn execute_built_in(args: Args) -> i32 {
    match args.command.as_str() {
        "cd" => execute_cd(args.args),
        "ls" => execute_ls(args.args),
        "exit" => 0,
        _ => panic!("unkown command"), // TODO
    }
}

fn execute_cd(args: Vec<String>) -> i32 {
    if let Some(dir_name) = args.first() {
        match std::env::set_current_dir(Path::new(dir_name)) {
            Ok(_) => (),
            Err(_) => eprintln!("cd {dir_name}: No such file or directory"),
        }
    }
    1
}
fn execute_ls(_args: Vec<String>) -> i32 {
    let wd = std::env::current_dir().expect("Error getting current working directory");
    let dirs: Vec<_> = fs::read_dir(wd)
        .expect("Error reading directories")
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|dir_entry| {
            dir_entry
                .unwrap()
                .path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned()
        })
        .collect();

    for dir in dirs {
        print!("{}  ", dir)
    }
    print!("\n");
    io::stdout().flush().unwrap();

    1
}

struct Args {
    command: String,
    args: Vec<String>,
}

fn parse_args(line: String) -> Args {
    let args: Vec<_> = line.split_whitespace().map(|s| s.to_string()).collect();
    let command = args.get(0).cloned().unwrap_or_default();
    let rest = args.into_iter().skip(1).collect();

    Args {
        command,
        args: rest,
    }
}
