//! A collection of components for rapid protocol development

#![deny(warnings, missing_docs)]

extern crate bytes;
extern crate slab;
extern crate take;
extern crate rand;
extern crate smallvec;
extern crate tokio_core;
extern crate tokio_service;

#[macro_use]
extern crate futures;

#[macro_use]
extern crate log;

pub mod client;
pub mod multiplex;
pub mod pipeline;
pub mod server;
pub mod easy;

mod body;
mod error;
mod io;
mod message;
mod sender;

pub use body::Body;
pub use client::Client;
pub use error::Error;
pub use io::{TryRead, TryWrite};
pub use message::Message;
