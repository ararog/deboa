
# Vamo Macros

Vamo macros is a collection of macros to make possible
use structs as resources to be sent over vamo as client.

## Usage

```rust
use vamo_macros::Resource;
use vamo::{Vamo, ResourceMethod};

#[derive(Resource)]
#[get("/users/:id")]
#[post("/users")]
#[put("/users/:id")]
#[patch("/users/:id")]
#[delete("/users/:id")]
#[body_type(JsonBody)]
pub struct User {
    #[rid]
    id: i32,
    name: String,
}

let mut vamo = Vamo::new("https://api.example.com")?;
let response = vamo.post_resource(user).await?;
```

## Features

- derive macro for resource trait implementation

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

MIT

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
