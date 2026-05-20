use std::io::{self, Write};

fn main() {
    loop {
        // Affiche le prompt
        print!("$ ");

        // Force l'affichage immédiat du prompt
        io::stdout().flush().unwrap();

        // Variable qui contiendra l'entrée utilisateur
        let mut input = String::new();

        // Lit la ligne tapée
        io::stdin()
            .read_line(&mut input)
            .expect("Erreur lecture stdin");

        // Supprime les espaces et le retour à la ligne
        let command = input.trim();

        // Ignore une ligne vide
        if command.is_empty() {
            continue;
        }

        // Quitte le shell si l'utilisateur tape "exit"
        if command == "exit" {
            break;
        }

        // Affiche l'erreur si la commande n'existe pas
        println!("{}: command not found", command);
    }
}
