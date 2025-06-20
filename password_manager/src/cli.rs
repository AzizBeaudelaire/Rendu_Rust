use std::io::{self, Write};
use crate::password_manager;
use crate::random_generator::generate_password;

pub fn run_cli() {
    let path = "data.json";

    loop {
        println!("\nGestionnaire de mots de passe");
        println!("1. Générer un mot de passe");
        println!("2. Ajouter un mot de passe");
        println!("3. Afficher un mot de passe");
        println!("4. Supprimer un mot de passe");
        println!("5. Quitter");

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
                if let Err(e) = generate_password_cli() {
                    println!("Erreur lors de la génération du mot de passe: {}", e);
                }
            },
            2 => {
                let username = get_user_input("Entrez le nom d'utilisateur:").unwrap().trim().to_string();
                let password = get_user_input("Entrez le mot de passe:").unwrap().trim().to_string();
                if let Err(e) = password_manager::add_password(&username, &password, path) {
                    println!("Erreur lors de l'ajout du mot de passe: {}", e);
                } else {
                    println!("Mot de passe ajouté avec succès.");
                }
            },
            3 => {
                let username = get_user_input("Entrez le nom d'utilisateur:").unwrap().trim().to_string();
                match password_manager::get_password(&username, path) {
                    Ok(Some(password)) => println!("Mot de passe: {}", password),
                    Ok(None) => println!("Utilisateur non trouvé."),
                    Err(e) => println!("Erreur lors de la récupération du mot de passe: {}", e),
                }
            },
            4 => {
                let username = get_user_input("Entrez le nom d'utilisateur à supprimer:").unwrap().trim().to_string();
                if let Err(e) = password_manager::delete_password(&username, path) {
                    println!("Erreur lors de la suppression du mot de passe: {}", e);
                } else {
                    println!("Utilisateur supprimé avec succès.");
                }
            },
            5 => {
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

fn generate_password_cli() -> io::Result<()> {
    let length = match get_user_input("Entrez la longueur du mot de passe (au moins 8 caractères):")?.trim().parse::<usize>() {
        Ok(num) if num >= 8 => num,
        Ok(_) => {
            println!("La longueur doit être d'au moins 8 caractères.");
            return Ok(());
        },
        Err(_) => {
            println!("Veuillez entrer un nombre valide.");
            return Ok(());
        }
    };

    let password = generate_password(length);
    println!("Mot de passe généré: {}", password);

    Ok(())
}
