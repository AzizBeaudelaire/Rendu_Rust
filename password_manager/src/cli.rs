use std::io::{self, Write};

pub fn run_cli() {
    loop {
        println!("\nGestionnaire de mots de passe");
        println!("1. Générer un mot de passe");
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
                if let Err(e) = generate_password_cli() {
                    println!("Erreur lors de la génération du mot de passe: {}", e);
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

    let password = crate::random_generator::generate_password(length);
    println!("Mot de passe généré: {}", password);

    Ok(())
}

fn display_passwords() -> io::Result<()> {
    // Simuler l'affichage des mots de passe
    println!("Affichage des mots de passe stockés:");
    println!("1. Mot de passe pour exemple.com: Ex@mple1");
    println!("2. Mot de passe pour test.com: T3st!ng2");
    println!("3. Mot de passe pour demo.com: D3m0P@ss");

    Ok(())
}
