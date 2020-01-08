/*
 * PSD2 Demo
 *
 * Demo application for PSD2
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: noreply@example.com
 * Generated by: https://openapi-generator.tech
 */

/// ErrorTable : Error table.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Response)]
pub struct ErrorTable {
    /// Table name.
    #[serde(rename = "name")]
    pub name: String,
    /// List of cells.
    #[serde(rename = "cells")]
    pub cells: Vec<crate::models::ErrorCell>,
}

impl ErrorTable {
    /// Error table.
    pub fn new(name: String, cells: Vec<crate::models::ErrorCell>) -> ErrorTable {
        ErrorTable {
            name,
            cells,
        }
    }
}

