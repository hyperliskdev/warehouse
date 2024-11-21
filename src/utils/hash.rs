use argon2::{
    self,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use crate::routes::auth::AuthError;

// Hash a password using Argon2
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    // Generate a salt for this password
    let salt = SaltString::generate(&mut rand::thread_rng());

    // Create a new Argon2 hasher with default values
    let argon2 = Argon2::default();

    // Hash the password with the generated salt
    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::HashError("Failed to hash password".to_string()))?;

    // Return the hashed password as a string
    Ok(hashed_password.to_string())
}

// Verify a password against a hash using Argon2
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AuthError> {
    // Parse the hash into a PasswordHash
    let parsed_hash = PasswordHash::new(hash).map_err(|_| AuthError::InvalidHash)?;

    // Create a new Argon2 verifier
    let argon2 = Argon2::default();

    // Verify the password against the hash
    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| AuthError::InvalidPassword)
        .map(|_| true)
}
