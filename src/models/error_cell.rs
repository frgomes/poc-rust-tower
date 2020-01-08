/*
 * PSD2 Demo
 *
 * Demo application for PSD2
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: noreply@example.com
 * Generated by: https://openapi-generator.tech
 */

/// ErrorCell : Error cell.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Response)]
pub struct ErrorCell {
    #[serde(rename = "coord")]
    pub coord: crate::models::Coord,
    #[serde(rename = "error")]
    pub error: crate::models::Error,
}

impl ErrorCell {
    /// Error cell.
    pub fn new(coord: crate::models::Coord, error: crate::models::Error) -> ErrorCell {
        ErrorCell {
            coord,
            error,
        }
    }
}


