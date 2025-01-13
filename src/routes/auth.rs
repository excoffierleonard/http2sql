use crate::errors::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: impl Into<String>) -> Self {
        Self(password.into())
    }

    pub fn hash(&self) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2.hash_password(self.0.as_bytes(), &salt)?.to_string();

        Ok(hash)
    }

    pub fn verify(&self, hash: &str) -> Result<bool, ApiError> {
        let hash = PasswordHash::new(hash)?;
        let argon2 = Argon2::default();

        let is_valid = argon2.verify_password(self.0.as_bytes(), &hash).is_ok();

        Ok(is_valid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password() {
        let password = Password::new("password");

        let hash1 = password.hash().unwrap();
        let hash2 = password.hash().unwrap();

        // Ensure that the password is not stored in plain text
        assert_ne!(password.0, hash1);
        assert_ne!(password.0, hash2);

        // Ensure that the hashing function is not deterministic
        assert_ne!(hash1, hash2);

        // Ensure that the hash is of constant and expected length
        const EXPECTED_LENGTH: usize = 97;
        assert_eq!(hash1.len(), EXPECTED_LENGTH);
        assert_eq!(hash2.len(), EXPECTED_LENGTH);
    }

    #[test]
    fn verify_password() {
        let password = Password::new("password");
        let hash = password.hash().unwrap();

        let is_valid = password.verify(&hash).unwrap();

        // Ensure that the password verification is successful
        assert!(is_valid);
    }
}
