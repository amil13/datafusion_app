/*
* This file contains all the command-line argument for the application
*
* Author: Amil Shrivastava
*/

use clap::{ArgEnum, Parser, Subcommand};
use std::path::PathBuf;

/// Represents command-line arguments for the application
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct MDataAppArgs {
    /// Input file path
    #[clap(short, long, parse(from_os_str), help = "Input path")]
    pub input: PathBuf,
    
    /// Output file path
    #[clap(short, long, parse(from_os_str), help = "Output path")]
    pub output: PathBuf,
    
    /// Verbose level (0: info, 1: debug, 2: trace)
    #[clap(short, long, parse(from_occurrences), help = "Verbose level")]
    pub verbose: usize,
    
    /// Output file format (CSV or Parquet)
    #[clap(short, long, arg_enum, default_value_t = WriteFormat::Undefined,
    help = "Output format")]
    pub format: WriteFormat,
    
    /// Limit the number of rows in the output
    #[clap(short, long, default_value_t = 0, help = "Limit the result to the first <limit> rows")]
    pub limit: usize,
    
    /// Flag to display the inferred schema
    #[clap(short, long, parse(from_flag), help = "Display the inferred schema")]
    pub schema: bool,
    
    /// Column name for sorting
    #[clap(short='c', long, help = "Column name to sort by")]
    pub sort_column: Option<String>, 
    
    /// Sort order for the specified column
    #[clap(short='r', long, arg_enum, help = "Sorts the column in ascending or descending order")]
    pub sort_order: Option<SortOrder>,
    
    /// Optional filter to apply
    #[clap(subcommand)]
    pub filter: Option<Filters>,
}

/// Defines available filter operations for the application
#[derive(Subcommand, Debug)]
pub enum Filters {
    /// Filter rows where the column is equal to a specified value
    Eq { column: String, value: String },
}

/// Specifies the output file format options
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, ArgEnum)]
pub enum WriteFormat {
    Undefined = 0,
    Csv = 1,
    Parquet = 2,
}

/// Enum to specify sort order (Ascending or Descending)
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, ArgEnum)]
pub enum SortOrder {
    Asc = 1,
    Desc = 2,
}
