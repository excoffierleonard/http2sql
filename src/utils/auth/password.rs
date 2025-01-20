use crate::errors::ApiError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, SaltString},
    Argon2, PasswordHasher, PasswordVerifier,
};

#[derive(Debug, Clone)]
pub struct Password(String);

impl Password {
    pub fn new(password: impl Into<String>) -> Result<Self, ApiError> {
        let password = password.into();
        let password = Password(password);
        password.validate()?;
        Ok(password)
    }

    fn validate(&self) -> Result<&Self, ApiError> {
        let validations = [
            (self.0.is_empty(), "Password cannot be empty"),
            (
                !self.0.is_ascii(),
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

        Ok(self)
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
    fn create_password() {
        let password = Password::new("Randompassword4!").unwrap().hash().unwrap();
        println!("{:?}", password);
    }

    #[test]
    fn validate_password() {
        // Test empty password
        assert!(Password::new("").is_err());

        // Test non-ASCII characters
        assert!(Password::new("😀".repeat(12)).is_err());

        // Test exact boundary conditions
        assert!(Password::new("a".repeat(11)).is_err());
        assert!(Password::new("a".repeat(65)).is_err());

        // Test required characters
        assert!(Password::new("abcdefghij1!").is_err());
        assert!(Password::new("ABCDEFGHIJ1!").is_err());
        assert!(Password::new("Abcdefghijk!").is_err());
        assert!(Password::new("Abcdefghijk1").is_err());

        // Test valid passwords
        assert!(Password::new("Abcd123!efgh").is_ok());
        assert!(Password::new("P@ssw0rd585.").is_ok());
        assert!(Password::new("Super$3cret!Pass").is_ok());
    }

    #[test]
    fn hash_password() {
        let password = Password::new("Randompassword1!").unwrap();

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
        let password = Password::new("Randompassword1!").unwrap();
        let hash = password.hash().unwrap();
        assert!(password.verify(&hash).unwrap());

        // Test a non-matching password
        let different_password = Password::new("Randompassword2!").unwrap();
        let different_hash = different_password.hash().unwrap();
        assert!(!password.verify(&different_hash).unwrap());

        // Test an invalid hash
        let invalid_hash = "random_string";
        assert!(password.verify(&invalid_hash).is_err());
    }
}
