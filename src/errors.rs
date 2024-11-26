/*
* This file contains all the error types for the application
*
* Author: Amil Shrivastava
*/

use crate::cli::WriteFormat;

use std::path::PathBuf;
use thiserror::Error;
use datafusion::error::DataFusionError;
use std::str::ParseBoolError;
use std::num::ParseFloatError;

/// Custom error type for the application
#[derive(Error, Debug)]
pub enum MDataAppError {
    /// Error for invalid path encoding
    #[error("invalid path encoding {path:?}")]
    PathEncoding { path: PathBuf },

    /// Error for invalid input format
    #[error("invalid input format {path:?}")]
    InputFormat { path: PathBuf },

    /// Error for invalid output format
    #[error("invalid output format {format:?}")]
    OutputFormat { format: WriteFormat },

    /// Error for invalid filter value
    #[error("invalid filter value {error_message:?}")]
    FilterValue { error_message: String },

    /// Error when sort column name is missing
    #[error("column name missing")]
    SortColumnMissing { error_message: String },

    /// Wraps errors from the DataFusion library
    #[error("transparent")]
    DataFusionOp(#[from] DataFusionError),
}

// Conversion from ParseBoolError to MDataAppError for filtering
impl From<ParseBoolError> for MDataAppError {
    fn from(e: ParseBoolError) -> Self {
        MDataAppError::FilterValue {
            error_message: e.to_string(),
        }
    }
}

// Conversion from ParseFloatError to MDataAppError for filtering
impl From<ParseFloatError> for MDataAppError {
    fn from(e: ParseFloatError) -> Self {
        MDataAppError::FilterValue {
            error_message: e.to_string(),
        }
    }
}
