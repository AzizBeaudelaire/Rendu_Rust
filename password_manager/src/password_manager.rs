use std::collections::HashMap;
use std::fs;
use std::io;
use serde_json;
use crate::analysis::validate_password_mt;


pub fn add_password(username: &str, password: &str, path: &str) -> io::Result<()> {
    // Validation avant de sauvegarder
    match validate_password_mt(password) {
        Ok(_) => {
            let mut map: HashMap<String, String> = if fs::metadata(path).is_ok() {
                let content = fs::read_to_string(path)?;
                serde_json::from_str(&content).unwrap_or_default()
            } else {
                HashMap::new()
            };

            map.insert(username.to_string(), password.to_string());

            let json = serde_json::to_string_pretty(&map)?;
            fs::write(path, json)?;

            Ok(())
        }
        Err(errors) => {
            eprintln!("Mot de passe invalide pour l'utilisateur {username} :");
            for err in errors {
                eprintln!(" - {err}");
            }
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Mot de passe invalide"))
        }
    }
}


pub fn get_password(username: &str, path: &str) -> io::Result<Option<String>> {
    if let Ok(content) = fs::read_to_string(path) {
        let map: HashMap<String, String> = serde_json::from_str(&content)?;
        Ok(map.get(username).cloned())
    } else {
        Ok(None)
    }
}

pub fn delete_password(username: &str, path: &str) -> io::Result<()> {
    if let Ok(content) = fs::read_to_string(path) {
        let mut map: HashMap<String, String> = serde_json::from_str(&content)?;
        map.remove(username);

        let json = serde_json::to_string_pretty(&map)?;
        fs::write(path, json)?;
    }

    Ok(())
}
    
