# Vamo

Vamo is a rest wrapper for deboa. Vamo is a key part of the deboa ecosystem, allowing bora macro to generate api clients.

## Usage

```rust
use vamo::Vamo;

let vamo = Vamo::new("https://api.example.com")?;
let response = vamo.get("/users")?.go(vamo.client()).await?;
```

## Features

- all deboa features
- set base url only once, change it when needed
- request data only by specifying path

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License

MIT

## Authors

- [Rogerio Pereira Araújo](https://github.com/ararog)
