# deboa-fory

Apache Fory serializer support for Deboa

## Features

- [x] Fory serializer
- [x] Fory deserializer

## Install

Either run from command line:

`cargo add deboa-fory`

Or add to your `Cargo.toml`:

```toml
deboa-fory = "0.0.1"
```

## Usage

```rust, ignore
use deboa::{errors::DeboaError, request::post, Deboa};
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use fory::{Fory, ForyObject};

#[derive(ForyObject)]
struct Person {
    name: String,
    age: u8,
}

let mut fory = Fory::default();
let _ = fory.register::<Person>(1);

let mut client = Deboa::default();

let person = Person {
    name: "John Doe".to_string(),
    age: 30,
};

let request = post("http://localhost:8080/persons")?
    .body_as_fory(&fory, person)?;

let response: Person = request
    .send_with(&mut client)
    .await?
    .body_as_fory(&fory)
    .await?;
```

## License

MIT
