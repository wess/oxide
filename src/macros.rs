//
// macros.rs
// oxide
// 
// Author: Wess Cope (me@wess.io)
// Created: 08/17/2021
// 
// Copywrite (c) 2021 Wess.io
//

//! General-purpose utility macros.
//!
//! This module provides convenient macros for common Rust patterns including
//! conditional expressions, string manipulation, collection creation, concurrency
//! helpers, and file operations.



/// Conditional expression macro - returns one value or another based on a condition.
/// 
/// This macro provides a concise way to choose between two values based on a boolean
/// condition. It's similar to the ternary operator in other languages.
/// 
/// # Arguments
/// 
/// * `test` - A boolean expression to evaluate
/// * `truthy` - Value returned if the test is true
/// * `falsy` - Value returned if the test is false
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::either;
/// let x = 5;
/// let result = either!(x > 3 => "greater", "less or equal");
/// assert_eq!(result, "greater");
/// 
/// let name = either!(true => "Alice", "Bob");
/// assert_eq!(name, "Alice");
/// ```
#[macro_export]
macro_rules! either {
  ($test:expr => $truthy:expr , $falsy:expr) => {
    if $test {
      $truthy
    } else {
      $falsy
    }
  }
}

/// Creates a `String` from a string literal or formatted string.
/// 
/// This macro provides a shorter way to create `String` instances compared to
/// `String::from()` or `format!()`. It supports both simple string creation
/// and formatted string creation with arguments.
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::string;
/// let s = string!("hello");
/// assert_eq!(s, "hello");
/// 
/// let formatted = string!("Hello, {}!", "world");
/// assert_eq!(formatted, "Hello, world!");
/// 
/// let number = string!("Count: {}", 42);
/// assert_eq!(number, "Count: 42");
/// ```
#[macro_export]
macro_rules! string {
  ($str:expr) => 
    {{ String::from($str) }};

  ($str:expr, $($xs:expr)*) => 
    {{String::from(format!($str, $($xs)* ))}};
  
}

/// Combines two string-like values into a single `String`.
/// 
/// This macro efficiently concatenates two string values by converting the first
/// to a `String` and appending the second to it. Both arguments can be `&str`,
/// `String`, or anything that can be converted to a string.
/// 
/// # Arguments
/// 
/// * `head` - The first string value
/// * `tail` - The second string value to append
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::string_combined;
/// let result = string_combined!("hello", " world");
/// assert_eq!(result, "hello world");
/// 
/// let greeting = "Hi";
/// let name = " there";
/// let combined = string_combined!(greeting, name);
/// assert_eq!(combined, "Hi there");
/// ```
#[macro_export]
macro_rules! string_combined {
  ($head:expr,$tail:expr) => {{
    let mut destination = String::from($head);
    destination.push_str(&$tail);
    destination
  }}
 }

/// Creates a `HashMap` with the specified key-value pairs.
/// 
/// This macro provides a convenient way to create and populate a `HashMap`
/// in a single expression, similar to dictionary literals in other languages.
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::map;
/// use std::collections::HashMap;
/// 
/// let m = map!("key1" => "value1", "key2" => "value2");
/// assert_eq!(m.get("key1"), Some(&"value1"));
/// assert_eq!(m.get("key2"), Some(&"value2"));
/// 
/// let numbers = map!(1 => "one", 2 => "two", 3 => "three");
/// assert_eq!(numbers.len(), 3);
/// 
/// // Empty map
/// let empty: HashMap<&str, i32> = map!();
/// assert!(empty.is_empty());
/// ```
#[macro_export]
macro_rules! map {
  ($($key:expr => $value:expr),*) => {{
    {
      use std::collections::HashMap;
      #[allow(unused_mut)]
      let mut hm = HashMap::new();
      $(hm.insert($key, $value);)*
      hm
    }
  }};
}

/// Creates a `Vec` with the specified values.
/// 
/// This macro provides an alternative to the standard `vec!` macro for creating
/// vectors. While functionally equivalent to `vec!`, it follows the naming
/// convention of other macros in this crate.
/// 
/// **Note**: Consider using the standard `vec!` macro instead, as mentioned in the
/// library documentation.
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::list;
/// let numbers = list!(1, 2, 3, 4, 5);
/// assert_eq!(numbers, vec![1, 2, 3, 4, 5]);
/// 
/// let single = list!(42);
/// assert_eq!(single, vec![42]);
/// 
/// // Empty list
/// let empty: Vec<i32> = list!();
/// assert!(empty.is_empty());
/// ```
#[macro_export]
macro_rules! list {
  ($($value:expr),*) => {{
    {
      #[allow(unused_mut)]
      let mut list = Vec::new();
      $(list.push($value);)*
      list
    }
  }}
}

/// Creates an `Arc<Mutex<T>>` wrapper around a value.
/// 
/// This macro provides a shorthand for creating thread-safe, reference-counted
/// values that can be safely shared between threads. It combines `Arc` (Atomically
/// Reference Counted) with `Mutex` (Mutual Exclusion) for thread-safe access.
/// 
/// # Arguments
/// 
/// * `var` - The value to wrap in `Arc<Mutex<T>>`
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::arc_mu;
/// use std::sync::{Arc, Mutex};
/// use std::thread;
/// 
/// let shared_data = arc_mu!(42);
/// 
/// // Clone for use in another thread
/// let data_clone = shared_data.clone();
/// let handle = thread::spawn(move || {
///     let mut data = data_clone.lock().unwrap();
///     *data += 1;
/// });
/// 
/// handle.join().unwrap();
/// assert_eq!(*shared_data.lock().unwrap(), 43);
/// ```
#[macro_export]
macro_rules! arc_mu {
  ($var:expr) => {{
    use std::sync::{Arc, Mutex};
    Arc::new(Mutex::new($var))
  }}
}

/// Writes string data to a file, creating it if necessary.
/// 
/// This macro provides a simple way to write string content to a file. It will
/// create the file if it doesn't exist, or overwrite it if it does. The macro
/// panics with a formatted error message if the file cannot be created.
/// 
/// # Arguments
/// 
/// * `path` - Path to the file to write to
/// * `data` - String data to write to the file
/// 
/// # Returns
/// 
/// Returns `Result<(), std::io::Error>` from the write operation.
/// 
/// # Panics
/// 
/// Panics if the file cannot be created, with a descriptive error message.
/// 
/// # Examples
/// 
/// ```rust,no_run
/// # use oxide::{file_write, string};
/// # // Note: This example doesn't run in doc tests due to file system access
/// # // let content = string!("Hello, file!");
/// # // let result = file_write!("/tmp/test.txt", content);
/// # // result.expect("Failed to write");
/// # 
/// # // file_write!("/tmp/numbers.txt", "1\n2\n3\n").unwrap();
/// ```
#[macro_export]
macro_rules! file_write {
  ($path:expr, $data:expr) => {{
    use std::fs::File;
    use std::io::Write;
    let mut file = match File::create($path) {
      Ok(file) => file,
      Err(e) => {
        $crate::console_panic!("Unable to create file: {}", e);
      }
    };

    file.write_all($data.as_bytes())
  }};
}

/// Reads the entire contents of a file as a `String`.
/// 
/// This macro provides a simple way to read a file's contents into a string.
/// It panics with a formatted error message if the file cannot be opened or read.
/// 
/// # Arguments
/// 
/// * `path` - Path to the file to read from
/// 
/// # Returns
/// 
/// Returns the file contents as a `String`.
/// 
/// # Panics
/// 
/// Panics if:
/// - The file cannot be opened (e.g., doesn't exist, permission denied)
/// - The file cannot be read (e.g., I/O error)
/// - The file contains invalid UTF-8 data
/// 
/// # Examples
/// 
/// ```rust,no_run
/// # use oxide::file_read;
/// # // Note: This example doesn't run in doc tests due to file system access
/// # // let content = file_read!("/etc/hostname");
/// # // println!("Hostname: {}", content.trim());
/// # 
/// # // let config = file_read!("config.txt");
/// # // Process config content...
/// ```
#[macro_export]
macro_rules! file_read {
  ($path:expr) => {{
    use std::io::Read;
    
    let mut file = match std::fs::File::open($path) {
      Ok(file) => file,
      Err(e) => {
        $crate::console_panic!("Unable to open file: {}", e);
      }
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
      Ok(_) => {},
      Err(e) => {
        $crate::console_panic!("Unable to read file: {}", e);
      }
    };

    buffer
  }};
}

/// Checks if a file or directory exists at the given path.
/// 
/// This macro provides a simple way to test for the existence of a file
/// or directory without having to import `std::path::Path`.
/// 
/// # Arguments
/// 
/// * `path` - Path to check for existence
/// 
/// # Returns
/// 
/// Returns `true` if the path exists, `false` otherwise.
/// 
/// # Examples
/// 
/// ```rust
/// # use oxide::file_exists;
/// if file_exists!("/etc/passwd") {
///     println!("Password file exists");
/// }
/// 
/// let config_path = "./config.toml";
/// if !file_exists!(config_path) {
///     println!("Config file not found");
/// }
/// ```
#[macro_export]
macro_rules! file_exists {
  ($path:expr) => {{
    use std::path::Path;
    let path = Path::new($path);
    path.exists()
  }};
}
