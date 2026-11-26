#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    InvalidToken,
    Forbidden,
    Internal,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            AuthError::InvalidToken => write!(f, "Missing or invalid token"),
            AuthError::Forbidden => write!(f, "Forbidden"),
            AuthError::Internal => write!(f, "Authentication error"),
        }
    }
}

impl std::error::Error for AuthError {}
