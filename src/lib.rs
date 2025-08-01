//
// lib.rs
// oxide
// 
// Author: Wess Cope (me@wess.io)
// Created: 08/17/2021
// 
// Copywrite (c) 2021 Wess.io
//

//! # Oxide
//!
//! A collection of useful Rust macros for console output, string manipulation, 
//! collections, and file operations. This library provides convenient shortcuts
//! for common Rust patterns while maintaining safety and performance.
//!
//! ## Features
//!
//! - **Console macros**: Colored console output with consistent formatting
//! - **String utilities**: Simplified string creation and manipulation
//! - **Collection builders**: Easy creation of HashMap and Vec instances
//! - **File operations**: Simplified file read/write operations
//! - **Concurrency helpers**: `Arc<Mutex<T>>` creation shorthand
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.oxide]
//! git = 'https://github.com/wess/oxide.git'
//! branch = 'master'
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use oxide::*;
//!
//! // Console output
//! console_success!("Operation completed successfully");
//! let error_msg = "file not found";
//! console_error!("Something went wrong: {}", error_msg);
//!
//! // String creation
//! let s = string!("Hello, world!");
//! let combined = string_combined!("Hello", " world");
//!
//! // Collections
//! let map = map!("key1" => "value1", "key2" => "value2");
//! let list = list!(1, 2, 3, 4, 5);
//! ```

pub mod console;
pub mod macros;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::NamedTempFile;

    #[test]
    fn test_either_macro_true() {
        let result = either!(true => "truthy", "falsy");
        assert_eq!(result, "truthy");
    }

    #[test]
    fn test_either_macro_false() {
        let result = either!(false => "truthy", "falsy");
        assert_eq!(result, "falsy");
    }

    #[test]
    fn test_either_macro_expression() {
        let x = 5;
        let result = either!(x > 3 => "greater", "less or equal");
        assert_eq!(result, "greater");
    }

    #[test]
    fn test_string_macro_simple() {
        let s = string!("hello");
        assert_eq!(s, "hello");
        assert_eq!(std::any::type_name_of_val(&s), "alloc::string::String");
    }

    #[test]
    fn test_string_macro_formatted() {
        let s = string!("hello {}", "world");
        assert_eq!(s, "hello world");
    }

    #[test]
    fn test_string_combined_macro() {
        let result = string_combined!("hello", " world");
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_string_combined_with_string_macro() {
        let result = string_combined!(string!("hello"), string!(" world"));
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_map_macro_empty() {
        let m: HashMap<&str, i32> = map!();
        assert!(m.is_empty());
    }

    #[test]
    fn test_map_macro_single() {
        let m = map!("key" => "value");
        assert_eq!(m.get("key"), Some(&"value"));
    }

    #[test]
    fn test_map_macro_multiple() {
        let m = map!(
            "key1" => "value1",
            "key2" => "value2",
            "key3" => "value3"
        );
        assert_eq!(m.get("key1"), Some(&"value1"));
        assert_eq!(m.get("key2"), Some(&"value2"));
        assert_eq!(m.get("key3"), Some(&"value3"));
        assert_eq!(m.len(), 3);
    }

    #[test]
    fn test_list_macro_empty() {
        let l: Vec<i32> = list!();
        assert!(l.is_empty());
    }

    #[test]
    fn test_list_macro_single() {
        let l = list!(42);
        assert_eq!(l, vec![42]);
    }

    #[test]
    fn test_list_macro_multiple() {
        let l = list!(1, 2, 3, 4, 5);
        assert_eq!(l, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_arc_mu_macro() {
        let data = arc_mu!(42);
        assert_eq!(*data.lock().unwrap(), 42);
    }

    #[test]
    fn test_file_write_and_read() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();
        
        file_write!(file_path, string!("test content")).unwrap();
        
        let content = file_read!(file_path);
        assert_eq!(content, "test content");
    }

    #[test]
    fn test_file_exists_true() {
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap().to_string();
        
        std::fs::write(&file_path, "test").unwrap();
        
        assert!(file_exists!(&file_path));
    }

    #[test]
    fn test_file_exists_false() {
        assert!(!file_exists!("/nonexistent/path/file.txt"));
    }

    #[test]
    fn test_console_marks() {
        let success = console::_success_mark();
        let error = console::_error_mark();
        let warning = console::_warning_mark();
        let info = console::_info_mark();
        let bug = console::_bug_mark();
        let status = console::_status_mark();

        assert!(success.contains("✔"));
        assert!(error.contains("✖"));
        assert!(warning.contains("⚠"));
        assert!(info.contains("ℹ"));
        assert!(bug.contains("🐞"));
        assert!(status.contains("⮕"));
    }

    #[test]
    fn test_wordmark() {
        let wordmark = console::_wordmark();
        assert!(wordmark.starts_with('['));
        assert!(wordmark.ends_with(']'));
    }
}