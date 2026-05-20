use std::io::{self, Write};

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

        // Ignore les lignes vides
        if input.is_empty() {
            continue;
        }

        // Découpe la commande en morceaux
        let parts: Vec<&str> = input.split_whitespace().collect();

        let command = parts[0];

        // Builtin exit
        if command == "exit" {
            break;
        }

        // Builtin echo
        if command == "echo" {
            // Affiche tous les arguments après "echo"
            let args = &parts[1..];

            println!("{}", args.join(" "));
        } else if command == "type" {
            // Builtin type
            if parts.len() < 2 {
                continue;
            }

            let target = parts[1];

            if target == "exit" || target == "echo" || target == "type" {
                println!("{} is a shell builtin", target);
            } else {
                println!("{}: not found", target);
            }
        } else {
            // Commande inconnue
            println!("{}: command not found", command);
        }
    }
}
