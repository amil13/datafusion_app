
# DataFusion App - Rust-Based CLI Tool

**Author:** Amil Shrivastava

## Introduction

The **DataFusion App** is a **Rust-based command-line interface (CLI) tool** that utilizes the **Apache DataFusion** framework for data processing. It provides a straightforward interface for performing various data operations and showcases the following features:

- **Feature 1:** Filter - Filter the DataFrame by specifying a column and a filter value.
- **Feature 2:** Limit - Limit the number of rows returned in the DataFrame.
- **Feature 3:** Schema - Print the schema of the DataFrame.
- **Feature 4:** Format - Choose the output file format between CSV or Parquet.
- **Feature 5:** Sort - Sort the specified column in ascending or descending order.
- **Feature 6:** Help - Display information about using the application commands.

## Prerequisites

To run this application, ensure you have the latest version of **Rust** installed.

## How to Use the App:

1. Clone the repo to your local machine.
2. Navigate to the `datafusion_app` directory and build the app using the following command:

   ```bash
   cargo build .
   ```

3. Once built, you can run the application with the following command structure:

   ```bash
   cargo run -- [OPTIONS] --input <INPUT> --output <OUTPUT> [SUBCOMMAND]
   ```

   **Note:** The folder contains a sample file named `sample_data.csv` to use with the commands.

## Example Usage:

- To display help information, run:

   ```bash
   cargo run -- --help
   ```

- To limit the DataFrame by filtering the `col_bool` column for TRUE values, use the following command:

   ```bash
    cargo run -- -i sample_data.csv -o outputfile -l 6 eq col_bool true
   ```

   This command will output a file in **CSV format** at the same directory level with 6 rows and all the values of the `col_bool` column being `True`. You can also change the output format by adding the `--format parquet` flag.

Feel free to explore all the features in a similar manner using the flags from the help description!

## Testing:

- To run tests, ensure you have **Rust** installed on your system.
- You just have to use the following command:

   ```bash
   cargo test
   ```

   This will build and run all tests defined in the `test.rs` file.
