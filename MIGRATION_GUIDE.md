# Migration guide

## From 0.0.8 to 0.0.9

### Breaking changes

* Made request, response, connection and runtime traits sealed

### Non-breaking changes

* Improved documentation
* Added more examples


## From 0.0.7 to 0.0.8

### Breaking changes

* 


## From 0.0.6 to 0.0.7

### Breaking changes

* 

## From 0.0.5 to 0.0.6

### Breaking changes


## From 0.0.4 to 0.0.5

### Breaking changes

* Config struct was removed
* DeboaResponse allow traits to add body deserialization
* Removed builtin json support
* Added DeboaBuilder
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

