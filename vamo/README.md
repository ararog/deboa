# Bora

Bora is a rest wrapper for deboa.

## Usage

```rust
use bora::Bora;

let bora = Bora::new("https://api.example.com");
let response = bora.get("/users").await?;
```

## Features

- [x] GET
- [x] POST
- [x] PUT
- [x] DELETE
- [x] PATCH

## License

MIT

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
