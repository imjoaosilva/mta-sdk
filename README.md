
# MTA_SDK

The **mta_sdk** is a Rust library designed to provide ease of calling functions in the MTA


## Installation

```toml
[dependencies]
mta_sdk = "0.1"
```

**or** 

```cargo
cargo add mta_sdk
```
    
## Usage

```rust
#[tokio::main]
async fn main() {
    
    let auth: client::Auth = client::Auth {
        username: "User".to_string(),
        password: "Password".to_string(),
    };
    let mta = client::Mta::new(String::from("127.0.0.1"), 22005, auth, true);

    let res = mta
        .call("teste", "getData", vec!["arg1".to_string(), "arg2".to_string()])
        .await;

    match res {
        Ok(res) => println!("{}", res),
        Err(err) => println!("{}", err)
    }
}
```

## Documentation

[Documentation](https://crates.io/crates/mtasa_sdk)

