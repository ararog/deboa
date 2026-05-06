//! # bora - api Documentation
//!
//! Hello, and welcome to the bora API documentation!
//!
//! This API documentation is highly technical and is purely a reference.
//!
//! Depend on `bora` in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vamo-macros = { path = "../vamo-macros" }
//! ```
//!
//! <small>Note that development versions, tagged with `-dev`, are not published
//! and need to be specified as [git dependencies].</small>
//!
//! ```rust,no_run
//! use deboa::{errors::DeboaError, Result};
//! use vamo::Vamo;
//! use vamo_macros::bora;
//!
//! use serde::Deserialize;
//!
//! #[derive(Deserialize, Debug)]
//! pub struct Post {
//!     pub id: u32,
//!     pub title: String,
//! }
//!
//! #[bora(
//!     api(
//!         get(name="get_all", path="/posts", res_body=Vec<Post>, format="json"),
//!         get(name="get_by_id", path="/posts/<id:i32>", res_body=Post, format="json"),
//!         get(name="query_by_id", path="/posts?<id:i32>", res_body=Vec<Post>, format="json"),
//!         get(name="query_by_title", path="/posts?<id:i32>&<title:&str>", res_body=Vec<Post>, format="json")
//!     )
//! )]
//! pub struct PostService;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let client = Vamo::new("https://jsonplaceholder.typicode.com")?;
//!
//!     let mut post_service = PostService::new(client);
//!
//!     let post = post_service.get_by_id(1).await?;
//!
//!     println!("id...: {}", post.id);
//!     println!("title: {}", post.title);
//!
//!     assert_eq!(post.id, 1);
//!     Ok(())
//! }
//! ```
//!
//! Disabled features can be selectively enabled in `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! vamo = { version = "0.0.1" }
//! deboa-extras = { version = "0.0.1" }
//! ```
//!

pub(crate) mod config;
pub(crate) mod parser;
pub(crate) mod token;

pub use config::api::bora;
