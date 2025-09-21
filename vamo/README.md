# Vamo

Vamo is a rest wrapper for deboa.

## Usage

```rust
use vamo::Vamo;

let vamo = Vamo::new("https://api.example.com");
let response = vamo.get("/users").await?;
```

## Features

- [x] GET
- [x] POST
- [x] PUT
- [x] DELETE
- [x] PATCH

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

MIT

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
