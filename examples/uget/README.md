# uget

A minimal cli tool to make http requests. You want, you get!

## Install

```sh
cargo install uget
```

## Usage

```sh
uget <url> [OPTIONS]
```

## Example

### GET
```sh
uget https://example.com
```

### JSON (defaults to POST method)
```sh
uget https://example.com --body "{title: 'foo', body: 'bar', userId: 1}"
```

### Form (defaults to POST method)
```sh
uget https://example.com --field "title=foo" --field "body=bar" --field "userId=1"
```

### Header
```sh
uget https://example.com -m POST --header "Content-Type: application/json" --body "{ title: 'foo', body: 'bar', userId: 1 }"
```

### Bearer
```sh
uget https://example.com/users/1 -m DELETE --bearer <token>
```

### Basic
```sh
uget https://example.com -m POST --basic <username>:<password>
```

## License

MIT

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
