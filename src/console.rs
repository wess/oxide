//
// console.rs
// oxide
// 
// Author: Wess Cope (me@wess.io)
// Created: 08/17/2021
// 
// Copywrite (c) 2021 Wess.io
//

//! Console output utilities with colored formatting.
//!
//! This module provides functions and macros for formatted console output with
//! consistent styling and colored indicators. All console output follows the
//! pattern: `[<program_name>] <emoji> <message>`

use colored::*;

use std::env;
use std::path::Path;
use std::ffi::OsStr;

/// Extracts the program name from command line arguments.
/// 
/// Returns the filename portion of the first command line argument (usually the program name).
/// If extraction fails, returns None.
fn prog() -> Option<String> {
  env::args().next()
      .as_ref()
      .map(Path::new)
      .and_then(Path::file_name)
      .and_then(OsStr::to_str)
      .map(String::from)
}


/// Unicode character for success/check mark
pub const CHECK_MARK:&str="✔";
/// Unicode character for error/cross mark
pub const CROSS_MARK:&str="✖";
/// Unicode character for status/arrow mark
pub const ARROW_MARK:&str="⮕";
/// Unicode character for warning mark
pub const WARNING_MARK:&str="⚠";
/// Unicode character for info mark
pub const INFO_MARK:&str="ℹ";
/// Unicode character for debug/bug mark
pub const BUG_MARK:&str="🐞";

/// Creates a formatted success mark with program name prefix.
/// 
/// Returns a string containing the program wordmark followed by a green checkmark.
pub fn _success_mark() -> String {
  format!("{} {}", _wordmark(), CHECK_MARK.bold().green())
}

/// Creates a formatted error mark with program name prefix.
/// 
/// Returns a string containing the program wordmark followed by a red cross mark.
pub fn _error_mark() -> String {
  format!("{} {}", _wordmark(), CROSS_MARK.bold().red())
}

/// Creates a formatted status mark with program name prefix.
/// 
/// Returns a string containing the program wordmark followed by a cyan arrow mark.
pub fn _status_mark() -> String {
  format!("{} {}", _wordmark(), ARROW_MARK.bold().cyan())
}

/// Creates a formatted warning mark with program name prefix.
/// 
/// Returns a string containing the program wordmark followed by a yellow warning mark.
pub fn _warning_mark() -> String {
  format!("{} {}", _wordmark(), WARNING_MARK.bold().yellow())
}

/// Creates a formatted info mark with program name prefix.
/// 
/// Returns a string containing the program wordmark followed by a blue info mark.
pub fn _info_mark() -> String {
  format!("{} {}", _wordmark(), INFO_MARK.bold().blue())
}

/// Creates a formatted debug mark with program name prefix.
/// 
/// Returns a string containing the program wordmark followed by a magenta bug mark.
pub fn _bug_mark() -> String {
  format!("{} {}", _wordmark(), BUG_MARK.bold().magenta())
}

/// Creates a formatted program name wordmark.
/// 
/// Returns the program name enclosed in brackets with dimmed styling.
/// If program name cannot be determined, uses "error" as fallback.
pub fn _wordmark() -> String {
  let name = prog().unwrap_or("error".to_string());

  format!("[{}]", name).bold().dimmed().to_string()
}

#[macro_export]
macro_rules! console_print {
    ($msg:expr) => (
      println!("{}", $msg);
    );

    ($fmt:expr, $($xs:expr)*) => (console_print!(format!($fmt, $($xs)* )));
}


#[macro_export]
macro_rules! console_log {
  ($msg:expr) => ($crate::console_print!(format!("{}  {}", $crate::console::_status_mark(), $msg)));

  ($fmt:expr, $($xs:expr)*) => (console_log!(format!($fmt, $($xs)* )));
}

#[macro_export]
macro_rules! console_info {
  ($msg:expr) => ($crate::console_print!(format!("{}  {}", $crate::console::_info_mark(), $msg)));

  ($fmt:expr, $($xs:expr)*) => (console_info!(format!($fmt, $($xs)* )));
}

#[macro_export]
macro_rules! console_success {
  ($msg:expr) => ($crate::console_print!(format!("{}  {}", $crate::console::_success_mark(), $msg)));

  ($fmt:expr, $($xs:expr)*) => (console_success!(format!($fmt, $($xs)* )));
}

#[macro_export]
macro_rules! console_warning {
  ($msg:expr) => ($crate::console_print!(format!("{}  {}", $crate::console::_warning_mark(), $msg)));

  ($fmt:expr, $($xs:expr)*) => (console_warning!(format!($fmt, $($xs)* )));
}

#[macro_export]
macro_rules! console_error {
  ($msg:expr) => ($crate::console_print!(format!("{}  {}", $crate::console::_error_mark(), $msg)));

  ($fmt:expr, $($xs:expr)*) => (console_error!(format!($fmt, $($xs)* )));
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! console_debug {
  ($msg:expr) => ($crate::console_print!(format!("{}  {}", $crate::console::_bug_mark(), $msg)));

  ($fmt:expr, $($xs:expr)*) => (console_debug!(format!($fmt, $($xs)* )));
}


#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! console_debug {
  ($msg:expr) => ();

  ($fmt:expr, $($xs:expr)*) => (console_debug!(format!($fmt, $($xs)* )));
}

/// Panics with a formatted error message.
/// 
/// Similar to the standard `panic!` macro, but prefixes the panic message
/// with the program name and a red error mark for consistency with other
/// console output.
/// 
/// # Examples
/// 
/// ```rust,should_panic
/// # use oxide::console_panic;
/// console_panic!("Critical error occurred");
/// ```
/// 
/// ```rust,should_panic
/// # use oxide::console_panic;
/// let invalid_value = -1;
/// console_panic!("Invalid value: {}", invalid_value);
/// ```
#[macro_export]
macro_rules! console_panic {
  ($msg:expr) => (
    panic!("{}  {}", $crate::console::_error_mark(), $msg);
  );

  ($fmt:expr, $($xs:expr)*) => (console_panic!(format!($fmt, $($xs)* )));
}

