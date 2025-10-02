# uget

A minimal cli tool to make http requests. You want, you get!

## Install

```sh
cargo install uget
```

## Usage

```sh
uget --url <url>
```

## Example

```sh
uget --url https://jsonplaceholder.typicode.com/posts -m POST -b "{title: 'foo', body: 'bar', userId: 1}"
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
