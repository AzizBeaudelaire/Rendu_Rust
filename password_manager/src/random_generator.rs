use rand::rng;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;


pub fn generate_password(length: usize) -> String {
    assert!(length >= 8, "Le mot de passe doit faire au moins 8 caractères");

    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &str = "0123456789";
    const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}<>?,.";

     let mut password_chars = vec![
        LOWERCASE.chars().collect::<Vec<_>>().into_iter().choose(&mut rng()).unwrap(),
        UPPERCASE.chars().collect::<Vec<_>>().into_iter().choose(&mut rng()).unwrap(),
        DIGITS.chars().collect::<Vec<_>>().into_iter().choose(&mut rng()).unwrap(),
        SYMBOLS.chars().collect::<Vec<_>>().into_iter().choose(&mut rng()).unwrap(),
    ];

    let all_chars: Vec<char> = format!("{LOWERCASE}{UPPERCASE}{DIGITS}{SYMBOLS}").chars().collect();
    let mut rng = rng();
    for _ in password_chars.len()..length {
        password_chars.push(all_chars.iter().choose(&mut rng).copied().unwrap());
    }

    password_chars.shuffle(&mut rng);

    password_chars.into_iter().collect()

}