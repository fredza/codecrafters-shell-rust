use std::io::{self, Write};

fn main() {
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
        return;
    }

    // Quitte le programme si la commande est exit
    if command == "exit" {
        return;
    }

    // Affiche l'erreur demandée
    println!("{}: command not found", command);
}
