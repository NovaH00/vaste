use super::errors::UserServiceError;

pub fn validate_username(username: &str) -> Result<(), UserServiceError> {
    if username.len() < 3 {
        return Err(UserServiceError::InvalidUsername(
            "username must be at least 3 characters".to_string(),
        ));
    }
    if username.len() > 32 {
        return Err(UserServiceError::InvalidUsername(
            "username must be at most 32 characters".to_string(),
        ));
    }
    if !username
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(UserServiceError::InvalidUsername(
            "username can only contain letters, numbers, underscores, and hyphens".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), UserServiceError> {
    if !email.contains('@') || !email.contains('.') {
        return Err(UserServiceError::InvalidEmail(
            "email must contain '@' and a domain".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_password(password: &str) -> Result<(), UserServiceError> {
    if password.len() < 8 {
        return Err(UserServiceError::WeakPassword(
            "password must be at least 8 characters".to_string(),
        ));
    }
    if password.len() > 128 {
        return Err(UserServiceError::WeakPassword(
            "password must be at most 128 characters".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(UserServiceError::WeakPassword(
            "password must contain at least one uppercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(UserServiceError::WeakPassword(
            "password must contain at least one lowercase letter".to_string(),
        ));
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(UserServiceError::WeakPassword(
            "password must contain at least one number".to_string(),
        ));
    }
    Ok(())
}
