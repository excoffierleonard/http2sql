use crate::errors::ApiError;
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::random;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone)]
pub struct ApiKey(String);

impl ApiKey {
    // This constant helps us maintain consistency and makes changes easier
    const PREFIX: &'static str = "ak_prod_";

    // Create a new API key, with a custom input
    pub fn new(api_key: impl Into<String>) -> Result<Self, ApiError> {
        let api_key = ApiKey(api_key.into());
        api_key.validate()?;
        Ok(api_key)
    }

    // Helper method to check if a string matches our API key format
    fn validate(&self) -> Result<&Self, ApiError> {
        let validations = [
            // The API key must start with the correct prefix
            (
                !self.0.starts_with(Self::PREFIX),
                "API key must start with the correct prefix",
            ),
            // The base64 encoded data must be 32 bytes long
            (
                STANDARD
                    .decode(&self.0[Self::PREFIX.len()..])
                    .map_or(true, |decoded| decoded.len() != 32),
                "API key must contain valid base64 encoded data of correct length",
            ),
        ];

        for (condition, message) in validations {
            if condition {
                return Err(ApiError::InvalidInput(message.to_string()));
            }
        }

        Ok(self)
    }

    // Generate a new API key
    pub fn generate() -> Self {
        // A random 32 bytes long string
        let random_bytes: [u8; 32] = random();

        // Encode the random bytes to a base64 string to ensure its url safe
        let secret = STANDARD.encode(random_bytes);

        // The API key is the concatenation of a prefix and the secret
        let api_key = format!("{}{}", Self::PREFIX, secret);

        Self(api_key)
    }

    // Deterministic Hash function
    pub fn hash(&self) -> String {
        // Create a SHA-256 hasher
        let mut hasher = Sha256::new();

        // Update the hasher with the API key
        hasher.update(self.0.as_bytes());

        // Finalize the hasher and return the hash as a hexadecimal string
        format!("{:x}", hasher.finalize())
    }

    // Method to get the string value
    #[cfg(test)]
    fn as_str(&self) -> &str {
        &self.0
    }

    // Method to get the owned string
    pub fn into_string(self) -> String {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_key_generation_is_random() {
        let api_key1 = ApiKey::generate();
        let api_key2 = ApiKey::generate();

        assert_ne!(api_key1.as_str(), api_key2.as_str());
    }

    #[test]
    fn api_key_has_constant_prefix() {
        let api_key1 = ApiKey::generate();
        let api_key2 = ApiKey::generate();

        assert_eq!(&api_key1.as_str()[..ApiKey::PREFIX.len()], ApiKey::PREFIX);
        assert_eq!(&api_key2.as_str()[..ApiKey::PREFIX.len()], ApiKey::PREFIX);
    }

    #[test]
    fn api_key_format_is_valid() {
        // Correct key
        let valid_key = ApiKey::new("ak_prod_YhssYXDTEhrycWESFjjwSorIkL79VzWreI7+NYPSLaU=");
        // Incorrect prefix
        let invalid_key1 = ApiKey::new("invalid_key");
        // Incorrect length
        let invalid_key2 = ApiKey::new("ak_prod_invalid_key");
        // Correct prefix and length but invalid base64 characters
        let invalid_key3 = ApiKey::new("ak_prod_YhssYXDT@hrycWE$FjjwSorIkL79VzWreI7+NYPSLaU=");

        assert!(valid_key.is_ok());
        assert!(invalid_key1.is_err());
        assert!(invalid_key2.is_err());
        assert!(invalid_key3.is_err());
    }

    // That hash function must be deterministic as a standalone way to lookup in the db the corresponding user
    #[test]
    fn api_key_hash_is_deterministic() {
        let api_key = ApiKey::new("ak_prod_YhssYXDTEhrycWESFjjwSorIkL79VzWreI7+NYPSLaU=").unwrap();
        let hash1 = api_key.hash();
        let hash2 = api_key.hash();

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn validate_works_with_generated_keys() {
        let api_key = ApiKey::generate();
        assert!(api_key.validate().is_ok());
    }
}
