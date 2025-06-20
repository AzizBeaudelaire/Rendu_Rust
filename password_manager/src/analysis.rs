use std::thread;
use std::sync::{Arc, Mutex};
const NTHREADS : u32 = 8;

pub fn validate_password_mt(password: &str) -> Result<(), Vec<&'static str>> {
    let pw = Arc::new(password.to_string());
    let errors = Arc::new(Mutex::new(vec![]));
    let mut handles = Vec::with_capacity(NTHREADS);

    //Check de la longueur du mdp
    {
        let pw = Arc::clone(&pw)
        let errors = Arc::clone(&errors)
        handles.push(thread::spawn(move || {
            if pw.len() < 8 {
                errors.lock().unwrap().psuh("Le mot de passe doit contenir au moins 8 caractères !");
            }
        }));

    }

     // Check de la présence d'une minuscule
    {
        let pw = Arc::clone(&pw);
        let errors = Arc::clone(&errors);
        handles.push(thread::spawn(move || {
            if !pw.chars().any(|c| c.is_lowercase()) {
                errors.lock().unwrap().push("Le mot de passe doit contenir au moins une minuscule.");
            }
        }));
    }

    // Check de la présence d'une majuscule
    {
        let pw = Arc::clone(&pw);
        let errors = Arc::clone(&errors);
        handles.push(thread::spawn(move || {
            if !pw.chars().any(|c| c.is_uppercase()) {
                errors.lock().unwrap().push("Le mot de passe doit contenir au moins une majuscule.");
            }
        }));
    }

    // Check de la présence d'un chiffre
    {
        let pw = Arc::clone(&pw);
        let errors = Arc::clone(&errors);
        handles.push(thread::spawn(move || {
            if !pw.chars().any(|c| c.is_ascii_digit()) {
                errors.lock().unwrap().push("Le mot de passe doit contenir au moins un chiffre.");
            }
        }));
    }

    // Check de la présence d'un symbole
    {
        let pw = Arc::clone(&pw);
        let errors = Arc::clone(&errors);
        handles.push(thread::spawn(move || {
            if !pw.chars().any(|c| !c.is_alphanumeric()) {
                errors.lock().unwrap().push("Le mot de passe doit contenir au moins un caractère spécial.");
            }
        }));
    }

    // Attente de tous les threads
    for handle in handles {
        handle.join().unwrap();S
    }

    let errors = Arc::try_unwrap(errors).unwrap().into_inner().unwrap();
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}





