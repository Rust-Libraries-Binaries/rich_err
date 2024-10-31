# rich_err

**rich_err** is a Rust library designed to simplify and enrich error handling by adding structured and meaningful context to errors. It provides a developer-friendly API for consistent and detailed error reporting, making debugging and error analysis more efficient.

## Features
- **Rich Error Context**: Easily add and format detailed context to errors for improved debugging.
- **Zero-Cost Abstractions**: Designed with performance in mind, adding context without unnecessary overhead.
- **Easy to Use**: Simple and intuitive API for managing error contexts.

## Getting Started
To include `rich_err` in your project, add the following line to your `Cargo.toml`:

```toml
[dependencies]
rich_err = "0.1.0"
```
## Example Usage
Here's a basic example demonstrating how to use rich_err:

```rust

use rich_err::RichError;

fn main() {
    let error = RichError::new("An error occurred")
        .with_context("Failed to open file")
        .with_context("Permission denied");

    println!("{}", error.to_string_with_context());
}
```
## Output:
An error occurred Context: Failed to open file Context: Permission denied

## API Reference
RichError
new(message: &str) -> Self: Creates a new RichError with a base error message.
with_context(mut self, context: &str) -> Self: Adds additional context to the error.
to_string_with_context(&self) -> String: Formats the error message with all context included.

## Running Tests
To run the tests and ensure everything is working correctly, use the following command:

```bash

cargo test
```

## Contributing
Contributions are welcome! If you'd like to help improve rich_err, feel free to submit issues or pull requests on GitHub.

## How to Contribute:
Fork the repository.
Create a new feature branch: git checkout -b feature/your-feature-name.
Commit your changes: git commit -m 'Add your feature'.
Push to the branch: git push origin feature/your-feature-name.
Open a pull request.

## License
This project is licensed under the MIT License. See the LICENSE file for more details.

## Author
Ben Santora (<bensatlantik@gmail.com>)

