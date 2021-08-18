//
// macros.rs
// oxide
// 
// Author: Wess Cope (me@wess.io)
// Created: 08/17/2021
// 
// Copywrite (c) 2021 Wess.io
//

#[macro_export]
macro_rules! string {
  ($str:expr) => (String::from($str);)
}

#[macro_export]
macro_rules! map {
  ($($key:expr => $value:expr),*) => {{
    let mut hm = HashMap::new();

    $(hm.insert($key, $value);)*

    hm
  }};
}

#[macro_export]
macro_rules! list {
  ($($value:expr),*) => {{
    let mut list = Vec::new();

    $(list.push($value);)*
  }}
}

#[macro_export]
macro_rules! armu {
  ($var:expr) => (Arc::new(Mutex::new($var));)
}