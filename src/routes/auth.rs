use crate::errors::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, ApiError> {
    let hash = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();

    let is_valid = argon2.verify_password(password.as_bytes(), &hash).is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn standard_hash_password() {
        let password = "password";

        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();

        assert_ne!(password, hash1);
        assert_ne!(password, hash2);
        assert_ne!(hash1, hash2);

        const EXPECTED_LENGTH: usize = 97;

        assert_eq!(hash1.len(), EXPECTED_LENGTH);
        assert_eq!(hash2.len(), EXPECTED_LENGTH);
    }

    #[test]
    fn standard_verify_password() {
        let password = "password";

        let hash = hash_password(password).unwrap();

        let is_valid = verify_password(password, &hash).unwrap();

        assert!(is_valid);
    }
}
