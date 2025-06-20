use rand::seq::{IteratorRandom, SliceRandom};

pub fn generate_password(length: usize) -> String {
    assert!(length >= 8, "Le mot de passe doit faire au moins 8 caractères");

    const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
    const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const DIGITS: &str = "0123456789";
    const SYMBOLS: &str = "!@#$%^&*()-_=+[]{}<>?,.";

    let mut rng = rand::thread_rng();

    let mut password_chars = vec![
        LOWERCASE.chars().choose(&mut rng).unwrap(),
        UPPERCASE.chars().choose(&mut rng).unwrap(),
        DIGITS.chars().choose(&mut rng).unwrap(),
        SYMBOLS.chars().choose(&mut rng).unwrap(),
    ];

    let all_chars: Vec<char> = format!("{}{}{}{}", LOWERCASE, UPPERCASE, DIGITS, SYMBOLS).chars().collect();

    for _ in password_chars.len()..length {
        password_chars.push(*all_chars.choose(&mut rng).unwrap());
    }

    password_chars.shuffle(&mut rng);

    password_chars.into_iter().collect()
}
