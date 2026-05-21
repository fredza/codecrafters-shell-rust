use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

fn is_builtin(command: &str) -> bool {
    matches!(command, "cd" | "echo" | "exit" | "pwd" | "type")
}

fn find_executable(command: &str) -> Option<String> {
    let path_var = env::var("PATH").unwrap_or_default();

    for dir in path_var.split(':') {
        let full_path = format!("{}/{}", dir, command);
        let path = Path::new(&full_path);

        if path.exists() {
            if let Ok(metadata) = fs::metadata(path) {
                let permissions = metadata.permissions();
                if permissions.mode() & 0o111 != 0 {
                    return Some(full_path);
                }
            }
        }
    }

    None
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];

        if command == "exit" {
            break;
        } else if command == "echo" {
            let args = &parts[1..];
            println!("{}", args.join(" "));
        } else if command == "pwd" {
            match env::current_dir() {
                Ok(path) => println!("{}", path.display()),
                Err(e) => eprintln!("pwd: {}", e),
            }
        } else if command == "cd" {
            let raw = parts.get(1).copied().unwrap_or("~");
            // Expand ~ to the value of $HOME
            let target = if raw == "~" {
                env::var("HOME").unwrap_or_else(|_| String::from("/"))
            } else {
                raw.to_string()
            };
            match env::set_current_dir(&target) {
                Ok(()) => {}
                Err(_) => eprintln!("cd: {}: No such file or directory", target),
            }
        } else if command == "type" {
            if parts.len() < 2 {
                continue;
            }

            let target = parts[1];

            if is_builtin(target) {
                println!("{} is a shell builtin", target);
            } else if let Some(path) = find_executable(target) {
                println!("{} is {}", target, path);
            } else {
                println!("{}: not found", target);
            }
        } else if let Some(executable) = find_executable(command) {
            let args = &parts[1..];
            Command::new(&executable)
                .arg0(command)
                .args(args)
                .status()
                .expect("Failed to execute process");
        } else {
            println!("{}: command not found", command);
        }
    }
}
