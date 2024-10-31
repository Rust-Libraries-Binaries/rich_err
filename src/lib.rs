//! `rich_err` - A library for rich, contextual error reporting in Rust.
//!
//! This library aims to simplify and enhance error handling by allowing you to
//! add meaningful, structured context to errors for easier debugging and analysis.

/// A custom error type that allows for rich context.
#[derive(Debug)]
pub struct RichError {
    message: String,
    context: Vec<String>,
}

impl RichError {
    /// Creates a new `RichError` with a base message.
    pub fn new(message: &str) -> Self {
        RichError {
            message: message.to_string(),
            context: Vec::new(),
        }
    }

    /// Adds context to the error message.
    pub fn with_context(mut self, context: &str) -> Self {
        self.context.push(context.to_string());
        self
    }

    /// Formats the full error message with all context included.
    pub fn to_string_with_context(&self) -> String {
        let mut full_message = self.message.clone();
        for ctx in &self.context {
            full_message.push_str(&format!("\nContext: {}", ctx));
        }
        full_message
    }
}

// Unit tests to ensure the `RichError` functionality works as expected.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rich_error_creation() {
        let error = RichError::new("An error occurred");
        assert_eq!(error.message, "An error occurred");
        assert!(error.context.is_empty());
    }

    #[test]
    fn test_adding_context() {
        let error = RichError::new("An error occurred")
            .with_context("Failed to open file")
            .with_context("Permission denied");

        assert_eq!(error.context.len(), 2);
        assert_eq!(error.context[0], "Failed to open file");
        assert_eq!(error.context[1], "Permission denied");

        let formatted_message = error.to_string_with_context();
        assert!(formatted_message.contains("An error occurred"));
        assert!(formatted_message.contains("Context: Failed to open file"));
        assert!(formatted_message.contains("Context: Permission denied"));
    }
}
