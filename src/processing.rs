/*
* This file contains all the dataprocessing using datafusion for the application
*
* Author: Amil Shrivastava
*/

use crate::cli::{MDataAppArgs, Filters, WriteFormat, SortOrder};
use crate::errors::MDataAppError;
use crate::utils::{get_os_path,infer_file_type,ensure_output_path_extension};
use datafusion::prelude::*;
use datafusion::arrow::datatypes::DataType;
use datafusion::dataframe::DataFrameWriteOptions;

use log::info;

/// The primary application function that orchestrates data processing
/// This function handls data loading, transformations, and saving results.
/// # Arguments
/// - `opts`: Parsed command-line arguments
pub async fn mdata_app(opts: MDataAppArgs) -> Result<(), MDataAppError>{
    
        let ctx = SessionContext::new();
        let input_path = get_os_path(&opts.input)?;  
        
        // the inferred format is returned
        let inferred_input_format = match infer_file_type(&opts.input, true) {
            WriteFormat::Csv => {
                ctx.register_csv("input", input_path, CsvReadOptions::new())
                .await?;
            WriteFormat::Csv
        }
        WriteFormat::Parquet => {
            ctx.register_parquet("input", input_path, ParquetReadOptions::default())
            .await?;
        WriteFormat::Parquet
    }
    WriteFormat::Undefined => {
        return Err(MDataAppError::InputFormat { path: opts.input });
    }
    };
    
    // data-transformation
    let df_input = ctx.table("input").await?;
    
    // Schema: get the table schema
    let schema = df_input.schema();
    
    // print-it
    if opts.schema {
        info!("# Schema\n{:#?}", schema)
    }
    
    // Filter: process user filters
    let df_flt = if let Some(Filters::Eq {
        column: column_name,
            value,
        }) = &opts.filter
        {
            // get the data type of the filtered column
            let filter_type = schema.field_with_name(None, column_name)?.data_type();
            // parse the filter value based on column type
            let filter_value = match filter_type {
                DataType::Boolean => lit(value.to_string().parse::<bool>()?),
                DataType::Utf8 => lit(value.to_string()),
                x if DataType::is_numeric(x) => lit(value.to_string().parse::<f64>()?),
                _ => {
                    return Err(MDataAppError::FilterValue {
                        error_message: "invalid filter value".to_string(),
                    })
                }
            };
            // filter the current dataframe
            df_input.filter(col(column_name).eq(filter_value))?
        } else {
            df_input
        };
        
        //Limit: if limit is active, only N first rows are written
        let df_lmt = if opts.limit > 0 {
            df_flt.limit(0, Some(opts.limit))?
        } else {
            df_flt
        };
    
    // Sort: If Ascending-Descending is active, apply sorting on df_lmt
    let df_final = if let Some(sort_column) = &opts.sort_column {
        // Check if the column name is empty
        if sort_column.is_empty() {
            return Err(MDataAppError::SortColumnMissing {
                error_message: "The sort column is missing or empty.".to_string(),
            });
        }
        // Sort the dataframe by the specified column and order
        let sort_order = opts.sort_order == Some(SortOrder::Asc); // true for ascending, false for descending
        df_lmt.sort(vec![col(sort_column).sort(sort_order, true)])?
    } else {
        df_lmt // No sorting is applied if sort_column is None
    };
    
        
        // if the output format is undefined, default to input format
        let output_format = match opts.format {
            WriteFormat::Undefined => {
                if inferred_input_format == WriteFormat::Undefined {
                    WriteFormat::Undefined
                } else {
                    inferred_input_format
                }
            }
            _ => opts.format.clone(),
        };
        
        let output_path = ensure_output_path_extension(&opts.output, &output_format);

        // Finally write the output
        match output_format {
            
            WriteFormat::Csv => df_final.write_csv(&output_path,DataFrameWriteOptions::new(),None).await?,
            WriteFormat::Parquet => df_final.write_parquet(&output_path,DataFrameWriteOptions::new(),None).await?,
            WriteFormat::Undefined => {
                return Err(MDataAppError::OutputFormat {
                    format: opts.format.clone(),
                });
            }
        };
    
        Ok(())
    
}
