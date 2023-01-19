use hyperdb_rs;

#[test]
fn test_all() {
    let mut hyper = hyperdb_rs::HyperClient::new(String::from("http://localhost:8765"));

    // Ping the server
    let mut r = hyper.ping().expect("failed");
    println!("{}", r); // PONG

    // Get the version number
    r = hyper.version().expect("failed");
    println!("{}", r); // "[HyperDB v0.2.0 (https://afaan.dev)]"

    // Set a value
    r = hyper.set("test", "value").expect("failed");
    println!("{}", r); // value

    // Check if a key is present
    r = hyper.has("test").expect("failed");
    println!("{}", r); // YES

    // Get a value
    r = hyper.get("test").expect("failed");
    println!("{}", r); // value

    // Get all stored data
    r = hyper.all().expect("failed");
    println!("{}", r); // {"test": "value"}

    // Remove a key
    r = hyper.delete("test").expect("failed");
    println!("{}", r); // OK

    // Delete all stored data
    r = hyper.clear().expect("failed");
    println!("{}", r); // OK

    // Check if the store is empty
    r = hyper.empty().expect("failed");
    println!("{}", r); // YES

    // Persist the store to disk
    r = hyper.save().expect("failed");
    println!("{}", r); // OK

    // Reload the store from disk
    r = hyper.reload().expect("failed");
    println!("{}", r); // OK

    // Delete all store data from memory and disk
    r = hyper.reset().expect("failed");
    println!("{}", r); // OK
}
