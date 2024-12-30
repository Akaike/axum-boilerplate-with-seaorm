use validator::ValidationError;

pub fn validate_title_length(title: &str) -> Result<(), ValidationError> {
    if title.len() < 1 as usize || title.len() > 255 as usize {
        return Err(ValidationError::new(
            "must be between 1 and 255 characters long",
        ));
    }
    Ok(())
}
