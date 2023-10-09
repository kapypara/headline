use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
        Error
    },
    Argon2
};

fn hash_password(password: &str) -> Result<String, Error> {

    let salt = SaltString::generate(&mut OsRng);

    Ok(
        Argon2::default().hash_password(password.as_bytes(), &salt)?
        .to_string()
    )
}

/// true when password is correct, false when not, error when something goes wrong
fn verify_password(pass: String, hash: String) -> Result<bool, Error> {

    let parsed_hash = PasswordHash::new(&hash)?;

    match Argon2::default().verify_password(pass.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(Error::Password) => Ok(false),
        Err(err) => Err(err),
    }
}
