/*
* This file contains all the unit tests for the application
*
* Author: Amil Shrivastava
*/

//unit_testing
#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    
    use crate::cli::{WriteFormat,SortOrder,Filters,MDataAppArgs};
    use crate::errors::MDataAppError;
    use crate::processing::mdata_app;
    use crate::utils::{get_os_path,infer_file_type,ensure_output_path_extension};
    use std::path::PathBuf;



    // Test file type inference for CSV, Parquet, and undefined extensions
    #[test]
    fn test_infer_file_type_csv() {
        let path = PathBuf::from("test.csv");
        assert_eq!(infer_file_type(&path, true), WriteFormat::Csv);
    }

    #[test]
    fn test_infer_file_type_parquet() {
        let path = PathBuf::from("test.parquet");
        assert_eq!(infer_file_type(&path, true), WriteFormat::Parquet);
    }

    #[test]
    fn test_infer_file_type_undefined() {
        let path = PathBuf::from("test.txt");
        assert_eq!(infer_file_type(&path, true), WriteFormat::Undefined);
    }

    // Test OS path handling
    #[test]
    fn test_get_os_path_existing() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("temp_file.csv");
        File::create(&file_path).unwrap();
        assert!(get_os_path(&file_path).is_ok());
    }

    // Test automatic output file extension addition
    #[test]
    fn test_ensure_output_path_extension_csv() {
        let path = PathBuf::from("output");
        let extended_path = ensure_output_path_extension(&path, &WriteFormat::Csv);
        assert_eq!(extended_path, "output.csv");
    }

    #[test]
    fn test_ensure_output_path_extension_parquet() {
        let path = PathBuf::from("output");
        let extended_path = ensure_output_path_extension(&path, &WriteFormat::Parquet);
        assert_eq!(extended_path, "output.parquet");
    }

    #[test]
    fn test_ensure_output_path_extension_undefined() {
        let path = PathBuf::from("output");
        let extended_path = ensure_output_path_extension(&path, &WriteFormat::Undefined);
        assert_eq!(extended_path, "output.txt");
    }

    // Acceptance test for the main program, testing filter, sorting, schema display, and format
    #[tokio::test]
    async fn test_mdata_app_full_run() {
        // Set up a temporary directory for test files
        let temp_dir = tempdir().unwrap();
        let input_file = temp_dir.path().join("input.csv");
        let output_file = temp_dir.path().join("output.csv");

        // Create a test CSV input file
        let mut file = File::create(&input_file).unwrap();
        writeln!(file, "column1,column2\nvalue1,value2\nvalue3,value4").unwrap();

        // Set up CLI arguments for the test run
        let args = MDataAppArgs {
            input: input_file.clone(),
            output: output_file.clone(),
            verbose: 1,
            format: WriteFormat::Csv,
            limit: 1,
            schema: true,
            sort_column: Some("column1".to_string()),
            sort_order: Some(SortOrder::Asc),
            filter: Some(Filters::Eq {
                column: "column1".to_string(),
                value: "value1".to_string(),
            }),
        };

        // Run the application with the test arguments
        let result = mdata_app(args).await;
        assert!(
            result.is_ok(),
            "Expected Ok result, but got Err: {:?}",
            result
        );

        // Verify the output file was created
        assert!(output_file.exists());

        // Check that only the first line was outputted (limited to 1 row)
        let output_content = fs::read_to_string(&output_file).unwrap();
        assert!(output_content.contains("value1,value2"));
        assert!(!output_content.contains("value3,value4"));
    }

    // Test error handling for missing sort column when sorting is requested
    #[tokio::test]
    async fn test_mdata_app_sort_column_missing() {
        let temp_dir = tempdir().unwrap();
        let input_file = temp_dir.path().join("input.csv");
        let output_file = temp_dir.path().join("output.csv");

        let mut file = File::create(&input_file).unwrap();
        writeln!(file, "column1,column2\nvalue1,value2\n").unwrap();

        let args = MDataAppArgs {
            input: input_file.clone(),
            output: output_file.clone(),
            verbose: 1,
            format: WriteFormat::Csv,
            limit: 1,
            schema: false,
            sort_column: Some("".to_string()), // Empty column name
            sort_order: Some(SortOrder::Asc),
            filter: None,
        };

        let result = mdata_app(args).await;
        assert!(matches!(
            result,
            Err(MDataAppError::SortColumnMissing { .. })
        ));
    }

    // Test if schema display is functioning as expected
    #[tokio::test]
    async fn test_mdata_app_schema_display() {
        let temp_dir = tempdir().unwrap();
        let input_file = temp_dir.path().join("input.csv");
        let output_file = temp_dir.path().join("output.csv");

        let mut file = File::create(&input_file).unwrap();
        writeln!(file, "column1,column2\nvalue1,value2\n").unwrap();

        let args = MDataAppArgs {
            input: input_file.clone(),
            output: output_file.clone(),
            verbose: 1,
            format: WriteFormat::Csv,
            limit: 1,
            schema: true, // Request to display schema
            sort_column: None,
            sort_order: None,
            filter: None,
        };

        // Capture schema output with a valid run
        let result = mdata_app(args).await;
        assert!(result.is_ok());
    }
}
