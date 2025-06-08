//! # Error Types and Handling
//!
//! This module defines all error types used throughout the library, following
//! the new architecture error message formatting and structure.

use thiserror::Error;

/// Primary error type for parsing operations
///
/// All error messages follow the format: "Failed to <action>: <reason>"
/// or "Invalid <target>: <details>" for validation errors.
#[derive(Error, Debug)]
pub enum ParseError {
    // CSS Selector related errors
    #[error("Failed to create CSS selector '{selector}': {context}")]
    SelectorCreationFailed { selector: String, context: String },

    #[error("Failed to find element with selector '{selector}': {context}")]
    ElementNotFound { selector: String, context: String },

    #[error("Failed to extract attribute '{attribute}' from element: {context}")]
    AttributeExtractionFailed { attribute: String, context: String },

    // Data extraction related errors
    #[error("Failed to extract {data_type}: {reason}")]
    DataExtractionFailed { data_type: String, reason: String },

    #[error("Failed to parse {data_type}: {value}")]
    DataParsingFailed { data_type: String, value: String },

    #[error("Invalid {data_type} format: {details}")]
    InvalidDataFormat { data_type: String, details: String },

    // HTML/DOM related errors
    #[error("Failed to parse HTML content: {reason}")]
    HtmlParsingFailed { reason: String },

    #[error("Invalid HTML structure: expected {expected}")]
    InvalidHtmlStructure { expected: String },

    #[error("Failed to process empty HTML content")]
    EmptyHtmlContent,

    // File and I/O related errors
    #[error("Failed to find file: {path}")]
    FileNotFound { path: String },

    #[error("Failed to access file due to permissions: {path}")]
    FilePermissionDenied { path: String },

    #[error("Failed to decode file encoding: {path}")]
    InvalidFileEncoding { path: String },

    // HTTP and network related errors
    #[error("Invalid session ID: {session_id}")]
    InvalidSessionId { session_id: String },

    #[error("Invalid URL format: {url}")]
    InvalidUrl { url: String },

    #[error("Failed to complete request: timeout occurred")]
    RequestTimeout,

    #[error("Failed to process server response: {status}")]
    ServerError { status: String },

    // Configuration and validation errors
    #[error("Failed to find required configuration: {config_name}")]
    MissingConfiguration { config_name: String },

    #[error("Invalid configuration value for '{config_name}': {value}")]
    InvalidConfiguration { config_name: String, value: String },

    #[error("Invalid value range for '{field}': {value} (expected: {range})")]
    ValueOutOfRange {
        field: String,
        value: String,
        range: String,
    },

    // General operation errors
    #[error("Failed to execute operation: {operation} not implemented")]
    NotImplemented { operation: String },

    #[error("Failed to complete operation: {operation} was cancelled")]
    OperationCancelled { operation: String },

    #[error("Failed to process {context}: {message}")]
    UnexpectedError { context: String, message: String },

    // Data validation errors
    #[error("Failed to process empty {data_type} entry")]
    EmptyDataEntry { data_type: String },

    #[error("Failed to find required field: {field_name}")]
    MissingRequiredField { field_name: String },

    // Form processing errors
    #[error("Failed to extract form data: {reason}")]
    FormDataExtractionFailed { reason: String },

    #[error("Failed to find form element: {form_selector}")]
    FormElementNotFound { form_selector: String },

    // Table processing errors
    #[error("Failed to extract table data: {reason}")]
    TableDataExtractionFailed { reason: String },

    #[error("Failed to process table row: {row_context}")]
    TableRowProcessingFailed { row_context: String },

    // Navigation processing errors
    #[error("Failed to extract navigation data: {reason}")]
    NavigationExtractionFailed { reason: String },

    #[error("Failed to process pagination: {reason}")]
    PaginationProcessingFailed { reason: String },
}

impl ParseError {
    // Convenience constructors following the naming guidelines

    /// Create attribute extraction failure error
    pub fn attribute_extraction_failed(attribute: &str, context: &str) -> Self {
        Self::AttributeExtractionFailed {
            attribute: attribute.to_string(),
            context: context.to_string(),
        }
    }

    /// Create selector creation failure error
    pub fn selector_creation_failed(selector: &str, context: &str) -> Self {
        Self::SelectorCreationFailed {
            selector: selector.to_string(),
            context: context.to_string(),
        }
    }

    /// Create element not found error
    pub fn element_not_found(selector: &str, context: &str) -> Self {
        Self::ElementNotFound {
            selector: selector.to_string(),
            context: context.to_string(),
        }
    }

    /// Create data extraction failure error
    pub fn data_extraction_failed(data_type: &str, reason: &str) -> Self {
        Self::DataExtractionFailed {
            data_type: data_type.to_string(),
            reason: reason.to_string(),
        }
    }

    /// Create data parsing failure error
    pub fn data_parsing_failed(data_type: &str, value: &str) -> Self {
        Self::DataParsingFailed {
            data_type: data_type.to_string(),
            value: value.to_string(),
        }
    }

    /// Create empty data entry error
    pub fn empty_data_entry(data_type: &str) -> Self {
        Self::EmptyDataEntry {
            data_type: data_type.to_string(),
        }
    }

    /// Create empty HTML content error
    pub fn empty_html_content() -> Self {
        Self::EmptyHtmlContent
    }

    /// Create unexpected error
    pub fn unexpected_error(context: &str, message: &str) -> Self {
        Self::UnexpectedError {
            context: context.to_string(),
            message: message.to_string(),
        }
    }

    /// Create validation failure error
    pub fn validation_failed(message: &str) -> Self {
        Self::UnexpectedError {
            context: "Validation".to_string(),
            message: message.to_string(),
        }
    }

    /// Create missing required field error
    pub fn missing_required_field(field_name: &str) -> Self {
        Self::MissingRequiredField {
            field_name: field_name.to_string(),
        }
    }
}
