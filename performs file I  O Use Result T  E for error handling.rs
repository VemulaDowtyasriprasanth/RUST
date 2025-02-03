use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// Reads the content of a file and returns it as a String
pub fn read_file_to_string(file_path: &str) -> Result<String, io::Error> {
    let mut file = File::open(file_path)?; // Open the file
    let mut content = String::new();
    file.read_to_string(&mut content)?; // Read the file's content
    Ok(content) // Return the content
}

// Writes the given content to a file
pub fn write_string_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?; // Create or overwrite the file
    file.write_all(content.as_bytes())?; // Write the content to the file
    Ok(()) // Return Ok if successful
}

// Appends content to an existing file
pub fn append_to_file(file_path: &str, content: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?; // Open the file in append mode
    file.write_all(content.as_bytes())?; // Append the content
    Ok(()) // Return Ok if successful
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_io() {
        let file_path = "test.txt";

        // Test writing to a file
        write_string_to_file(file_path, "Hello, Rust!").unwrap();

        // Test reading from the file
        let content = read_file_to_string(file_path).unwrap();
        assert_eq!(content, "Hello, Rust!");

        // Test appending to the file
        append_to_file(file_path, " Appending more data.").unwrap();

        // Validate the final content
        let updated_content = read_file_to_string(file_path).unwrap();
        assert_eq!(updated_content, "Hello, Rust! Appending more data.");
    }
}



/*

Explanation of Key Features
Error Propagation with Result<T, E>:

The ? operator is used to propagate errors. If an operation (e.g., opening or reading a file) fails,
 the function returns the error immediately, 
simplifying error handling.
Idiomatic File Handling:

File operations (File::open, File::create, etc.) are wrapped in Result types, ensuring that any issues
 (like missing files or permissions errors) are caught and propagated.
Flexible Functionality:

The module supports reading, writing, and appending content to files, which are common file I/O use cases.
Testing with #[cfg(test)]:

A test suite ensures the functions work as expected, covering scenarios like writing, reading, and appending to a file.
RAII for Resource Management:

Files are automatically closed when they go out of scope, thanks to Rust’s RAII (Resource Acquisition Is Initialization) principle.




Best Practices Demonstrated
Error Transparency:

The functions return Result<T, E>, making it explicit that errors may occur. This approach allows callers to handle errors appropriately.
Error Context:

While this example doesn’t include custom error types, you can extend it using the thiserror or anyhow crates for
 better error descriptions in larger projects.
Test Coverage:

The #[cfg(test)] module ensures that changes to the code don’t introduce bugs in critical file I/O operations


Example Usage
Example 1: Reading a File
rust
Copy code
fn main() -> Result<(), std::io::Error> {
    match read_file_to_string("example.txt") {
        Ok(content) => println!("File content:\n{}", content),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
    Ok(())
}
Example 2: Writing to a File
rust
Copy code
fn main() -> Result<(), std::io::Error> {
    write_string_to_file("output.txt", "This is Rust File I/O!")?;
    println!("Content written successfully.");
    Ok(())
}
Let me know if you’d like me to extend this module with custom error types or logging!










*/