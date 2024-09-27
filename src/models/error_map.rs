use std::collections::HashMap;
use std::fmt;

/// # ErrorMap
///
/// A struct that holds a HashMap of custom error messages. The usecase for this is,
/// to collect errors from a function and return them all at once.
pub struct ErrorMap {
    pub error_message: HashMap<String, String>,
}

impl std::fmt::Display for ErrorMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (key, value) in &self.error_message {
            write!(f, "{}: {} ", key, value)?;
        }
        Ok(())
    }
}

impl ErrorMap {
    pub fn new() -> Self {
        Self {
            error_message: HashMap::new(),
        }
    }

    pub fn default(msg: Option<String>) -> Self {
        let mut error_message = HashMap::new();

        match msg {
            Some(m) => {
                error_message.insert("Error".to_string(), m);
            }
            None => {
                error_message.insert("Error".to_string(), "An error occurred".to_string());
            }
        }

        Self { error_message }
    }
}
