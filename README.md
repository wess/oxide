# Oxide
> Just some mild macros that help me in my adventures in Rust.

---

### Adding it:
To add to your project you could add it to your Cargo.toml like:

```
[dependencies.oxide]
git = 'https://github.com/wess/oxide.git'
branch = 'master'
```

or like:

```
oxide = {git = 'https://github.com/wess/oxide.git', branch = 'master'}

```

---

## Now the breakdown:


### Console:
> Console macros print to stdout with a format like: `[<project_name>] <emoji> <your message here>`

- console_print : Prints with prefix and no emoji.
- console_log : Prints message with "ℹ" mark
- console_info : Prints message with "ℹ" mark 
- console_success : Prints message with "✔" mark
- console_warning : Prints message with "⚠" mark
- console_error : Prints message with "✖" mark
- console_debug : Prints message with "🐞" mark
- console_panic : Prints message with "✖" mark

Example usage:
```rust
console_success!("Yay: ", "woot");
```


## Other Macros (because why not?)
> btw, I know these exist because I'm lazy, and I'm ok with that. They might not be for everyone ;)

- either : Based on condition, give me this or that.
  - `let x = either!(hello == "world", "truthy", "false");`
- string : Because `String::new(...)` was just to much.
  - `let s:String = string!("hi");`
- string_combined : Im sure you can guess. 
  - `let res:String = string_combined!(string!("hello,"), string!("world"));`
- map : This was on a macro tutorial and I liked it.
  - `let m = map!("hello" => "world");`
- list : Don't use this, just use `vec!`
- arc_mu : Arc with a Mutex with a thingie.
  - `let am = arc_mu!(thingie);` 
- file_write : Not clear enough, or should I write it out for you. 
  - `file_write!("/path/to/blah.txt", string!("yaya"));
- file_read : Quick, get my contents or fail.
  - `let content = file_read!("/path/to/content");`
- file_exists : Is it there, or not? 
  - `if file_exists!("/path/to") {`
