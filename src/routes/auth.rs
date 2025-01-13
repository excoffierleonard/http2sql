use crate::errors::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    fn new(password: impl Into<String>) -> Self {
        Self(password.into())
    }

    fn validate(&self) -> Result<(), ApiError> {
        let validations = [
            (self.0.is_empty(), "Password cannot be empty"),
            (
                !self.0.chars().all(|c| c.is_ascii()),
                "Password must contain only ASCII characters",
            ),
            (
                self.0.len() < 12,
                "Password must be at least 12 characters long",
            ),
            (
                self.0.len() > 64,
                "Password must be at most 64 characters long",
            ),
            (
                !self.0.chars().any(|c| c.is_ascii_lowercase()),
                "Password must contain at least one lowercase letter",
            ),
            (
                !self.0.chars().any(|c| c.is_ascii_uppercase()),
                "Password must contain at least one uppercase letter",
            ),
            (
                !self.0.chars().any(|c| c.is_ascii_digit()),
                "Password must contain at least one digit",
            ),
            (
                !self.0.chars().any(|c| !c.is_ascii_alphanumeric()),
                "Password must contain at least one special character",
            ),
        ];

        for (condition, message) in validations {
            if condition {
                return Err(ApiError::InvalidInput(message.to_string()));
            }
        }

        Ok(())
    }

    fn hash(&self) -> Result<String, ApiError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();

        let hash = argon2.hash_password(self.0.as_bytes(), &salt)?.to_string();

        Ok(hash)
    }

    fn verify(&self, hash: &str) -> Result<bool, ApiError> {
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
    fn validate_password() {
        // Test empty password
        assert!(Password::new("").validate().is_err());

        // Test non-ASCII characters
        assert!(Password::new("😀".repeat(12)).validate().is_err());

        // Test exact boundary conditions
        assert!(Password::new("a".repeat(11)).validate().is_err());
        assert!(Password::new("a".repeat(65)).validate().is_err());

        // Test required characters
        assert!(Password::new("abcdefghij1!").validate().is_err());
        assert!(Password::new("ABCDEFGHIJ1!").validate().is_err());
        assert!(Password::new("Abcdefghijk!").validate().is_err());
        assert!(Password::new("Abcdefghijk1").validate().is_err());

        // Test valid passwords
        assert!(Password::new("Abcd123!efgh").validate().is_ok());
        assert!(Password::new("P@ssw0rd585.").validate().is_ok());
        assert!(Password::new("Super$3cret!Pass").validate().is_ok());
    }

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
        // Test a matching password
        let password = Password::new("password");
        let hash = password.hash().unwrap();
        assert!(password.verify(&hash).unwrap());

        // Test a non-matching password
        let different_password = Password::new("different_password");
        let different_hash = different_password.hash().unwrap();
        assert!(!password.verify(&different_hash).unwrap());

        // Test an invalid hash
        let invalid_hash = "random_string";
        assert!(password.verify(&invalid_hash).is_err());
    }
}
