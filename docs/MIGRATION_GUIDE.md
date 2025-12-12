---
layout: default
title: Migration Guide
nav_order: 1
permalink: /migration-guide
---

# Migration guide

## From 0.0.8 to 0.0.9

### Breaking changes

* Made request, response, connection and runtime traits sealed
* Removed deboa-bora crate
* Removed bora macro from deboa-macros crate
* Moved bora macro to vamo-macros crate

### Non-breaking changes

* Improved documentation
* Added more examples
* Deprecated Fetch trait, added FetchWith trait
* Deprecated go method, added send_with method to DeboaRequestBuilder

## From 0.0.7 to 0.0.8

### Breaking changes

* Improved error handling, added more error variants

### Non-breaking changes

* Added multipart support

## From 0.0.6 to 0.0.7

### Non-breaking changes

* Improved documentation
* Added more examples

## From 0.0.5 to 0.0.6

### Breaking changes

* Macro migration: `bora` macro moved to `deboa-bora`, update all macro imports.            

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

