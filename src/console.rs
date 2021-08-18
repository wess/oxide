//
// console.rs
// oxide
// 
// Author: Wess Cope (me@wess.io)
// Created: 08/17/2021
// 
// Copywrite (c) 2021 Wess.io
//

use colored::*;

pub const CHECK_MARK:&str="✔";
pub const CROSS_MARK:&str="✖";
pub const ARROW_MARK:&str="⮕";
pub const WARNING_MARK:&str="⚠";
pub const INFO_MARK:&str="ℹ";
pub const BUG_MARK:&str="🐞";

pub fn _success_mark() -> String {
  format!("{} {}", _wordmark(), CHECK_MARK.bold().green())
}

pub fn _error_mark() -> String {
  format!("{} {}", _wordmark(), CROSS_MARK.bold().red())
}

pub fn _status_mark() -> String {
  format!("{} {}", _wordmark(), ARROW_MARK.bold().cyan())
}

pub fn _warning_mark() -> String {
  format!("{} {}", _wordmark(), WARNING_MARK.bold().yellow())
}

pub fn _info_mark() -> String {
  format!("{} {}", _wordmark(), INFO_MARK.bold().blue())
}

pub fn _bug_mark() -> String {
  format!("{} {}", _wordmark(), BUG_MARK.bold().magenta())
}

pub fn _wordmark() -> String {
  let name = env!("CARGO_PKG_NAME");

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

#[macro_export]
macro_rules! console_panic {
  ($msg:expr) => (
    panic!(format!("{}  {}", $crate::console::_error_mark(), $msg));
  );

  ($fmt:expr, $($xs:expr)*) => (console_panic!(format!($fmt, $($xs)* )));
}

