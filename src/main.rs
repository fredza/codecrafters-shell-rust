use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

fn is_builtin(command: &str) -> bool {
    matches!(command, "echo" | "exit" | "type")
}

fn find_executable(command: &str) -> Option<String> {
    // Récupère PATH
    let path_var = env::var("PATH").unwrap_or_default();

    // Parcourt chaque dossier du PATH
    for dir in path_var.split(':') {
        let full_path = format!("{}/{}", dir, command);

        let path = Path::new(&full_path);

        // Vérifie que le fichier existe
        if path.exists() {
            // Récupère les métadonnées
            if let Ok(metadata) = fs::metadata(path) {
                let permissions = metadata.permissions();

                // Vérifie le bit exécutable Unix
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
        // Prompt
        print!("$ ");
        io::stdout().flush().unwrap();

        // Lecture utilisateur
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        // Ignore ligne vide
        if input.is_empty() {
            continue;
        }

        // Découpe commande + arguments
        let parts: Vec<&str> = input.split_whitespace().collect();

        let command = parts[0];

        // exit
        if command == "exit" {
            break;
        }
        // echo
        else if command == "echo" {
            let args = &parts[1..];
            println!("{}", args.join(" "));
        }
        // type
        else if command == "type" {
            // Vérifie qu'un argument existe
            if parts.len() < 2 {
                continue;
            }

            let target = parts[1];

            // Builtin ?
            if is_builtin(target) {
                println!("{} is a shell builtin", target);
            }
            // Cherche dans PATH
            else if let Some(path) = find_executable(target) {
                println!("{} is {}", target, path);
            }
            // Introuvable
            else {
                println!("{}: not found", target);
            }
        }
        // Commande externe ou inconnue
        else if let Some(executable) = find_executable(command) {
            let args = &parts[1..];
            Command::new(&executable)
                .args(args)
                .status()
                .expect("Failed to execute process");
        }
        // Commande inconnue
        else {
            println!("{}: command not found", command);
        }
    }
}
