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
macro_rules! either {
  ($test:expr => $truthy:expr , $falsy:expr) => {
    if $test {
      $truthy
    } else {
      $falsy
    }
  }
}

#[macro_export]
macro_rules! string {
  ($str:expr) => 
    {{ String::from($str) }};

  ($str:expr, $(xs:expr)*) => 
    {{String::from(format!($str, $(xs)* ))}};
  
}

#[macro_export]
macro_rules! string_combined {
  ($head:expr,$tail:expr) => {{
    let mut destination = String::from($head);
    let appending = String::from($tail);

    destination.push_str(&appending);

    String::from(destination)
  }}
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
macro_rules! arc_mu {
  ($var:expr) => (Arc::new(Mutex::new($var));)
}

#[macro_export]
macro_rules! file_write {
  ($path:expr, $data:expr) => {{
    let mut file = match File::create($path) {
      Ok(file) => file,
      Err(e) => {
        console_panic!("Unable to create file: {}", e);
      }
    };

    file.write_all($data.as_bytes())
  }};
}

#[macro_export]
macro_rules! file_read {
  ($path:expr) => {{
    use std::io::Read;
    
    let mut file = match std::fs::File::open($path) {
      Ok(file) => file,
      Err(e) => {
        console_panic!("Unable to open file: {}", e);
      }
    };

    let mut buffer = String::new();

    match file.read_to_string(&mut buffer) {
      Ok(_) => {},
      Err(e) => {
        console_panic!("Unable to read file: {}", e);
      }
    };

    buffer.clone()
  }};
}

#[macro_export]
macro_rules! file_exists {
  ($path:expr) => {{
    let path = Path::new($path);

    path.exists()
  }};
}
