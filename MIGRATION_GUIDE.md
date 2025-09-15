# Migration guide

## From 0.0.4 to 0.0.5

### Breaking changes

* Config struct was removed
* DeboaResponse allow traits to add body deserialization
* Removed builtin json support
* Added DeboaError
* Added DeboaRequest and DeboaRequestBuilder

### Non-breaking changes

* Catchers (interceptors) support
* HTTP2 support
* Responses decompression
* Introduced deboa_extras crate
* Introduced deboa_macro crate
* Introduced vamo crate

## From 0.0.3 to 0.0.4

### Breaking changes

* Added built-in json support
* Removed data and config params from requests
* Introduced DeboaResponse
* Removed anyhow support
  
### Non-breaking changes

* Added benchmarks
* Added unit tests
* Added integration tests

## From 0.0.2 to 0.0.3

### Non-breaking changes

* Added support to smol runtime

