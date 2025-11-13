# deboa-fory

Apache Fory serializer support for Deboa

## Usage

```rust
use deboa_fory::{ForyRequestBuilder, ForyResponse};
use deboa::{errors::DeboaError, request::post, Deboa};
use fory::{Fory, ForyObject};

#[derive(ForyObject)]
struct Person {
    name: String,
    age: u8,
}

let mut fory = Fory::default();
let _ = fory.register::<Person>(1);

let mut client = Deboa::new();

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

## Features

- [x] Fory serializer
- [x] Fory deserializer

## License

MIT
