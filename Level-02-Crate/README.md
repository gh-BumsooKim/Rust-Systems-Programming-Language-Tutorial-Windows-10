# Level-02-Crate

In Rust, code package is called 'Crate'

If you want to use external package, you should add crate name on dependencies in Cargo.toml with crate version like following code.

```
[dependencies]

rand = "0.3.14"
```

- `extern crate xxx` : add used library crate depend on external code

- `use xxx::xxx` : bring trait scope 
