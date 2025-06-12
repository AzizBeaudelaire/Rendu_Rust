use std::io::{self, Write};

pub fn run_cli() {
    loop {
        println!("\nGestionnaire de mots de passe");
        println!("1. Ajouter un mot de passe");
        println!("2. Afficher les mots de passe");
        println!("3. Quitter");

        let choice = match get_user_input("Veuillez entrer votre choix:") {
            Ok(input) => match input.trim().parse::<u32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("Veuillez entrer un nombre valide.");
                    continue;
                }
            },
            Err(e) => {
                println!("Erreur: {}", e);
                continue;
            }
        };

        match choice {
            1 => {
                if let Err(e) = add_password() {
                    println!("Erreur lors de l'ajout du mot de passe: {}", e);
                }
            },
            2 => {
                if let Err(e) = display_passwords() {
                    println!("Erreur lors de l'affichage des mots de passe: {}", e);
                }
            },
            3 => {
                println!("Au revoir!");
                break;
            },
            _ => println!("Option non valide, veuillez réessayer."),
        }
    }
}

fn get_user_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}