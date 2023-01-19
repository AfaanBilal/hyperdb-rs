HyperDB Rust Client
===================

Author: **[Afaan Bilal](https://afaan.dev)**

## Introduction
**HyperDB Rust** is a Rust crate for the [HyperDB server](https://github.com/AfaanBilal/hyperdb).

## Installation
````
cargo add hyperdb-rs
````

## Example usage
````rs
let mut hyper = hyperdb_rs::HyperClient::new(String::from("http://localhost:8765"));

// Ping the server
let r = hyper.ping().expect("failed");
println!("{}", r) // PONG

// Get the version number
let r = hyper.version().expect("failed");
println!("{}", r) // "[HyperDB v0.2.0 (https://afaan.dev)]"

// Set a value
let r = hyper.set("test", "value").expect("failed");
println!("{}", r) // "value"

// Check if a key is present
let r = hyper.has("test").expect("failed");
println!("{}", r) // YES

// Get a value
let r = hyper.get("test").expect("failed");
println!("{}", r) // "value"

// Get all stored data
let r = hyper.all().expect("failed");
println!("{}", r) // {test: "value"}

// Remove a key
let r = hyper.delete("test").expect("failed");
println!("{}", r) // OK

// Delete all stored data
r = hyper.clear().expect("failed");
println!("{}", r) // OK

// Check if the store is empty
r = hyper.empty().expect("failed");
println!("{}", r) // YES

// Persist the store to disk
r = hyper.save().expect("failed");
println!("{}", r) // OK

// Reload the store from disk
r = hyper.reload().expect("failed");
println!("{}", r) // OK

// Delete all store data from memory and disk
r = hyper.reset().expect("failed");
println!("{}", r) // OK

````

## Contributing
All contributions are welcome. Please create an issue first for any feature request
or bug. Then fork the repository, create a branch and make any changes to fix the bug
or add the feature and create a pull request. That's it!
Thanks!

## License
**HyperDB Rust** is released under the MIT License.
Check out the full license [here](LICENSE).
