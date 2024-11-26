/* 
* This file contains all the utility functions for the application
*
* Author: Amil Shrivastava
*/

use std::path::PathBuf;
use std::ffi::OsStr;
use crate::cli::WriteFormat;
use crate::errors::MDataAppError;

/// Infers file type from a file path
/// 
/// # Arguments
/// - `path`: The path to check for a file type.
/// - `check_extension`: If true, checks the file extension to infer type.
///
/// # Returns
/// - The inferred `WriteFormat`, or `Undefined` if inference fails.
pub fn infer_file_type(path: &PathBuf, check_extension: bool) -> WriteFormat {
    if check_extension {
        match path.extension().and_then(OsStr::to_str) {
            Some("csv") => WriteFormat::Csv,
            Some("parquet") => WriteFormat::Parquet,
            _ => WriteFormat::Undefined,
        }
    } else {
        WriteFormat::Undefined
    }
}

/// Converts a path to a string, returning an error for invalid encoding
///
/// # Arguments
/// - `path`: The path to be converted.
///
/// # Returns
/// - `&str` representation of the path, or an `MDataAppError` for invalid encoding.
pub fn get_os_path(path: &PathBuf) -> Result<&str, MDataAppError> {
    if path.exists() {
        // Convert the Path to &str using `to_str()`
        path.to_str()
            .ok_or_else(|| MDataAppError::PathEncoding { path: path.clone() })
    } else {
        // Convert the newly created Path to &str
        path.to_str()
            .ok_or_else(|| MDataAppError::PathEncoding { path: path.clone() })
    }
}


/// Ensures the output path has the correct extension based on format
/// # Arguments
/// - `path`: Original path
/// - `format`: Desired file format
///
/// # Returns
/// - A `String` containing the modified path with the correct extension.
pub fn ensure_output_path_extension(path: &PathBuf, format: &WriteFormat) -> String {
    let mut out_path = path.clone();
    if out_path.extension().is_none() {
        let extension = match format {
            WriteFormat::Csv => "csv",
            WriteFormat::Parquet => "parquet",
            WriteFormat::Undefined => "txt", // Default extension for undefined format
        };
        out_path.set_extension(extension);
    }
    out_path.to_str().unwrap_or_else(|| {
        // Handle case where conversion fails
        panic!("Failed to convert output path to string.")
    }).to_string()
}
