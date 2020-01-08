/*
 * PSD2 Demo
 *
 * Demo application for PSD2
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: noreply@example.com
 * Generated by: https://openapi-generator.tech
 */

/// Error : Contains an error code and its description.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Response)]
pub struct Error {
    /// error code
    #[serde(rename = "code")]
    pub code: i32,
    /// error message
    #[serde(rename = "message")]
    pub message: String,
}

impl Error {
    /// Contains an error code and its description.
    pub fn new(code: i32, message: String) -> Error {
        Error {
            code,
            message,
        }
    }
}

